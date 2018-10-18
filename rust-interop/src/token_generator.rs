use std::fmt;

use proc_macro2;
use quote::ToTokens;


pub struct TokenGenerator<T> {
    value: T,
    function: fn(T, &mut proc_macro2::TokenStream),
}

impl<T> TokenGenerator<T> {
    pub fn new(value: T, function: fn(T, &mut proc_macro2::TokenStream)) -> Self {
        Self { value, function }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> fmt::Debug for TokenGenerator<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TokenGenerator")
            .field("ref_value", &self.value)
            .finish()
    }
}

impl<T> ToTokens for TokenGenerator<T>
where
    T: Clone,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        (self.function)(self.value.clone(), tokens)
    }
}
