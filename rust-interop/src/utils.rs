use tl_lang_syn as tlsn;


pub(crate) enum TraversalMode {
    Types,
    Functions,
}


pub(crate) fn tl_ident_span_zeroed(string: &str) -> Option<tlsn::Ident> {
    tlsn::Ident::new(tlsn::span::Span::zeroed(), string)
}


macro_rules! try_option {
    ($e:expr) => {{
        match { $e } {
            Some(v) => v,
            None => return None,
        }
    }};
}
