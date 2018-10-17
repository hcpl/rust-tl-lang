use proc_macro2;
use quote::ToTokens;
use syn;
use tl_lang_syn as tlsn;

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

    pub fn to_syn_ident(&self) -> syn::Ident {
        let escaped_ident = match self.0.as_str() {
            "type" => "type_",
            other => other,
        };

        syn::Ident::new(escaped_ident, proc_macro2::Span::call_site())
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_syn_ident().to_tokens(tokens);
    }
}
