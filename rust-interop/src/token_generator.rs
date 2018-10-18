use std::fmt;

use proc_macro2;
use quote::ToTokens;


pub struct TokenGenerator<'a, T: 'a> {
    pub(crate) ref_value: &'a T,
    pub(crate) function: fn(&T, &mut proc_macro2::TokenStream),
}

impl<'a, T> TokenGenerator<'a, T> {
    pub fn new(ref_value: &'a T, function: fn(&T, &mut proc_macro2::TokenStream)) -> Self {
        Self { ref_value, function }
    }
}

impl<'a, T> fmt::Debug for TokenGenerator<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TokenGenerator")
            .field("ref_value", &self.ref_value)
            .finish()
    }
}

impl<'a, T> ToTokens for TokenGenerator<'a, T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        (self.function)(self.ref_value, tokens)
    }
}
