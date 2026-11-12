pub mod token_api;

use auth_backend::db::has_access;
use core::{
    marker::PhantomData,
    task::{Context, Poll},
};
use diesel_ext::db_backend::DbConnector;
use digivine_server::server::Authorization;
use futures::FutureExt;
use hyper::{service::Service, Request};
use std::sync::{Arc, Mutex};
use swagger::{auth::Scopes, AuthData, Has, Push};

pub trait RcBound: Has<Option<AuthData>> + Push<Option<Authorization>> + Send + 'static {}

impl<T> RcBound for T where T: Has<Option<AuthData>> + Push<Option<Authorization>> + Send + 'static {}

pub struct MakeAllowAPIAuthenticator<T, RC, Conn>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Conn: DbConnector,
{
    inner: T,
    backend: Arc<Mutex<Conn>>,
    marker: PhantomData<RC>,
}

impl<T, RC, Conn> MakeAllowAPIAuthenticator<T, RC, Conn>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Conn: DbConnector,
{
    /// Create a middleware that authorizes with the configured subject.
    pub fn new(inner: T, backend: Arc<Mutex<Conn>>) -> Self {
        MakeAllowAPIAuthenticator {
            inner,
            backend,
            marker: PhantomData,
        }
    }
}

impl<Inner, RC, Target, Conn> Service<Target> for MakeAllowAPIAuthenticator<Inner, RC, Conn>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Inner: Service<Target>,
    Inner::Future: Send + 'static,
    Conn: DbConnector,
{
    type Error = Inner::Error;
    type Response = AllowAPIAuthenticator<Inner::Response, RC, Conn>;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, target: Target) -> Self::Future {
        let back = self.backend.clone();
        Box::pin(
            self.inner
                .call(target)
                .map(|s| Ok(AllowAPIAuthenticator::new(s?, back))),
        )
    }
}

#[derive(Clone)]
pub struct AllowAPIAuthenticator<T, RC, Conn>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Conn: DbConnector,
{
    inner: T,
    marker: PhantomData<RC>,
    backend: Arc<Mutex<Conn>>,
}

impl<T, RC, Conn> AllowAPIAuthenticator<T, RC, Conn>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Conn: DbConnector,
{
    /// Create a middleware that authorizes with the configured subject.
    pub fn new(inner: T, backend: Arc<Mutex<Conn>>) -> Self {
        AllowAPIAuthenticator {
            inner,
            marker: PhantomData,
            backend,
        }
    }
}

impl<T, B, RC, Conn> Service<(Request<B>, RC)> for AllowAPIAuthenticator<T, RC, Conn>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    T: Service<(Request<B>, RC::Result)>,
    Conn: DbConnector,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<B>, RC)) -> Self::Future {
        let (request, context) = req;
        macro_rules! failed {
            () => {
                self.inner
                    .call((request, context.push(None::<Authorization>)))
            };
        }
        macro_rules! unwrap {
            ($e:expr, $msg:expr) => {
                match $e {
                    Ok(r) => r,
                    Err(err) => {
                        error!($msg, err);
                        return failed!();
                    }
                }
            };
        }

        match context.get() {
            Some(AuthData::ApiKey(key)) => {
                let locked = unwrap!(self.backend.lock(), "Failed to lock auth backend: {}");
                let mut conn = unwrap!(
                    locked.establish_connection(),
                    "Failed to get db connection from auth backend: {}"
                );
                let read = request.method().is_safe();
                let access = unwrap!(
                    has_access(&mut conn, key, request.uri().path(), read),
                    "Failed to get authorization from auth backend: {}"
                );
                if access {
                    let context = context.push(Some(Authorization {
                        subject: "API".into(),
                        scopes: Scopes::All,
                        issuer: None,
                    }));
                    self.inner.call((request, context))
                } else {
                    //TODO 403?
                    failed!()
                }
            }
            _ => {
                failed!()
            }
        }
    }
}
