use proc_macro2;
use tl_lang_syn as tlsn;

use ::token_generator::TokenGenerator;


#[derive(Debug, Eq, PartialEq)]
pub struct Path(pub tlsn::ParameterizedPath);
