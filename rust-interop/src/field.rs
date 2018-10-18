use either::Either;
use proc_macro2;
use tl_lang_syn as tlsn;

use ::ident::Ident;
use ::token_generator::TokenGenerator;
use ::ty::Type;


#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Named(FieldNamed),
    Unnamed(FieldUnnamed),
}

#[derive(Debug, Eq, PartialEq)]
pub struct FieldNamed {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FieldUnnamed {
    pub index: usize,
    pub ty: Type,
}

impl Field {
    fn from_ident_tl_ty(name: Ident, ty: &tlsn::Type) -> Option<Self> {
        Type::from_tl_type(ty).map(|ty| Field::Named(FieldNamed { name, ty }))
    }

    fn from_index_tl_ty(index: usize, ty: &tlsn::Type) -> Option<Self> {
        Type::from_tl_type(ty).map(|ty| Field::Unnamed(FieldUnnamed { index, ty }))
    }

    pub fn from_tl_params(params: &[tlsn::Param]) -> Vec<Self> {
        let mut unnamed_field_index = 0;

        params.iter().flat_map(|param| match *param {
            tlsn::Param::Conditional(ref conditional) => {
                let tlsn::ParamConditional {
                    ref var_ident,
                    ref conditional_param_def,
                    ref ty,
                    ..
                } = *conditional;

                let res = match *conditional_param_def {
                    None => Self::from_ident_tl_ty(Ident(var_ident.clone()), ty),
                    Some(_) => None,  // FIXME
                };

                Either::Left(res.into_iter())
            },
            tlsn::Param::Repeated(_) => unimplemented!(),
            tlsn::Param::WithParen(ref with_paren) => {
                Either::Right(with_paren.var_idents.iter().cloned().filter_map(move |var_ident| {
                    Self::from_ident_tl_ty(Ident(var_ident), &with_paren.ty)
                }))
            },
            tlsn::Param::TypeOnly(ref type_only) => {
                let index = unnamed_field_index;
                unnamed_field_index += 1;

                // FIXME
                Either::Left(Self::from_index_tl_ty(index, &type_only.ty).into_iter())
            },
        }).collect()
    }
}
