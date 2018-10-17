use proc_macro2;
use quote::{ToTokens, TokenStreamExt};
use syn;
use tl_lang_syn as tlsn;


#[derive(Debug, Eq, PartialEq)]
pub struct Path(pub tlsn::ParameterizedPath);

impl Path {
    pub fn to_syn_path(&self) -> syn::Path {
        syn::parse2(self.into_token_stream()).unwrap()
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tlsn::ParameterizedPath { ref path, ref args } = self.0;

        let segments = path.segments.iter().cloned().map(|ident| {
            syn::PathSegment::from(syn::Ident::new(ident.as_str(), proc_macro2::Span::call_site()))
        });

        let args = args.as_ref().map(|args| {
            match *args {
                tlsn::GenericArguments::AngleBracketed(ref args) => {
                    let args = args.args.iter().map(|path| {
                        let s = path.path.segments[0].as_str();
                        syn::Ident::new(s, proc_macro2::Span::call_site())
                    });
                    quote! { <#(#args),*> }
                },
                tlsn::GenericArguments::SpaceSeparated(ref args) => {
                    let args = args.args.iter().map(|path| {
                        let s = path.path.segments[0].as_str();
                        syn::Ident::new(s, proc_macro2::Span::call_site())
                    });
                    quote! { <#(#args),*> }
                },
            }
        });

        tokens.append_all(quote! {
            #(::#segments)* #args
        });
    }
}
