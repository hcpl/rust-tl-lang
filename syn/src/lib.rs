//! A parsing library for parsing a string of TL language schema into a syntax tree of TL language
//! source text.

#[macro_use]
extern crate nom;


#[macro_use]
mod parsers;
mod utils;

pub mod print;
pub mod punctuated;
pub mod span;
pub mod spanned;
pub mod synom;
#[macro_use]
pub mod token;


mod bit;
pub use bit::BitIndex;

mod comment;
pub use comment::{Comment, CommentMultiLine, CommentSingleLine};

mod file;
pub use file::File;

mod id;
pub use id::Id;

mod ident;
pub use ident::Ident;

mod item;
pub use item::{
    BitSelector, CombinatorId, ConditionalParamDef, Delimiter, DelimiterTypes, DelimiterFunctions,
    Item, ItemCombinator, ItemComment, ItemDelimiter, ItemLayer, Multiplicity, OptParam, Param,
    ParamConditional, ParamRepeated, ParamRepeatedIdent, ParamTypeOnly, ParamWithParen,
};

mod path;
pub use path::{
    AngleBracketedGenericArguments, GenericArguments, ParameterizedPath, Path,
    SafeParameterizedPath, SafeParameterizedPathSpaceImmune, SafeParameterizedPathParenthesized,
    SpaceSeparatedGenericArguments,
};

mod ty;
pub use ty::{Type, TypeInt, TypeParameterizedPath, TypeTypeParameter};


use synom::{Parser, Synom};

/// Parse a string of TL language schema into the chosen syntax tree node.
pub fn parse_str<T: Synom>(s: &str) -> Result<T, nom::Err<&str>> {
    let parser = T::parse_str;
    parser.parse_str(s)
}

/// Parse the content of a file of TL language schema.
pub fn parse_file(mut content: &str) -> Result<File, nom::Err<&str>> {
    // Strip the BOM if it is present
    const BOM: &str = "\u{feff}";
    if content.starts_with(BOM) {
        content = &content[BOM.len()..];
    }

    parse_str(content)
}
