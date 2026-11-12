use crate::utils::{find_variant_with_name, find_variant_with_prefix, get_field_type, TypeState};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::iter::Iterator;
use syn::{Data::Enum, DeriveInput, Variant};

pub fn generate(input: DeriveInput) -> TokenStream {
    let mut tokens = Vec::new();
    macro_rules! done {
    () => {
       return quote!(#(#tokens)*).into()
      };
    }

    let DeriveInput { ident, data, .. } = input;
    let my_enum = match data {
        Enum(my_enum) => my_enum,
        _ => done!(),
    };

    // the name of a response variant is controlled by `description` in the response api specification.
    let success_var = find_variant_with_prefix(&my_enum, "Successful");
    let error_var = find_variant_with_name(&my_enum, "BackendError");

    if find_variant_with_name(&my_enum, "BadRequest").is_some() {
        tokens.push(gen_bad_request_response(&ident));
    }

    if find_variant_with_name(&my_enum, "TheSpecifiedResourceWasNotFound").is_some() {
        tokens.push(gen_impl_not_found_response(&ident));
    }

    match (success_var, error_var) {
        (None, None) => (),
        (None, Some(_)) => tokens.push(gen_impl_as_response_error(&ident)),
        (Some(success_var), None) => {
            let (tks, _) = handle_success_variant(&ident, success_var);
            tokens.extend_from_slice(&tks);
        }
        (Some(success_var), Some(_)) => {
            tokens.push(gen_impl_as_response_error(&ident));
            let (tks, success_type_state) = handle_success_variant(&ident, success_var);
            tokens.extend_from_slice(&tks);
            tokens.push(gen_from_result(
                &ident,
                &success_var.ident,
                success_type_state,
            ))
        }
    };

    done!()
}

fn handle_success_variant<'a>(
    enum_ident: &Ident,
    var: &'a Variant,
) -> (Vec<TokenStream>, TypeState<'a>) {
    let ty = get_field_type(&var.fields);
    let type_state = TypeState::from(ty);
    let tokens = match type_state {
        TypeState::Null => vec![gen_impl_unit_success_response(enum_ident, &var.ident)],
        TypeState::Vec(ty) => vec![gen_from_iter(enum_ident, &var.ident, ty)],
        _ => vec![gen_from_ok_variant(enum_ident, &var.ident, ty.unwrap())],
    };
    (tokens, type_state)
}

fn gen_impl_unit_success_response(resp_ident: &Ident, variant: &Ident) -> TokenStream {
    quote! {
      impl UnitSuccessResponse for #resp_ident {
         fn succeed() -> Self {
             Self::#variant
         }
      }
    }
}

fn gen_from_ok_variant(resp_ident: &Ident, enum_var: &Ident, ok_type: &syn::Type) -> TokenStream {
    quote! {
      impl<T: Into<#ok_type>> From<T> for #resp_ident {
          fn from(v: T) -> Self {
              Self::#enum_var(v.into())
          }
      }
    }
}

fn gen_from_iter(resp_ident: &Ident, enum_var: &Ident, inner: &syn::Type) -> TokenStream {
    quote! {
      impl<It: Into<#inner>> ::std::iter::FromIterator<It> for #resp_ident {
          fn from_iter<T: ::std::iter::IntoIterator<Item = It>>(iter: T) -> Self {
              Self::#enum_var(iter.into_iter().map(|x| x.into()).collect())
          }
      }
    }
}

fn gen_impl_as_response_error(resp_ident: &Ident) -> TokenStream {
    quote! {
      impl<E: ::std::string::ToString> AsBackendErrorResp<#resp_ident> for E {
          fn default_code() -> &'static str {
              stringify!(#resp_ident)
          }

          fn as_backend_error_resp_with_code(&self, code: String) -> #resp_ident {
              #resp_ident::BackendError((code, self.to_string()).into())
          }
      }
    }
}

fn gen_bad_request_response(resp_ident: &Ident) -> TokenStream {
    quote! {
        impl<E: ::std::string::ToString> AsBadRequestResponse<#resp_ident> for E {
            fn as_bad_request(self) -> #resp_ident {
                #resp_ident::BadRequest(models::InlineResponse400::new(format!(
                    "Invalid request: {}",
                    self.to_string()
                )))
            }
        }
    }
}

fn gen_impl_not_found_response(resp_ident: &Ident) -> TokenStream {
    quote! {
      impl NotFoundResponse for #resp_ident {
         fn not_found() -> Self {
             Self::TheSpecifiedResourceWasNotFound
         }
      }
    }
}

fn gen_from_result(resp_ident: &Ident, ok_var: &Ident, ok_type: TypeState) -> TokenStream {
    match ok_type {
        TypeState::Null => quote! {
           impl<E> From<Result<(), E>> for #resp_ident
           where
               E: AsBackendErrorResp<#resp_ident>,
           {
               fn from(res: Result<(), E>) -> Self {
                   match res {
                       Ok(()) => Self::#ok_var,
                       Err(v) => v.as_backend_error_resp(),
                   }
               }
           }
        },
        _ => {
            quote! {
              impl<T, E> From<Result<T, E>> for #resp_ident
              where
                  T: Into<#resp_ident>,
                  E: AsBackendErrorResp<#resp_ident>,
              {
                  fn from(res: Result<T, E>) -> Self {
                      match res {
                          Ok(v) => v.into(),
                          Err(v) => v.as_backend_error_resp(),
                      }
                  }
              }
            }
        }
    }
}
