use crate::server::{api_addr, array_params, post_empty_coordinates_test};
use diesel::PgConnection;
use diesel_ext::{
    db_backend::{Backend, DbConnector, Searchable},
    db_tester::ReadTestable,
};
use digivine_api::response::{self, FromEntry, FromJson};
use digivine_api_postgres::{field::FieldFilter, harvest::harvest_complete::NestedHarvestComplete};
use digivine_server::{
    models, AsBackendErrorResp, AsBadRequestResponse, GetAllFieldsResponse,
    GetAllScheduledHarvestTasksResponse, GetAllTokensResponse, GetOneScheduledHarvestTaskResponse,
    GetUserTokensResponse, PostCompletedHarvestTaskResponse,
};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use reqwest::blocking::{Client, RequestBuilder};
use serde_json::json;
use std::any::type_name;

pub trait AsHttpParam<Resp>: Sized {
    fn as_http_param(&self, client: &Client) -> RequestBuilder;
}

pub trait AsApiParam<F>: Sized {
    fn as_api_param(&self) -> F;
}

pub trait ServerClientTestableResp
where
    Self: std::fmt::Debug + PartialEq,
{
    fn server_client_test<'a, Entry, Json, Param, FilterJson>(test_name: &str, p: Param)
    where
        Self: response::Search<Entry, Json> + FromIterator<Json>,
        String: AsBackendErrorResp<Self> + AsBadRequestResponse<Self>,
        Param: AsHttpParam<Self> + AsApiParam<FilterJson>,
        FilterJson: 'a,
        Json: FromEntry<Entry>,
        for<'de> Json: serde::de::Deserialize<'de>,
        Entry: 'a + Searchable<PgConnection>,
        Entry::Filter<'a>: FromJson<FilterJson>,
        <Entry::B as Backend>::Error: AsBackendErrorResp<Self>,
    {
        let client = reqwest::blocking::Client::new();
        let from_http: Vec<Json> = p
            .as_http_param(&client)
            .send()
            .expect("could not send request")
            .json()
            .unwrap_or_else(|err| {
                panic!(
                    "could not deserialize resp as Vec<{}>: {err}",
                    type_name::<Json>(),
                )
            });
        let resp: Self = from_http.into_iter().collect();

        let mut conn = Entry::B::db_tester().test_connection();
        let from_api = Self::search(&mut conn, p.as_api_param(), 0, 100);
        assert_eq!(
            resp, from_api,
            "[{test_name}] responses got via http and internal api are different"
        );
    }
}

#[derive(Debug, Default)]
pub struct FieldParams<'a> {
    pub bbox: Option<&'a Vec<f64>>,
    pub filter: FieldFilter<'a>,
}

impl<'a> AsHttpParam<GetAllFieldsResponse> for FieldParams<'a> {
    fn as_http_param(&self, client: &Client) -> RequestBuilder {
        let mut req = client
            .get(format!("{}/field", api_addr()))
            .query(&[
                ("api-token", "all_readonly"),
                ("offset", "0"),
                ("limit", "100"),
                ("Accept", "application/json"),
            ])
            .query(&array_params(
                "fuzzyFieldKeywords",
                self.filter.fuzzy_keywords.raw(),
            ))
            .query(&array_params(
                "fieldNameContains",
                self.filter.field_name_filter.raw(),
            ))
            .query(&array_params(
                "fruitNameContains",
                self.filter.fruit_name_filter.raw(),
            ));
        if let Some(bbox) = &self.bbox {
            req = req.query(&array_params("box2d", bbox));
        }
        req
    }
}

impl<'a> AsApiParam<(FieldFilter<'a>, Option<&'a Vec<f64>>)> for FieldParams<'a> {
    fn as_api_param(&self) -> (FieldFilter<'a>, Option<&'a Vec<f64>>) {
        (self.filter.clone(), self.bbox)
    }
}

#[derive(Debug, Default)]
pub struct TokenParams<'a> {
    pub user_id: Option<i32>,
    pub tokenstr_contains: Option<&'a Vec<String>>,
}

impl<'a> AsHttpParam<GetAllTokensResponse> for TokenParams<'a> {
    fn as_http_param(&self, client: &Client) -> RequestBuilder {
        let mut req = client.get(format!("{}/token", api_addr())).query(&[
            ("api-token", "all_readonly"),
            ("Accept", "application/json"),
        ]);
        if let Some(f) = self.tokenstr_contains {
            req = req.query(&array_params("tokenstrContains", f));
        }
        req
    }
}

impl<'a> AsHttpParam<GetUserTokensResponse> for TokenParams<'a> {
    fn as_http_param(&self, client: &Client) -> RequestBuilder {
        let mut req = client
            .get(format!(
                "{}/token/{}",
                api_addr(),
                self.user_id.expect("user id is none")
            ))
            .query(&[
                ("api-token", "all_readonly"),
                ("Accept", "application/json"),
            ]);
        if let Some(f) = self.tokenstr_contains {
            req = req.query(&array_params("tokenstrContains", f));
        }
        req
    }
}

impl<'a> AsApiParam<Option<&'a Vec<String>>> for TokenParams<'a> {
    fn as_api_param(&self) -> Option<&'a Vec<String>> {
        self.tokenstr_contains
    }
}
