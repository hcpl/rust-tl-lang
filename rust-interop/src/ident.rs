use tl_lang_syn as tlsn;

use ::utils;


#[derive(Clone, Debug, Eq, PartialEq)]
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

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
