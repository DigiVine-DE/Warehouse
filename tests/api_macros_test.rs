use async_trait::async_trait;
use macros_lib::init;
use std::convert::TryFrom;
use swagger::ApiError;

const DEFAULT_OFFSET: i32 = 0;
const DEFAULT_LIMIT: i32 = 100;

#[async_trait]
pub trait Api {
    async fn get_offset_and_limit(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<(usize, usize), ApiError>;
}

#[derive(Clone)]
pub struct Server;

#[async_trait]
impl Api for Server {
    async fn get_offset_and_limit(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<(usize, usize), ApiError> {
        init!(offset, limit);
        Ok((offset, limit))
    }
}

#[tokio::test]
async fn async_works() {
    let s = Server;
    let (offset, limit) = s.get_offset_and_limit(Some(1), Some(2)).await.unwrap();
    assert_eq!(offset, 1);
    assert_eq!(limit, 2);

    let (offset, limit) = s.get_offset_and_limit(None, None).await.unwrap();
    assert_eq!(offset, usize::try_from(DEFAULT_OFFSET).unwrap());
    assert_eq!(limit, usize::try_from(DEFAULT_LIMIT).unwrap());

    let error = s.get_offset_and_limit(Some(-99), Some(-99)).await;
    assert!(error.is_err())
}
