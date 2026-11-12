use crate::utils::TypeState;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::iter::IntoIterator;
use syn::{DataStruct, DeriveInput, Type};

pub fn generate(input: DeriveInput) -> TokenStream {
    let mut tokens = Vec::new();
    macro_rules! done {
    () => {
       return quote!(#(#tokens)*).into()
      };
    }

    let DeriveInput { ident, data, .. } = input;
    let st = match data {
        syn::Data::Struct(st) => st,
        _ => done!(),
    };

    let mut types = get_fields(st);
    match types.len() {
        0 => (),
        1 => {
            let (from_ident, from_ty) = types.pop().unwrap();
            tokens = gen_from_single(&ident, &from_ident, &from_ty);
        }
        _ => tokens.push(gen_from_multi(&ident, types)),
    };

    done!()
}

fn get_fields(st: DataStruct) -> Vec<(Ident, Type)> {
    let st = match st.fields {
        syn::Fields::Named(st) => st,
        _ => return Vec::new(),
    };

    st.named
        .into_iter()
        .map(|f| (f.ident.expect("only named field is supported"), f.ty))
        .collect()
}

fn gen_from_multi(ident: &Ident, fields: Vec<(Ident, Type)>) -> TokenStream {
    let (from_idents, from_types): (Vec<Ident>, Vec<Type>) = fields.into_iter().unzip();
    quote! {
          impl ::std::convert::From<(#(#from_types),*)> for #ident {
              fn from(v: (#(#from_types),*)) -> Self {
                  let (#(#from_idents),*) = v;
                  #ident { #(#from_idents),* }
              }
          }
    }
}

fn gen_from_single(ident: &Ident, from_ident: &Ident, from_ty: &Type) -> Vec<TokenStream> {
    let mut tokens = Vec::new();
    tokens.push(quote! {
      impl ::std::convert::From<#from_ty> for #ident {
          fn from(v: #from_ty) -> Self {
              #ident { #from_ident: v }
          }
      }
    });
    if let TypeState::Option(inner) = from_ty.into() {
        tokens.push(quote! {
          impl ::std::convert::From<#inner> for #ident {
              fn from(v: #inner) -> Self {
                  #ident { #from_ident: Some(v) }
              }
          }
        });
    }
    tokens
}
