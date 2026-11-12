table! {
    accessrights (tid, path) {
        tid -> Int4,
        path -> Varchar,
        readonly -> Bool,
    }
}

table! {
    token (id) {
        id -> Int4,
        tokenstr -> Varchar,
        user_id -> Int4,
    }
}

joinable!(accessrights -> token (tid));

allow_tables_to_appear_in_same_query!(accessrights, token,);
