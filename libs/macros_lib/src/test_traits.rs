use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, Expr, ExprRange, Ident, ItemImpl, LitInt};

pub fn read_tests(args: ReadTestArgs, item: ItemImpl) -> TokenStream {
    assert_eq!(
        extract_trait_name(&item),
        "Readable",
        "gen_read_tests can only be used above Readable impl block"
    );
    let entry = extract_implementor(&item);
    let entry_str = entry.to_string().to_lowercase();
    let test_mod = format_ident!("{}_read_tests", entry_str);
    let test_fun = format_ident!("{}_read_test", entry_str);
    let testable_ids = args.testable_ids.to_tokens();
    let id = args.id.unwrap_or_else(|| parse_str("self.id").unwrap());
    quote! {
        #item

        #[cfg(test)]
        pub mod #test_mod {
            use super::{#entry};
            use diesel_ext::db_tester::ReadTestable;
            use serial_test::serial;

            impl ReadTestable for #entry {
                const NAME: &'static str = #entry_str;

                fn testable_ids() -> Vec<Self::ID> {
                    #testable_ids
                }

                fn missing_id() -> Self::ID {
                    Self::testable_ids().into_iter().max().expect("no testable ids") + 1
                }

                fn id(&self) -> Self::ID {
                    #id
                }
            }

            #[test]
            #[serial]
            fn #test_fun() {
                #entry::read_test()
            }
        }
    }
}

pub fn update_tests(_: TokenStream, item: ItemImpl) -> TokenStream {
    assert_eq!(
        extract_trait_name(&item),
        "Updatable",
        "gen_create_tests can only be used above Creatable impl block"
    );
    let update = extract_implementor(&item);
    let update_str = update.to_string().to_lowercase();
    let entry = extract_entry_type(&item, &update);
    let test_mod = format_ident!("{}_update_tests", update_str);
    let test_fun = format_ident!("{}_update_test", update_str);
    quote! {
        #item

        #[cfg(test)]
        pub mod #test_mod {
            use diesel_ext::db_tester::UpdateTestable;
            use serial_test::serial;

            impl UpdateTestable<super::#entry> for super::#update {}

            #[test]
            #[serial]
            fn #test_fun() {
                super::#update::update_test()
            }
        }
    }
}

pub fn create_tests(item: ItemImpl) -> TokenStream {
    assert_eq!(
        extract_trait_name(&item),
        "Creatable",
        "gen_create_tests can only be used above Creatable impl block"
    );
    let insert = extract_implementor(&item);
    let insert_str = insert.to_string().to_lowercase();
    let entry = extract_entry_type(&item, &insert);
    let test_mod = format_ident!("{}_create_tests", insert_str);
    let test_fun = format_ident!("{}_create_test", insert_str);
    quote! {
        #item

        #[cfg(test)]
        pub mod #test_mod {
            use diesel_ext::db_tester::CreateTestable;
            use serial_test::serial;

            impl CreateTestable<super::#entry> for super::#insert {}

            #[test]
            #[serial]
            fn #test_fun() {
                super::#insert::create_test()
            }
        }
    }
}

pub fn delete_tests(item: ItemImpl) -> TokenStream {
    assert_eq!(
        extract_trait_name(&item),
        "Deletable",
        "gen_delete_tests can only be used above Deletable impl block"
    );
    let entry = extract_implementor(&item);
    let entry_str = entry.to_string().to_lowercase();
    let test_mod = format_ident!("{}_delete_tests", entry_str);
    let test_fun = format_ident!("{}_delete_test", entry_str);
    quote! {
        #item

        #[cfg(test)]
        pub mod #test_mod {
            use super::{#entry};
            use diesel_ext::db_tester::DeleteTestable;
            use serial_test::serial;

            impl DeleteTestable for #entry {}

            #[test]
            #[serial]
            fn #test_fun() {
                #entry::delete_test()
            }
        }
    }
}

pub fn batch_read_tests(args: BatchReadTestArgs, item: ItemImpl) -> TokenStream {
    let trait_name = extract_trait_name(&item);
    assert!(
        trait_name == "BatchReadable" || trait_name == "Searchable",
        "gen_batch_read_tests can only be used above BatchReadable or Searchable impl block"
    );
    let entry = extract_implementor(&item);
    let entry_str = entry.to_string().to_lowercase();
    let test_mod = format_ident!("{}_read_batch_tests", entry_str);
    let test_fun = format_ident!("{}_read_batch_test", entry_str);
    let offset = args.offset;
    let limit = args.limit;
    // implement batch_readable for searchable entries to test batch read
    let impl_batch_readable = if trait_name == "Searchable" {
        quote! {
           impl diesel_ext::db_backend::BatchReadable<diesel::PgConnection> for #entry {
               fn read_all(
                   c: &mut diesel::PgConnection,
                   offset: usize,
                   count: usize,
               ) -> Result<Vec<Self>, diesel_ext::db_backend::DatabaseError> {
                   diesel_ext::db_backend::Searchable::search(c, Default::default(), offset, count)
               }
           }
        }
    } else {
        quote! {}
    };
    quote! {
        #item

        #[cfg(test)]
        pub mod #test_mod {
            use super::{#entry};
            use diesel_ext::db_tester::BatchReadTestable;
            use serial_test::serial;

            #impl_batch_readable

            impl BatchReadTestable for #entry {
                const OFFSET: usize = #offset;
                const LIMIT: usize = #limit;
            }

            #[test]
            #[serial]
            fn #test_fun() {
                #entry::batch_read_test()
            }
        }
    }
}

#[derive(FromMeta, Debug)]
pub struct BatchReadTestArgs {
    #[darling(default)]
    offset: usize,
    limit: usize,
}

#[derive(FromMeta, Debug)]
pub struct ReadTestArgs {
    #[darling(default)]
    id: Option<Expr>,
    testable_ids: IdRanges,
}

#[derive(Debug)]
pub struct IdRanges(Vec<IdRange>);

impl IdRanges {
    fn to_tokens(&self) -> TokenStream {
        let tokens: Vec<_> = self.0.iter().map(IdRange::to_tokens).collect();
        quote! {{
            let mut res = vec![];
            #(res.extend(#tokens);)*
            res
        }}
    }
}

impl FromMeta for IdRanges {
    fn from_expr(expr: &Expr) -> darling::Result<Self> {
        Ok(IdRanges(match *expr {
            Expr::Array(ref arr) => arr
                .elems
                .iter()
                .map(IdRange::from_expr)
                .collect::<darling::Result<_>>()?,
            _ => vec![IdRange::from_expr(expr)?],
        }))
    }
}

#[derive(Debug)]
pub enum IdRange {
    Single(LitInt),
    Range(ExprRange),
}

impl IdRange {
    fn to_tokens(&self) -> TokenStream {
        match self {
            IdRange::Single(int) => quote!(#int..=#int),
            IdRange::Range(r) => quote!(#r),
        }
    }
}

impl FromMeta for IdRange {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        (match *value {
            syn::Lit::Int(ref int) => Ok(IdRange::Single(int.clone())),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        })
        .map_err(|e| e.with_span(value))
    }

    fn from_expr(expr: &Expr) -> darling::Result<Self> {
        match *expr {
            Expr::Lit(ref lit) => Self::from_value(&lit.lit),
            Expr::Group(ref group) => Self::from_expr(&group.expr),
            Expr::Range(ref r) => Ok(IdRange::Range(r.clone())),
            _ => Err(darling::Error::unexpected_expr_type(expr)),
        }
        .map_err(|e| e.with_span(expr))
    }
}

/// get `Creatable` from e.g. `impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, PlantPersistent> for PlantInsert {...}`
fn extract_trait_name(item: &ItemImpl) -> String {
    item.trait_
        .as_ref()
        .expect("the trait name can not be found")
        .1
        .segments
        .first()
        .unwrap()
        .ident
        .to_string()
}

/// get `PlantInsert` from e.g. `impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, PlantPersistent> for PlantInsert {...}`
fn extract_implementor(item: &ItemImpl) -> Ident {
    match item.self_ty.as_ref() {
        syn::Type::Path(tp) => tp.path.get_ident().unwrap().clone(),
        _ => panic!("could not get implementor"),
    }
}

/// get `PlantPersistent` from e.g. `impl<Conn: LoadConnection<Backend = Pg>> Creatable<Conn, PlantPersistent> for PlantInsert {...}`
fn extract_entry_type(item: &ItemImpl, implementor: &Ident) -> Ident {
    let generic_arg = match &item
        .trait_
        .as_ref()
        .expect("the trait name can not be found")
        .1
        .segments
        .first()
        .unwrap()
        .arguments
    {
        syn::PathArguments::AngleBracketed(a) => a.args.last().unwrap(),
        _ => panic!("could not get generic arg"),
    };
    let ident = match generic_arg {
        syn::GenericArgument::Type(syn::Type::Path(tp)) => tp.path.get_ident().unwrap(),
        _ => panic!("could not get the type of generic arg"),
    };

    if *ident == "Self" { implementor } else { ident }.clone()
}
