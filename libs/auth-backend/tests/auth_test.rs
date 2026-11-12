use auth_backend::{
    db::{find_token_with_user, has_access, AccessRightsPersistent, TokenInsert, TokenPersistent},
    Auth, TokenFilter,
};
use diesel::PgConnection;
use diesel_ext::db_backend::{
    Backend, BatchDeletable, Creatable, Deletable, Readable, Searchable, Upsertable,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

const HIT: bool = true;
const MISS: bool = !HIT;

const READ: bool = true;
const WRITE: bool = !READ;

/*
Assume auth db has the following entries
|----+--------------+---------+--------------+----------|
| id | tokenstr     | user_id | accessrights | readonly |
|----+--------------+---------+--------------+----------|
|  1 | all          |       5 | *            | false    |
|  2 | all_readonly |       5 | *            | true     |
|  3 | wagner_read  |       5 | /wagner%     | true     |
|  4 | none         |       5 |              |          |
|----+--------------+---------+--------------+----------|
*/

#[test]
fn has_access_right_test() {
    let mut c = TokenChecker::default();
    c.has_access_right_test("wagner_read", "/wagner/any", READ, HIT);
    c.has_access_right_test("wagner_read", "/wagner/any", WRITE, MISS);
    c.has_access_right_test("wagner_read", "/wagner", READ, HIT);
    c.has_access_right_test("wagner_read", "/wagnea", READ, MISS);
    c.has_access_right_test("all_readonly", "/token", READ, HIT);
    c.has_access_right_test("all_readonly", "/wagner", WRITE, MISS);
    c.has_access_right_test("none", "/wagner", READ, MISS);
}

#[test]
fn search_token_test() {
    let mut c = TokenChecker::default();
    c.search_test(&[], None, &["all", "all_readonly", "wagner_read", "none"]);
    c.search_test(&[], Some(1), &[]);
    c.search_test(&["all"], None, &["all", "all_readonly"]);
    c.search_test(&["all", "read"], None, &["all_readonly"]);
    c.search_test(&["all"], Some(5), &["all", "all_readonly"]);
    c.search_test(&["all"], Some(6), &[]);
}

#[test]
fn find_token_test() {
    let mut c = TokenChecker::default();
    c.find_token_test(5, "all", HIT);
    c.find_token_test(2, "all", MISS);
    c.find_token_test(5, "all_readonly", HIT);
}

#[test]
fn add_remove_token_test() {
    let mut c = TokenChecker::default();
    let uid = 5;
    c.add_token("test", uid);
    let tk = c.find_token_test(uid, "test", HIT).unwrap();
    assert!(c.remove_token(tk.id));
    assert!(!c.remove_token(tk.id), "tokenstr test was already deleted")
}

#[test]
fn remove_user_test() {
    let mut c = TokenChecker::default();
    let user_id = 99;
    c.add_token("1", user_id);
    c.add_token("2", user_id);

    assert_eq!(c.remove_user(user_id).len(), 2);
    assert!(
        c.remove_user(user_id).is_empty(),
        "user id {} was already deleted",
        user_id
    )
}

#[test]
fn get_access_rights_test() {
    AccessRightChecker::with_tid(1).assert_eq(vec![("*", WRITE)]);

    let mut wagner = AccessRightChecker::with_tid(3);
    assert!(wagner.get("*").is_none());
    assert!(wagner.get("").is_none());
    wagner.assert_eq(vec![("/wagner%", READ)]);
}

#[test]
fn modify_access_rights_test() {
    let tid = 4;
    let mut ac = AccessRightChecker::with_tid(tid);
    assert!(ac.get_all().is_empty());

    ac.upsert("/test1", READ);
    ac.assert_eq(vec![("/test1", READ)]);

    // insert new right
    ac.upsert("/test2", WRITE);
    ac.assert_eq(vec![("/test1", READ), ("/test2", WRITE)]);

    // update readonly of the same tid
    ac.upsert("/test1", WRITE);
    ac.assert_eq(vec![("/test1", WRITE), ("/test2", WRITE)]);

    assert!(ac.remove_access_right("/test2"));
    ac.assert_eq(vec![("/test1", WRITE)]);
}

struct TokenChecker {
    conn: PgConnection,
}

struct AccessRightChecker {
    tc: TokenChecker,
    tid: i32,
}

impl AccessRightChecker {
    fn with_tid(tid: i32) -> Self {
        Self {
            tc: TokenChecker::default(),
            tid,
        }
    }

    fn upsert(&mut self, tokenstr: &str, readonly: bool) {
        let tid = self.tid;
        let ar = AccessRightsPersistent {
            tid,
            path: tokenstr.to_string(),
            readonly,
        };

        ar.upsert(&mut self.tc.conn).unwrap_or_else(|err| {
            panic!(
                "Error while upserting access right with tid {}: {}",
                &self.tid, err
            )
        });
    }

    fn get_all(&mut self) -> Vec<AccessRightsPersistent> {
        AccessRightsPersistent::search(&mut self.tc.conn, Some(self.tid), 0, 4).unwrap_or_else(
            |err| {
                panic!(
                    "Error while getting access rights with tid {}: {}",
                    &self.tid, err
                )
            },
        )
    }

    fn get(&mut self, path: &str) -> Option<AccessRightsPersistent> {
        AccessRightsPersistent::read_one(&mut self.tc.conn, (self.tid, path.to_string()))
            .expect("Error while getting access right")
    }

    fn remove_access_right(&mut self, path: &str) -> bool {
        let res = AccessRightsPersistent::delete(&mut self.tc.conn, (self.tid, path.to_string()))
            .unwrap_or_else(|err| {
                panic!(
                    "Error while removing access right with tid {}: {}",
                    &self.tid, err
                )
            });
        match res {
            Some((removed_id, removed_path)) => {
                assert_eq!(removed_id, self.tid);
                assert_eq!(removed_path, path);
                true
            }
            None => false,
        }
    }

    fn assert_eq(&mut self, want: Vec<(&str, bool)>) {
        // testing get_access_right
        for (p, readonly) in &want {
            assert_eq!(
                self.get(p),
                Some(AccessRightsPersistent {
                    tid: self.tid,
                    path: p.to_string(),
                    readonly: *readonly
                })
            );
        }

        // testing get_access_rights
        // using hash map to ignore order
        let want: HashMap<_, _> = want.iter().map(|(p, r)| (p.to_string(), *r)).collect();
        let got = self.get_all();
        let got: HashMap<_, _> = got.into_iter().map(|ar| (ar.path, ar.readonly)).collect();
        assert_eq!(got, want)
    }
}

impl TokenChecker {
    fn find_token_test(
        &mut self,
        user_id: i32,
        tokenstr: &str,
        exists: bool,
    ) -> Option<TokenPersistent> {
        let got = find_token_with_user(&mut self.conn, user_id, tokenstr).unwrap();
        if exists {
            let got = got.clone().unwrap_or_else(|| {
                panic!("could not find token with user_id:{user_id} and tokenstr:{tokenstr}",)
            });
            assert_eq!(got.user_id, user_id);
            assert_eq!(got.tokenstr, tokenstr);
        } else {
            assert!(got.is_none(), "wrong token was found")
        }
        got
    }

    fn has_access_right_test(&mut self, tokenstr: &str, path: &str, read: bool, want: bool) {
        let got = has_access(&mut self.conn, tokenstr, path, read).unwrap();
        assert_eq!(
            got, want,
            "token:{tokenstr}, path:{path}, want:{want}, got:{got}"
        )
    }

    fn search_test(&mut self, keywords: &[&str], user_id: Option<i32>, expect_tokens: &[&str]) {
        let f = TokenFilter {
            tokenstr_filter: keywords.iter().copied().collect(),
            user_id,
        };
        let res = TokenPersistent::search(&mut self.conn, f.clone(), 0, 100)
            .unwrap_or_else(|err| panic!("could not search tokens with filter {:?}: {err}", &f));
        let tokens: Vec<_> = res.iter().map(|t| &t.tokenstr).collect();
        assert_eq!(tokens, expect_tokens)
    }

    fn add_token(&mut self, tokenstr: &str, user_id: i32) {
        let insert = TokenInsert {
            tokenstr: tokenstr.to_string(),
            user_id,
        };
        insert.create(&mut self.conn).unwrap_or_else(|err| {
            panic!(
                "could not add tokenstr [{tokenstr}] with user id {user_id}: {}",
                err
            )
        });
    }

    fn remove_token(&mut self, tid: i32) -> bool {
        let removed_id = TokenPersistent::delete(&mut self.conn, tid)
            .unwrap_or_else(|err| panic!("could not remove token with token id {tid}: {err}"));
        match removed_id {
            Some(removed_id) => {
                assert_eq!(
                    removed_id, tid,
                    "removed id {removed_id} doesn't match the id want to remove {tid}"
                );
                true
            }
            None => false,
        }
    }

    fn remove_user(&mut self, user_id: i32) -> Vec<TokenPersistent> {
        let tokens = TokenPersistent::batch_delete(&mut self.conn, user_id)
            .unwrap_or_else(|err| panic!("could not remove tokens with user id {user_id}: {err}"));
        for tk in &tokens {
            assert_eq!(tk.user_id, user_id);
        }
        tokens
    }
}

impl Deref for AccessRightChecker {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.tc.conn
    }
}

impl DerefMut for AccessRightChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tc.conn
    }
}

impl Default for TokenChecker {
    fn default() -> Self {
        Self {
            conn: Auth::db_tester().test_connection(),
        }
    }
}

impl Deref for TokenChecker {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for TokenChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}
