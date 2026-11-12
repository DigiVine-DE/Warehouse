use crate::response::{Delete, DeleteAll, FromEntry, FromJson, Get, Post, Search, Upsert};
use auth_backend::{
    db::{AccessRightsPersistent, TokenInsert, TokenPersistent},
    TokenFilter,
};
use digivine_server::{
    models, DeleteOneAccessRightResponse, DeleteOneTokenResponse, DeleteOneUserResponse,
    GetAccessRightsResponse, GetAllTokensResponse, GetOneAccessRightResponse,
    GetUserTokensResponse, PostAccessRightResponse, PostTokenResponse, PostUserTokenResponse,
};

impl Search<TokenPersistent, models::Token> for GetAllTokensResponse {}
impl Search<TokenPersistent, models::Token> for GetUserTokensResponse {}
impl Delete<TokenPersistent> for DeleteOneTokenResponse {}
impl DeleteAll<TokenPersistent, i32, models::Token> for DeleteOneUserResponse {}
impl Post<TokenInsert, models::Token> for PostTokenResponse {}
impl Post<TokenInsert, (i32, String)> for PostUserTokenResponse {}

impl Delete<AccessRightsPersistent> for DeleteOneAccessRightResponse {}
impl Search<AccessRightsPersistent, models::AccessRight> for GetAccessRightsResponse {}
impl Get<AccessRightsPersistent, models::AccessRight> for GetOneAccessRightResponse {}
impl
    Upsert<
        AccessRightsPersistent,
        AccessRightsPersistent,
        (i32, models::AccessRight),
        models::AccessRight,
    > for PostAccessRightResponse
{
}

impl FromEntry<TokenPersistent> for models::Token {
    fn from_entry(tk: TokenPersistent) -> Result<Self, String> {
        Ok(Self {
            tokenstr: tk.tokenstr,
            user_id: tk.user_id,
        })
    }
}

impl FromEntry<AccessRightsPersistent> for models::AccessRight {
    fn from_entry(ar: AccessRightsPersistent) -> Result<Self, String> {
        Ok(Self {
            path: ar.path,
            read_only: ar.readonly,
        })
    }
}

impl FromJson<models::Token> for TokenInsert {
    fn from_json(models::Token { tokenstr, user_id }: models::Token) -> Result<Self, String> {
        Ok(TokenInsert { tokenstr, user_id })
    }
}

impl FromJson<(i32, String)> for TokenInsert {
    fn from_json((user_id, tokenstr): (i32, String)) -> Result<Self, String> {
        Ok(TokenInsert { tokenstr, user_id })
    }
}

impl FromJson<(i32, models::AccessRight)> for AccessRightsPersistent {
    fn from_json(
        (tid, models::AccessRight { path, read_only }): (i32, models::AccessRight),
    ) -> Result<Self, String> {
        Ok(AccessRightsPersistent {
            tid,
            path,
            readonly: read_only,
        })
    }
}

impl<'a> FromJson<Option<&'a Vec<String>>> for TokenFilter<'a> {
    fn from_json(f: Option<&'a Vec<String>>) -> Result<Self, String> {
        Ok(TokenFilter {
            tokenstr_filter: f.into(),
            user_id: None,
        })
    }
}

impl<'a> FromJson<(i32, Option<&'a Vec<String>>)> for TokenFilter<'a> {
    fn from_json((id, f): (i32, Option<&'a Vec<String>>)) -> Result<Self, String> {
        Ok(TokenFilter {
            tokenstr_filter: f.into(),
            user_id: Some(id),
        })
    }
}
