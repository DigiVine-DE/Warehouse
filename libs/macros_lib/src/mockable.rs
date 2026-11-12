use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    Attribute, DeriveInput, Expr, ExprAssign, Token,
};

pub fn generate(input: DeriveInput) -> TokenStream {
    let DeriveInput { ident, data, .. } = input;
    let st = match data {
        syn::Data::Struct(st) => st,
        _ => panic!("only struct can derive mockable"),
    };
    let new_fields: Vec<_> = st
        .fields
        .iter()
        .map(|f| {
            f.ident
                .as_ref()
                .map(|id| match find_attr("mockable_const", &f.attrs) {
                    Some(attr) => {
                        let val: Expr = attr
                            .parse_args()
                            .expect("expect an expression in mockable_value");
                        quote!(#id: #val,)
                    }
                    None => {
                        quote!(#id: diesel_ext::db_tester::Mockable::mock_new(rng),)
                    }
                })
        })
        .collect();
    quote! {
        impl diesel_ext::db_tester::Mockable for #ident {
            fn mock_new(rng: &mut rand::rngs::ThreadRng) -> Self {
                Self {
                    #(#new_fields)*
                }
            }
        }
    }
}

fn find_attr<'a, 'b>(want: &'a str, attrs: &'b [Attribute]) -> Option<&'b Attribute> {
    attrs.iter().find(|a| a.path().is_ident(want))
}

pub struct FieldValues(HashMap<Ident, Expr>);

impl Parse for FieldValues {
    fn parse(input: ParseStream) -> Result<Self> {
        let tokens = input.parse_terminated(
            // a list of `k = v` paris, i.e., k1 = v1, k2 = v2
            ExprAssign::parse,
            Token![,],
        )?;
        let args = tokens
            .into_iter()
            .map(|tk| {
                let k = match *tk.left {
                    Expr::Path(p) => p.path.get_ident().unwrap().clone(),
                    k => return Err(input.error(format!("key must be an ident, got: {:?}", k))),
                };
                let v = *tk.right;
                Ok((k, v))
            })
            .collect::<Result<_>>()?;
        Ok(Self(args))
    }
}
