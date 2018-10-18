use std::collections::HashMap;

use tl_lang_syn as tlsn;

use ::field::Field;
use ::ident::Ident;
use ::utils::TraversalMode;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructorDefNamespace {
    pub name: Ident,
    pub constructor_defs: Vec<ConstructorDef>,
    pub namespaces: HashMap<tlsn::Ident, ConstructorDefNamespace>,
}

impl ConstructorDefNamespace {
    fn with_str(name: &str) -> Option<Self> {
        Ident::with_str(name).map(Self::with_ident)
    }

    fn with_tl_ident(name: tlsn::Ident) -> Self {
        Self::with_ident(Ident(name))
    }

    fn with_ident(name: Ident) -> Self {
        Self {
            name,
            constructor_defs: Vec::new(),
            namespaces: HashMap::new(),
        }
    }

    pub fn from_tl_items(items: &[tlsn::Item]) -> Self {
        let mut mode = TraversalMode::Types;
        let mut constructor_def_ns = Self::with_str("constructors").unwrap();

        for item in items {
            match *item {
                tlsn::Item::Combinator(ref combinator) => match mode {
                    TraversalMode::Types => {
                        let segments = &combinator.name.segments;

                        if segments.len() == 1 {
                            match segments[0].as_str() {
                                "boolFalse" |
                                "boolTrue"  |
                                "true"      |
                                "vector"    => continue,
                                _ => (),
                            }
                        }

                        let mut constructor_def_ns = &mut constructor_def_ns;

                        for (i, name_segment) in segments.iter().enumerate() {
                            if i == segments.len() - 1 {
                                constructor_def_ns.constructor_defs
                                    .push(ConstructorDef::from_tl_combinator(combinator));
                            } else {
                                // Avoid cloning `Ident`s. Otherwise it could've been done as:
                                //
                                //     constructor_def_ns = {constructor_def_ns}
                                //         .namespaces
                                //         .entry(name_segment.clone())
                                //         .or_insert(Self::with_tl_ident(name_segment.clone()));
                                if !constructor_def_ns.namespaces.contains_key(name_segment) {
                                    constructor_def_ns.namespaces.insert(
                                        name_segment.clone(),
                                        Self::with_tl_ident(name_segment.clone()),
                                    );
                                }

                                constructor_def_ns = {constructor_def_ns}
                                    .namespaces
                                    .get_mut(name_segment)
                                    .unwrap();
                            }
                        }
                    },
                    TraversalMode::Functions => (),
                },
                tlsn::Item::Delimiter(ref delimiter) => match (mode, &delimiter.delimiter) {
                    (TraversalMode::Types, &tlsn::Delimiter::Functions(_)) => {
                        mode = TraversalMode::Functions;
                    },
                    (TraversalMode::Functions, &tlsn::Delimiter::Types(_)) => {
                        mode = TraversalMode::Types;
                    },
                    _ => panic!("wrong delimiter"),  // FIXME
                },
                tlsn::Item::Layer(_)   |
                tlsn::Item::Comment(_) => (),
            }
        }

        constructor_def_ns
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructorDef {
    pub name: Ident,
    pub fields: Vec<Field>,
}

impl ConstructorDef {
    pub fn from_tl_combinator(combinator: &tlsn::ItemCombinator) -> Self {
        let tlsn::ItemCombinator {
            ref name,
            ref params,
            ..
        } = *combinator;

        let name = Ident::from_path_last_segment(name).unwrap();  // FIXME
        let fields = Field::from_tl_params(params);

        Self { name, fields }
    }
}
