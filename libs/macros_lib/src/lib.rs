#![allow(missing_docs)]

mod api;
mod mockable;
mod model;
mod response;
mod test_traits;
mod utils;

use api::EndpointArgs;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemImpl};
use utils::{parse_args, AttrsIdents};

#[proc_macro_derive(Response)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    response::generate(parse_macro_input!(input as DeriveInput)).into()
}

/// Generate Mockable trait implementation, `mockable_const(v)` can be written above a field to make this field always generate value `v`
///
/// # Example
/// ``` compile_fail
/// // we only need Mockable in tests
/// #[cfg_attr(test, derive(Mockable))]
/// pub struct Foo {
///     #[cfg_attr(test, mockable_const(3))]
///     pub id: i64,
///     pub name: String,
///     pub timestamp: chrono::NaiveDateTime,
///     pub geo_point: Point,
/// }
/// ```
#[proc_macro_derive(Mockable, attributes(mockable_const))]
pub fn derive_mockable(input: TokenStream) -> TokenStream {
    mockable::generate(parse_macro_input!(input as DeriveInput)).into()
}

/// Generate common variables for convenience
/// Normally one could directly use HTTP method macros (e.g., `get!`, `delete!`, ...) if possible.
///
/// # Arguments
/// * `xxx_backend` - One of the field names of `Backends`. The generated code will try to lock the backend and then establish and assign the database connection to `xxx_backend`.
/// * `offset` - The veriable `offset` must be of type `Option<i32>`, convert it to `usize` or use `DEFAULT_OFFSET` if it is `None`.
/// * `limit` - The veriable `limit` must be of type `Option<i32>`, convert it to `usize` or use `DEFAULT_LIMIT` if it is `None`.
///
/// # Example
///
/// ``` compile_fail
/// async fn get_all_xxx(
///     &self,
///     xxx_id: i64,
///     offset: Option<i32>,
///     limit: Option<i32>,
///     _context: &C,
/// ) -> Result<GetAllXxxResponse, ApiError> {
///     init!(auth_backend, offset, limit);
///     ...
/// }
/// ```
#[proc_macro]
pub fn init(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttrsIdents);
    api::gen_init(args).into()
}

/// Generate resposne that implements `Get` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `id` (optional) - Id expression , default value is `id`
///
/// # Example
///
/// ``` compile_fail
/// async fn get_one_xxx(
///     &self,
///     xxx_id: i64,
///     _context: &C,
/// ) -> Result<GetOneXxxResponse, ApiError> {
///     get!(relational_backend, id = xxx_id)
/// }
/// ```
#[proc_macro]
pub fn get(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_get(args).into()
}

/// Generate resposne that implements `GetAll` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `offset` (optional) - Offset expression , default value is `offset`. Check the document of `init!` macro for more details.
/// * `limit` (optional) - Limit expression , default value is `limit`. Check the document of `init!` macro for more details.
///
/// # Example
///
/// ``` compile_fail
/// async fn get_all_xxx(
///     &self,
///     my_offset: Option<i32>,
///     limit: Option<i32>,
///     _context: &C,
/// ) -> Result<GetAllXxxResponse, ApiError> {
///     get_all!(relational_backend, offset = my_offset)
/// }
/// ```
#[proc_macro]
pub fn get_all(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_get_all(args).into()
}

/// Generate resposne that implements `Search` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `filter` (optional) - Filter expression , default value is `filter`.
/// * `offset` (optional) - Offset expression , default value is `offset`.
/// * `limit` (optional) - Limit expression , default value is `limit`.
///
/// # Example
///
/// ``` compile_fail
/// async fn get_all_xxx(
///     &self,
///     box2d: Option<&Vec<f64>>,
///     keywords: Option<&Vec<String>>,
///     offset: Option<i32>,
///     limit: Option<i32>,
///     _context: &C,
/// ) -> Result<GetAllXxxResponse, ApiError> {
///     search!(relational_backend, filter = (keywords, box2d))
/// }
/// ```
#[proc_macro]
pub fn search(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_search(args).into()
}

/// Generate resposne that implements `Delete` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
///
/// # Example
///
/// ``` compile_fail
/// async fn delete_one_xxx(
///     &self,
///     id: i64,
///     _context: &C,
/// ) -> Result<DeleteOneXxxResponse, ApiError> {
///     delete!(relational_backend)
/// }
/// ```
#[proc_macro]
pub fn delete(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_delete(args).into()
}

/// Generate resposne that implements `DeleteAll` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `filter` (optional) - Filter expression , default value is `filter`.
///
/// # Example
///
/// ``` compile_fail
/// async fn delete_all_xxx(
///     &self,
///     user_id: i32,
///     _context: &C,
/// ) -> Result<DeleteAllXxxResponse, ApiError> {
///     delete_all!(auth_backend, filter = user_id)
/// }
/// ```
#[proc_macro]
pub fn delete_all(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_delete_all(args).into()
}

/// Generate resposne that implements `Post` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `insert` (optional) - Insert expression , default value is `insert`.
///
/// # Example
///
/// ``` compile_fail
/// async fn post_xxx(
///     &self,
///     insert: Xxx,
///     _context: &C,
/// ) -> Result<PostXxxResponse, ApiError> {
///     post!(relational_backend)
/// }
/// ```
#[proc_macro]
pub fn post(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_post(args).into()
}

/// Generate resposne that implements `Upsert` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `Upsert` (optional) - upsert expression , default value is `upsert`.
///
/// # Example
///
/// ``` compile_fail
/// async fn post_xxx(
///     &self,
///     upsert: Xxx,
///     _context: &C,
/// ) -> Result<PostXxxResponse, ApiError> {
///     upsert!(relational_backend)
/// }
/// ```
#[proc_macro]
pub fn upsert(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_upsert(args).into()
}

/// Generate resposne that implements `Patch` trait.
///
/// # Arguments
/// * `backend` - One of the field names of `Backends`. Check the document of `init!` macro for more details.
/// * `id` (optional) - Id expression , default value is `id`.
/// * `patch` (optional) - Patch expression , default value is `patch`.
///
/// # Example
///
/// ``` compile_fail
/// async fn patch_one_xxx(
///     &self,
///     id: i64,
///     patch: Xxx,
///     _context: &C,
/// ) -> Result<PatchOneXxxResponse, ApiError> {
///     patch!(relational_backend)
/// }
///
/// ```
#[proc_macro]
pub fn patch(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EndpointArgs);
    api::gen_patch(args).into()
}

#[proc_macro_derive(Model)]
pub fn derive_models(input: TokenStream) -> TokenStream {
    model::generate(parse_macro_input!(input as DeriveInput)).into()
}

/// Generate read tests based on the below `Readable` trait implementation.
///
/// # Arguments
/// * `testable_ids` - testable IDs of type `DbEntry::ID` in the database. It can accept either a single number, a number range, or a list containing single numbers or number ranges.
///
/// # Example
///
/// ``` compile_fail
/// #[gen_read_tests(testable_ids = [3, 35..=74, 76, 77, 87, 167])]
/// impl<Conn: LoadConnection<Backend = Pg>> Readable<Conn> for XxxPersistent {
///     fn read_one(c: &mut Conn, id: Self::ID) -> Result<Option<Self>, DatabaseError> {
///         // implementation of read_one operation
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn gen_read_tests(args_tk: TokenStream, item: TokenStream) -> TokenStream {
    match parse_args(args_tk) {
        Ok(args) => test_traits::read_tests(args, parse_macro_input!(item as ItemImpl)).into(),
        Err(tk) => tk,
    }
}

/// Generate batch read tests based on the below `BatchReadable` or `Searchable` trait implementation. If it is written above `Searchable` trait, this marco will first implement `BatchReadable` based on `Searchable` with default filter.
///
/// # Arguments
/// * `limit` - The limit that will be passed to method `BatchReadable::read_all`.
/// * `offset` (optional) - The offset that will be passed to method `BatchReadable::read_all`, default value is 0.
///
/// # Example
///
/// ``` compile_fail
/// #[gen_batch_read_tests(limit = 45)]
/// impl<Conn: LoadConnection<Backend = Pg>> Searchable<Conn> for XxxPersistent {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn gen_batch_read_tests(args: TokenStream, item: TokenStream) -> TokenStream {
    match parse_args(args) {
        Ok(args) => {
            test_traits::batch_read_tests(args, parse_macro_input!(item as ItemImpl)).into()
        }
        Err(tk) => tk,
    }
}

/// Generate create tests based on the below `Creatable` trait implementation.
///
/// # Example
///
/// ``` compile_fail
/// #[gen_create_tests]
/// impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, NestedField> for XxxInsert {
/// ...
/// }
/// ```
#[proc_macro_attribute]
pub fn gen_create_tests(_args: TokenStream, item: TokenStream) -> TokenStream {
    test_traits::create_tests(parse_macro_input!(item as ItemImpl)).into()
}

/// Generate delete tests based on the below `Deletable` trait implementation.
///
/// # Example
///
/// ``` compile_fail
/// #[gen_delete_tests]
/// impl<Conn: LoadConnection<Backend = Pg>> Deletable<Conn> for XxxPersistent {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn gen_delete_tests(_args: TokenStream, item: TokenStream) -> TokenStream {
    test_traits::delete_tests(parse_macro_input!(item as ItemImpl)).into()
}

/// Generate update tests based on the below `Updatable` trait implementation.
///
/// # Example
///
/// ``` compile_fail
/// #[gen_update_tests]
/// impl<Conn: LoadConnection<Backend = Pg>> Updatable<Conn, FieldPersistent> for FieldChangeSet {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn gen_update_tests(args: TokenStream, item: TokenStream) -> TokenStream {
    test_traits::update_tests(args.into(), parse_macro_input!(item as ItemImpl)).into()
}
