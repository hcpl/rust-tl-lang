use std::iter;

use proc_macro2;
use quote::{ToTokens, TokenStreamExt};
use syn;
use tl_lang_syn as tlsn;

use ::ident::Ident;
use ::path::Path;
use ::utils;


#[derive(Debug ,Eq, PartialEq)]
pub enum Type {
    BuiltIn(TypeBuiltIn),
    Path(Path),
    Generic(Ident),
}

impl Type {
    pub fn from_tl_type(mut ty: &tlsn::Type) -> Option<Self> {
        loop {
            match *ty {
                tlsn::Type::Int(_) => return None,  // FIXME
                tlsn::Type::ParameterizedPath(ref parameterized_path) => {
                    //return Some(Self::from_tl_type_parameterized_path(parameterized_path));
                    return Some(Self::from_tl_safe_parameterized_path(
                        &parameterized_path.safe_parameterized_path,
                    ));
                },
                tlsn::Type::TypeParameter(ref type_parameter) => {
                    return Some(Type::Generic(Ident(type_parameter.ident.clone())));
                },
                tlsn::Type::Bare(ref bare) => {
                    ty = &*bare.inner;
                },
            }
        }
    }

    fn from_tl_parameterized_path(
        parameterized_path: &tlsn::ParameterizedPath,
    ) -> Self {
        match TypeBuiltIn::from_tl_parameterized_path(parameterized_path) {
            Some(built_in) => Type::BuiltIn(built_in),
            None => Type::Path(Path(tlsn::ParameterizedPath {
                path: tlsn::Path {
                    segments: iter::once(utils::tl_ident_span_zeroed("schema").unwrap())
                        .chain(iter::once(utils::tl_ident_span_zeroed("types").unwrap()))
                        .chain(parameterized_path.path.segments.iter().cloned())
                        .collect(),
                },
                args: parameterized_path.args.clone(),
            })),
        }
    }

    fn from_tl_safe_parameterized_path_space_immune(
        space_immune: &tlsn::SafeParameterizedPathSpaceImmune,
    ) -> Self {
        match TypeBuiltIn::from_tl_safe_parameterized_path_space_immune(space_immune) {
            Some(built_in) => Type::BuiltIn(built_in),
            None => Type::Path(Path(tlsn::ParameterizedPath {
                path: tlsn::Path {
                    segments: iter::once(utils::tl_ident_span_zeroed("schema").unwrap())
                        .chain(iter::once(utils::tl_ident_span_zeroed("types").unwrap()))
                        .chain(space_immune.path.segments.iter().cloned())
                        .collect(),
                },
                args: space_immune.args.clone().map(tlsn::GenericArguments::AngleBracketed),
            })),
        }
    }

    fn from_tl_safe_parameterized_path(
        safe_parameterized_path: &tlsn::SafeParameterizedPath,
    ) -> Self {
        match *safe_parameterized_path {
            tlsn::SafeParameterizedPath::SpaceImmune(ref space_immune) => {
                Self::from_tl_safe_parameterized_path_space_immune(space_immune)
            },
            tlsn::SafeParameterizedPath::Parenthesized(ref parenthesized) => {
                Self::from_tl_parameterized_path(&parenthesized.parameterized_path)
            },
        }
    }

    pub fn to_syn_type(&self) -> syn::Type {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Type::BuiltIn(ref built_in) => built_in.to_tokens(tokens),
            Type::Path(ref path) => path.to_tokens(tokens),
            Type::Generic(ref type_param) => type_param.to_tokens(tokens),
        }
    }
}


#[derive(Debug ,Eq, PartialEq)]
pub enum TypeBuiltIn {
    Bool,
    True,
    Int,
    Long,
    Int128,
    Int256,
    Double,
    Bytes,
    String,
    Vector(Vec<Type>),
    VectorBoxed(Vec<Type>),
}

impl TypeBuiltIn {
    pub fn from_tl_parameterized_path(parameterized_path: &tlsn::ParameterizedPath) -> Option<Self> {
        let segments = &parameterized_path.path.segments;
        let args = &parameterized_path.args;

        fn types_from_generic_arguments(
            args: &tlsn::GenericArguments,
        ) -> Vec<Type> {
            match *args {
                tlsn::GenericArguments::AngleBracketed(ref angle_bracketed) => {
                    angle_bracketed.args
                        .iter()
                        .map(|arg| Type::from_tl_parameterized_path(arg))
                        .collect()
                },
                tlsn::GenericArguments::SpaceSeparated(ref space_separated) => {
                    space_separated.args
                        .iter()
                        .map(|arg| Type::from_tl_parameterized_path(arg))
                        .collect()
                },
            }
        }

        let built_in = if segments.len() == 1 {
            match *args {
                None => try_option!(Self::from_primitive(segments[0].as_str())),
                Some(ref args) => match segments[0].as_str() {
                    "vector" => TypeBuiltIn::Vector(types_from_generic_arguments(args)),
                    "Vector" => TypeBuiltIn::VectorBoxed(types_from_generic_arguments(args)),
                    _ => return None,
                },
            }
        } else {
            return None;
        };

        Some(built_in)
    }

    fn from_tl_safe_parameterized_path_space_immune(
        space_immune: &tlsn::SafeParameterizedPathSpaceImmune,
    ) -> Option<Self> {
        let segments = &space_immune.path.segments;
        let args = &space_immune.args;

        fn types_from_angle_bracketed(args: &tlsn::AngleBracketedGenericArguments) -> Vec<Type> {
            args.args
                .iter()
                .map(|arg| Type::from_tl_parameterized_path(arg))
                .collect()
        }

        let built_in = if segments.len() == 1 {
            match *args {
                None => try_option!(Self::from_primitive(segments[0].as_str())),
                Some(ref args) => match segments[0].as_str() {
                    "vector" => TypeBuiltIn::Vector(types_from_angle_bracketed(args)),
                    "Vector" => TypeBuiltIn::VectorBoxed(types_from_angle_bracketed(args)),
                    _ => return None,
                },
            }
        } else {
            return None;
        };

        Some(built_in)
    }

    pub fn from_tl_safe_parameterized_path(
        safe_parameterized_path: &tlsn::SafeParameterizedPath,
    ) -> Option<Self> {
        match *safe_parameterized_path {
            tlsn::SafeParameterizedPath::SpaceImmune(ref space_immune) => {
                Self::from_tl_safe_parameterized_path_space_immune(space_immune)
            },
            tlsn::SafeParameterizedPath::Parenthesized(ref parenthesized) => {
                Self::from_tl_parameterized_path(&parenthesized.parameterized_path)
            },
        }
    }

    fn from_primitive(string: &str) -> Option<Self> {
        let built_in = match string {
            "Bool"   => TypeBuiltIn::Bool,
            "true"   => TypeBuiltIn::True,
            "int"    => TypeBuiltIn::Int,
            "long"   => TypeBuiltIn::Long,
            "int128" => TypeBuiltIn::Int128,
            "int256" => TypeBuiltIn::Int256,
            "double" => TypeBuiltIn::Double,
            "bytes"  => TypeBuiltIn::Bytes,
            "string" => TypeBuiltIn::String,
            _ => return None,
        };

        Some(built_in)
    }

    pub fn to_syn_type(&self) -> syn::Type {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for TypeBuiltIn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(match *self {
            TypeBuiltIn::Bool   => quote!(bool),
            TypeBuiltIn::True   => quote!(()),
            TypeBuiltIn::Int    => quote!(i32),
            TypeBuiltIn::Long   => quote!(i64),
            TypeBuiltIn::Int128 => quote!(i128),
            TypeBuiltIn::Int256 => quote!(::manual_types::i256::I256),
            TypeBuiltIn::Double => quote!(f64),
            TypeBuiltIn::Bytes  => quote!(::serde_bytes::ByteBuf),
            TypeBuiltIn::String => quote!(String),
            TypeBuiltIn::Vector(ref args) => quote!(Vec<#(#args),*>),
            TypeBuiltIn::VectorBoxed(ref args) => quote!(::serde_mtproto::Boxed<Vec<#(#args),*>>),
        });
    }
}
