use std::iter;

use tl_lang_syn as tlsn;

use ::ident::Ident;
use ::path::Path;
use ::utils;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructorVariant {
    pub name: Ident,
    pub id: u32,
    pub struct_path: Path,
}

impl ConstructorVariant {
    pub fn from_tl_combinator(combinator: &tlsn::ItemCombinator) -> Self {
        let tlsn::ItemCombinator {
            ref name,
            ref combinator_id,
            ..
        } = *combinator;

        let cs_name = {
            let last_segment = name.segments.last().unwrap().into_value();
            Ident::with_str(last_segment.as_str()).unwrap()  // FIXME
        };
        let id = combinator_id.as_ref().unwrap().id.id;  // FIXME
        let struct_path = Path(tlsn::ParameterizedPath {
            path: tlsn::Path {
                segments: iter::once(utils::tl_ident_span_zeroed("schema").unwrap())
                    .chain(iter::once(utils::tl_ident_span_zeroed("constructors").unwrap()))
                    .chain(name.segments.iter().cloned())
                    .collect(),
            },
            args: None,
        });

        Self { name: cs_name, id, struct_path }
    }
}
