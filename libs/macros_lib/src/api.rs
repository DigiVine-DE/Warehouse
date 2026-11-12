use crate::utils::AttrsIdents;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_str, Expr, ExprAssign, Token,
};

pub fn gen_init(args: AttrsIdents) -> TokenStream {
    let assignments: Vec<_> = args
        .idents
        .iter()
        .map(|arg| match arg.to_string().as_str() {
            be if be.ends_with("_backend") => init_backend(be),
            "offset" => init_offset(&parse_str("offset").unwrap()),
            "limit" => init_limit(&parse_str("limit").unwrap()),
            other => panic!("unsupported args: {}", other),
        })
        .collect();
    quote!(#(#assignments)*)
}

pub fn gen_get(args: EndpointArgs) -> TokenStream {
    let id = args.get("id");
    args.with_backend(|be| {
        quote! {
            Ok(crate::response::Get::get(&mut #be, #id))
        }
    })
}

pub fn gen_post(args: EndpointArgs) -> TokenStream {
    let insert = args.get("insert");
    args.with_backend(|be| {
        quote! {
            Ok(crate::response::Post::post(&mut #be, #insert))
        }
    })
}

pub fn gen_upsert(args: EndpointArgs) -> TokenStream {
    let upsert = args.get("upsert");
    args.with_backend(|be| {
        quote! {
            Ok(crate::response::Upsert::upsert(&mut #be, #upsert))
        }
    })
}

pub fn gen_patch(args: EndpointArgs) -> TokenStream {
    let id = args.get("id");
    let patch = args.get("patch");
    args.with_backend(|be| {
        quote! {
            Ok(crate::response::Patch::patch(&mut #be, &#id, #patch))
        }
    })
}

pub fn gen_delete(args: EndpointArgs) -> TokenStream {
    let id = args.get("id");
    args.with_backend(|be| {
        quote! {
            Ok(crate::response::Delete::delete(&mut #be, #id))
        }
    })
}

pub fn gen_delete_all(args: EndpointArgs) -> TokenStream {
    let filter = args.get("filter");
    args.with_backend(|be| {
        quote! {
            Ok(crate::response::DeleteAll::delete_all(&mut #be, #filter))
        }
    })
}

pub fn gen_get_all(args: EndpointArgs) -> TokenStream {
    args.with_get_all_params(|be, offset, limit| {
        quote! {
            Ok(crate::response::GetAll::get_all(&mut #be, #offset, #limit))
        }
    })
}

pub fn gen_search(args: EndpointArgs) -> TokenStream {
    let filter = args.get("filter");
    args.with_get_all_params(|be, offset, limit| {
        quote! {
            Ok(crate::response::Search::search(&mut #be, #filter, #offset, #limit))
        }
    })
}

#[derive(Debug)]
pub struct EndpointArgs {
    backend: Ident,
    args: HashMap<String, Expr>,
}

impl EndpointArgs {
    fn get(&self, key: &str) -> Expr {
        self.args
            .get(key)
            .cloned()
            .unwrap_or_else(|| parse_str(key).unwrap())
    }

    fn with_backend<Gen: Fn(&Ident) -> TokenStream>(&self, gen: Gen) -> TokenStream {
        let be = &self.backend;
        let init_be = init_backend(&be.to_string());
        let rest = gen(be);
        quote! {{
            #init_be
            #rest
        }}
    }

    fn with_get_all_params<Gen: Fn(&Ident, &Expr, &Expr) -> TokenStream>(
        &self,
        gen: Gen,
    ) -> TokenStream {
        let offset = &self.get("offset");
        let limit = &self.get("limit");
        let init_offset = init_offset(offset);
        let init_limit = init_limit(limit);
        self.with_backend(|be| {
            let rest = gen(be, offset, limit);
            quote! {
                #init_offset
                #init_limit
                #rest
            }
        })
    }
}

impl Parse for EndpointArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let backend: Ident = input.parse()?;
        if input.parse::<Token![,]>().is_err() {
            return Ok(EndpointArgs {
                backend,
                args: HashMap::new(),
            });
        }
        let rest_tokens = input.parse_terminated(
            // a list of `k = v` paris, i.e., k1 = v1, k2 = v2
            ExprAssign::parse,
            Token![,],
        )?;
        let args = rest_tokens
            .into_iter()
            .map(|tk| {
                let k = match *tk.left {
                    Expr::Path(p) => p.path.get_ident().unwrap().to_string(),
                    k => return Err(input.error(format!("key must be an ident, got: {:?}", k))),
                };
                let v = *tk.right;
                Ok((k, v))
            })
            .collect::<Result<_>>()?;
        Ok(EndpointArgs { backend, args })
    }
}

fn init_backend(backend: &str) -> TokenStream {
    let ident = Ident::new(backend, Span::call_site());
    quote! {
        log::trace!("try to lock {}", #backend);
        let mut #ident = match self.backends.#ident.lock() {
            Ok(b) => b,
            Err(e) => {
                log::error!("unable to lock {}", #backend);
                return Ok(e.as_backend_error_resp_with_code(format!("CouldNotLock<{}>", #backend)))
            },
        };
        let mut #ident = match #ident.establish_connection() {
            Ok(c) => c,
            Err(e) => {
                log::error!("unable to connect {}", #backend);
                return Ok(e.as_backend_error_resp_with_code(format!("CouldNotConnect{}", #backend)));
            }
        };
    }
}

fn init_offset(offset: &Expr) -> TokenStream {
    quote! {
        let offset: usize = match std::convert::TryFrom::try_from(#offset.unwrap_or(DEFAULT_OFFSET))
        {
            Ok(u) => u,
            Err(e) => {
                log::info!("received offset is invalid: {:?}", #offset);
                return Err(ApiError(format!("InvalidOffsetError: {}", e)))
              },
        };
    }
}

fn init_limit(limit: &Expr) -> TokenStream {
    quote! {
        let limit: usize = match std::convert::TryFrom::try_from(#limit.unwrap_or(DEFAULT_LIMIT))
        {
            Ok(u) => u,
            Err(e) => {
                log::info!("received limit is invalid: {:?}", #limit);
                return Err(ApiError(format!("InvalidLimitError: {}", e)))
              },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_parse_args() {
        let args: EndpointArgs = parse_str("relational_backend, id = field_id").unwrap();
        assert_eq!(args.backend.to_string(), "relational_backend");
        assert_eq!(args.args["id"], parse_str("field_id").unwrap());
    }
}
