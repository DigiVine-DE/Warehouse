use crate::schema;
use diesel::{pg::Pg, prelude::*};
use diesel_ext::Keywords;

#[derive(Default, Debug, Clone)]
pub struct TokenFilter<'a> {
    pub tokenstr_filter: Keywords<'a>,
    pub user_id: Option<i32>,
}

impl<'a> TokenFilter<'a> {
    pub fn to_query(&self) -> schema::token::BoxedQuery<'_, Pg> {
        use schema::token::dsl;
        let mut query = dsl::token.into_boxed();
        query = self
            .tokenstr_filter
            .like_pattern()
            .iter()
            .fold(query, |q, seg| q.filter(dsl::tokenstr.ilike(seg)));
        if let Some(user_id) = self.user_id {
            query = query.filter(dsl::user_id.eq(user_id));
        }
        query
    }
}
