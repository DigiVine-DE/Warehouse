use darling::{export::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    DataEnum,
    Fields::{self, Named, Unit, Unnamed},
    FieldsUnnamed, Ident, Token, Type, Variant,
};

pub enum TypeState<'a> {
    Null,
    Vec(&'a Type),
    Option(&'a Type),
    Rest(&'a Type),
}

impl<'a> From<&'a Type> for TypeState<'a> {
    fn from(ty: &'a Type) -> Self {
        if let Some(ty) = option_inner_type(ty) {
            return TypeState::Option(ty);
        }
        if let Some(ty) = vec_inner_type(ty) {
            return TypeState::Vec(ty);
        }
        TypeState::Rest(ty)
    }
}

impl<'a> From<Option<&'a Type>> for TypeState<'a> {
    fn from(value: Option<&'a Type>) -> Self {
        if let Some(ty) = value {
            ty.into()
        } else {
            TypeState::Null
        }
    }
}

pub fn get_field_type(f: &Fields) -> Option<&Type> {
    match f {
        Unnamed(unnamed) => Some(get_unnamed_type(unnamed)),
        Named(_) => panic!("named filed type is not yet supported!"),
        Unit => None,
    }
}

fn get_unnamed_type(f: &FieldsUnnamed) -> &Type {
    assert_eq!(f.unnamed.len(), 1, "Unnamed Field length must be 1");
    &f.unnamed.first().unwrap().ty
}

fn angle_bracketed_inner_type<'a>(ty: &'a Type, container: &str) -> Option<&'a Type> {
    match ty {
        Type::Path(p) => {
            let fst = p.path.segments.first()?;
            if fst.ident != container {
                return None;
            };
            match &fst.arguments {
                syn::PathArguments::AngleBracketed(a) => match a.args.first()? {
                    syn::GenericArgument::Type(t) => Some(t),
                    _ => None,
                },
                _ => None,
            }
        }
        _ => None,
    }
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    angle_bracketed_inner_type(ty, "Option")
}

fn vec_inner_type(ty: &Type) -> Option<&Type> {
    angle_bracketed_inner_type(ty, "Vec")
}

#[derive(Debug)]
pub enum Arg {
    Ident(Ident),
    KV(String, Ident),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("try parsing {:?} by =", input);
        let fields = input.parse_terminated(Ident::parse, Token![=])?;
        println!("parsed: {:?}", fields);
        match fields.len() {
            1 => Ok(Arg::Ident(fields.into_iter().next().unwrap())),
            2 => {
                let mut it = fields.into_iter();
                let k = it.next().unwrap().to_string();
                let v = it.next().unwrap();
                Ok(Arg::KV(k, v))
            }
            _ => Err(input.error("unsupported args")),
        }
    }
}

pub struct AttrsIdents {
    pub idents: Vec<Ident>,
}

impl Parse for AttrsIdents {
    fn parse(args: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(args)?;
        Ok(AttrsIdents {
            idents: vars.into_iter().collect(),
        })
    }
}

fn try_find_variant<'a>(
    mut varients: impl Iterator<Item = &'a Variant>,
    pred: impl Fn(String) -> bool,
) -> Option<&'a Variant> {
    varients.find(|v| pred((v.ident).to_string()))
}

pub fn find_variant_with_name<'a>(en: &'a DataEnum, name: &str) -> Option<&'a Variant> {
    try_find_variant(en.variants.iter(), |got| name == got)
}

pub fn find_variant_with_prefix<'a>(en: &'a DataEnum, name: &str) -> Option<&'a Variant> {
    try_find_variant(en.variants.iter(), |got| got.starts_with(name))
}

pub fn parse_args<T: FromMeta>(args: TokenStream) -> std::result::Result<T, TokenStream> {
    let attr_args = NestedMeta::parse_meta_list(args.into())
        .map_err(|e| TokenStream::from(darling::Error::from(e).write_errors()))?;
    T::from_list(&attr_args).map_err(|e| TokenStream::from(e.write_errors()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_str;

    #[test]
    fn test_get_unnamed_type() {
        let field_i32: FieldsUnnamed = parse_str("(i32)").unwrap();
        let type_i32 = get_unnamed_type(&field_i32);
        assert_eq!(quote!(#type_i32).to_string(), "i32");

        let field_tuple: FieldsUnnamed = parse_str("((i32,f64))").unwrap();
        let type_tuple = get_unnamed_type(&field_tuple);
        assert_eq!(quote!(#type_tuple).to_string(), "(i32 , f64)");
    }

    #[test]
    fn test_is_vec() {
        let ty: Type = parse_str("Vec<i32>").unwrap();
        let inner = vec_inner_type(&ty);
        assert!(inner.is_some());
        assert_eq!(inner.unwrap(), &parse_str("i32").unwrap());

        let ty: Type = parse_str("Vec<InlineResponse>").unwrap();
        let inner = vec_inner_type(&ty);
        assert!(inner.is_some());
        assert_eq!(inner.unwrap(), &parse_str("InlineResponse").unwrap());

        let ty: Type = parse_str("f64").unwrap();
        assert!(vec_inner_type(&ty).is_none());

        let ty: Type = parse_str("InlineResponse").unwrap();
        assert!(vec_inner_type(&ty).is_none());
    }
}
