use proc_macro2;
use tl_lang_syn as tlsn;

use ::token_generator::TokenGenerator;
use ::utils;


#[derive(Debug, Eq, PartialEq)]
pub struct Ident(pub tlsn::Ident);

impl Ident {
    pub(crate) fn with_str(string: &str) -> Option<Self> {
        utils::tl_ident_span_zeroed(string).map(Ident)
    }

    pub(crate) fn from_path_last_segment(path: &tlsn::Path) -> Option<Self> {
        path.segments.last().map(|pair| {
            Ident(pair.into_value().clone())
        })
    }
}
