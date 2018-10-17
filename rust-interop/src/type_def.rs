use std::collections::HashMap;

use proc_macro2;
use quote::{ToTokens, TokenStreamExt};
use syn;
use tl_lang_syn as tlsn;

use ::constructor_variant::ConstructorVariant;
use ::ident::Ident;
use ::utils::TraversalMode;



#[derive(Debug, Eq, PartialEq)]
pub struct TypeDefNamespace {
    pub name: Ident,
    pub type_defs: HashMap<tlsn::Ident, TypeDef>,
    pub namespaces: HashMap<tlsn::Ident, TypeDefNamespace>,
}

impl TypeDefNamespace {
    fn with_str(name: &str) -> Option<Self> {
        Ident::with_str(name).map(Self::with_ident)
    }

    fn with_tl_ident(name: tlsn::Ident) -> Self {
        Self::with_ident(Ident(name))
    }

    fn with_ident(name: Ident) -> Self {
        Self {
            name,
            type_defs: HashMap::new(),
            namespaces: HashMap::new(),
        }
    }

    pub fn from_tl_items(items: &[tlsn::Item]) -> Self {
        let mut mode = TraversalMode::Types;
        let mut type_def_ns = Self::with_str("types").unwrap();  // FIXME

        for item in items {
            match *item {
                tlsn::Item::Combinator(ref combinator) => match mode {
                    TraversalMode::Types => {
                        let segments = &combinator.result_type.path.segments;

                        if segments.len() == 1 {
                            match segments[0].as_str() {
                                "Bool"   |
                                "True"   |
                                "Vector" => continue,
                                _ => (),
                            }
                        }

                        let mut type_def_ns = &mut type_def_ns;

                        for (i, name_segment) in segments.iter().enumerate() {
                            if i == segments.len() - 1 {
                                // Avoid cloning `Ident`s. Otherwise it could've been done as:
                                //
                                //     type_def_ns
                                //         .type_defs
                                //         .entry(name_segment.clone())
                                //         .or_insert(TypeDef {
                                //             name: Ident(name_segment.clone()),
                                //             constructor_variants: Vec::new(),
                                //         })
                                //         .constructor_variants
                                //         .push(ConstructorVariant::from_tl_combinator(combinator));
                                if !type_def_ns.type_defs.contains_key(name_segment) {
                                    type_def_ns.type_defs.insert(
                                        name_segment.clone(),
                                        TypeDef {
                                            name: Ident(name_segment.clone()),
                                            constructor_variants: Vec::new(),
                                        },
                                    );
                                }

                                type_def_ns
                                    .type_defs
                                    .get_mut(name_segment)
                                    .unwrap()
                                    .constructor_variants
                                    .push(ConstructorVariant::from_tl_combinator(combinator));
                            } else {
                                // Avoid cloning `Ident`s. Otherwise it could've been done as:
                                //
                                //     type_def_ns = {type_def_ns}
                                //         .namespaces
                                //         .entry(name_segment.clone())
                                //         .or_insert(Self::with_tl_ident(name_segment.clone()));
                                if !type_def_ns.namespaces.contains_key(name_segment) {
                                    type_def_ns.namespaces.insert(
                                        name_segment.clone(),
                                        Self::with_tl_ident(name_segment.clone()),
                                    );
                                }

                                type_def_ns = {type_def_ns}
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

        type_def_ns
    }

    pub fn to_syn_mod(&self) -> syn::ItemMod {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for TypeDefNamespace {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TypeDefNamespace { ref name, ref type_defs, ref namespaces } = *self;
        let type_defs = type_defs.values();
        let namespaces = namespaces.values();

        tokens.append_all(quote! {
            pub mod #name {
                #(#type_defs)*
                #(#namespaces)*
            }
        });
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct TypeDef {
    pub name: Ident,
    pub constructor_variants: Vec<ConstructorVariant>,
}

impl TypeDef {
    pub fn to_syn_enum(&self) -> syn::ItemEnum {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TypeDef { ref name, ref constructor_variants } = *self;

        tokens.append_all(quote! {
            #[derive(
                Clone, Debug,
                Serialize, Deserialize,
                MtProtoIdentifiable, MtProtoSized,
            )]
            pub enum #name {
                #(#constructor_variants,)*
            }

            impl ::tl::TLObject for #name {
                fn object_type() -> ::tl::dynamic::ObjectType {
                    ::tl::dynamic::ObjectType::Type
                }
            }
        });
    }
}
