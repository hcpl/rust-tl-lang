use std::collections::HashMap;

use proc_macro2;
use quote::{ToTokens, TokenStreamExt};
use syn;
use tl_lang_syn as tlsn;

use ::field::Field;
use ::ident::Ident;
use ::path::Path;
use ::utils::TraversalMode;


#[derive(Debug, Eq, PartialEq)]
pub struct FunctionDefNamespace {
    pub name: Ident,
    pub function_defs: Vec<FunctionDef>,
    pub namespaces: HashMap<tlsn::Ident, FunctionDefNamespace>,
}

impl FunctionDefNamespace {
    fn with_str(name: &str) -> Option<Self> {
        Ident::with_str(name).map(Self::with_ident)
    }

    fn with_tl_ident(name: tlsn::Ident) -> Self {
        Self::with_ident(Ident(name))
    }

    fn with_ident(name: Ident) -> Self {
        Self {
            name,
            function_defs: Vec::new(),
            namespaces: HashMap::new(),
        }
    }

    pub fn from_tl_items(items: &[tlsn::Item]) -> Self {
        let mut mode = TraversalMode::Types;
        let mut function_def_ns = Self::with_str("functions").unwrap();  // FIXME

        for item in items {
            match *item {
                tlsn::Item::Combinator(ref combinator) => match mode {
                    TraversalMode::Types => (),
                    TraversalMode::Functions => {
                        let segments = &combinator.name.segments;
                        let mut function_def_ns = &mut function_def_ns;

                        for (i, name_segment) in segments.iter().enumerate() {
                            if i == segments.len() - 1 {
                                function_def_ns.function_defs
                                    .push(FunctionDef::from_tl_combinator(combinator));
                            } else {
                                // Avoid cloning `Ident`s. Otherwise it could've been done as:
                                //
                                //     function_def_ns = {function_def_ns}
                                //         .namespaces
                                //         .entry(name_segment.clone())
                                //         .or_insert(Self::with_tl_ident(name_segment.clone()));
                                if !function_def_ns.namespaces.contains_key(name_segment) {
                                    function_def_ns.namespaces.insert(
                                        name_segment.clone(),
                                        Self::with_tl_ident(name_segment.clone()),
                                    );
                                }

                                function_def_ns = {function_def_ns}
                                    .namespaces
                                    .get_mut(name_segment)
                                    .unwrap();
                            }
                        }
                    },
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

        function_def_ns
    }

    pub fn to_syn_mod(&self) -> syn::ItemMod {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for FunctionDefNamespace {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let FunctionDefNamespace { ref name, ref function_defs, ref namespaces } = *self;
        let namespaces = namespaces.values();

        tokens.append_all(quote! {
            pub mod #name {
                #(#function_defs)*
                #(#namespaces)*
            }
        });
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct FunctionDef {
    pub name: Ident,
    pub id: u32,
    pub generics: Vec<Ident>,
    pub fields: Vec<Field>,
    pub return_type: Path,
}

impl FunctionDef {
    pub fn from_tl_combinator(combinator: &tlsn::ItemCombinator) -> Self {
        let tlsn::ItemCombinator {
            ref name,
            ref combinator_id,
            ref opt_params,
            ref params,
            ref result_type,
            ..
        } = *combinator;

        let name = Ident::from_path_last_segment(name).unwrap();  // FIXME
        let id = combinator_id.as_ref().unwrap().id.id;  // FIXME
        let generics = opt_params.iter().filter_map(|opt_param| match opt_param.ty {
            tlsn::Type::ParameterizedPath(ref parameterized_path) => {
                match parameterized_path.safe_parameterized_path {
                    tlsn::SafeParameterizedPath::SpaceImmune(ref space_immune) => {
                        let var_idents = &opt_param.var_idents;
                        let args = &space_immune.args;
                        let segments = &space_immune.path.segments;

                        if args.is_none() && segments.len() == 1 && segments[0].as_str() == "Type" {
                            return Some(var_idents.iter().cloned().map(Ident));
                        }

                        None
                    },
                    _ => None,
                }
            },
            _ => None,
        }).flat_map(|idents| idents).collect();
        let fields = Field::from_tl_params(params);
        let return_type = Path(result_type.clone());

        Self { name, id, generics, fields, return_type }
    }

    pub fn to_syn_struct(&self) -> syn::ItemStruct {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for FunctionDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let FunctionDef { ref name, id, ref generics, ref fields, .. } = *self;

        let id_hex_string = format!("{:#x}", id);
        let (generics, impl_generics) = if generics.is_empty() {
            (None, None)
        } else {
            (
                Some(quote!(<#(#generics),*>)),
                Some(quote!(<#(#generics:
                    ::std::clone::Clone +
                    ::serde::Serialize +
                    ::serde_mtproto::MtProtoSized +
                    'static
                ),*>)),
            )
        };
        let fields = if fields.is_empty() {
            quote!(;)
        } else {
            quote! {
                { #(#fields,)* }
            }
        };

        tokens.append_all(quote! {
            #[derive(
                Clone, Debug,
                Serialize, Deserialize,
                MtProtoIdentifiable, MtProtoSized,
            )]
            #[mtproto_identifiable(id = #id_hex_string)]
            pub struct #name #generics #fields

            impl #impl_generics ::tl::TLObject for #name #generics {
                fn object_type() -> ::tl::dynamic::ObjectType {
                    ::tl::dynamic::ObjectType::Function
                }
            }
        });
    }
}
