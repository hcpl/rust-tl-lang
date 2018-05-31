//! A parsing library for parsing a string of TL language schema into a syntax tree of TL language
//! source text.

#[cfg(feature = "parsing")]
#[macro_use]
extern crate nom;


#[cfg(feature = "parsing")]
#[macro_use]
mod parsers;
#[cfg_attr(test, macro_use)]
mod utils;

#[cfg(feature = "parsing")]
pub mod cursor;
#[cfg(feature = "printing")]
pub mod print;
pub mod punctuated;
pub mod span;
pub mod spanned;
#[cfg(feature = "parsing")]
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


#[cfg(feature = "parsing")]
mod convenience_parsing_functions {
    use std::fs::File as FsFile;
    use std::io::{self, Read};
    use std::path::Path as FsPath;

    use super::*;
    use synom::{Parser, Synom};

    /// Parse a string of TL language schema into the chosen syntax tree node.
    pub fn parse_str<T: Synom>(s: &str) -> Result<T, nom::Err<&str>> {
        let parser = T::parse_cursor;
        parser.parse_str(s)
    }

    /// Parse the content of a file of TL language schema.
    pub fn parse_file_str(mut content: &str) -> Result<File, nom::Err<&str>> {
        // Strip the BOM if it is present
        const BOM: &str = "\u{feff}";
        if content.starts_with(BOM) {
            content = &content[BOM.len()..];
        }

        parse_str(content)
    }

    // FIXME: load&parse in a streaming fashion?
    /// Load the content of the entire file into the memory and parse it.
    pub fn parse_file<P: AsRef<FsPath>>(path: P) -> io::Result<File> {
        let mut file = FsFile::open(path)?;

        let initial_buffer_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
        let mut content = String::with_capacity(initial_buffer_size);
        file.read_to_string(&mut content)?;

        parse_file_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, nom_err_to_owned(e)))
    }

    fn nom_err_to_owned<I, E>(error: nom::Err<&I, E>) -> nom::Err<I::Owned, E>
    where
        I: ToOwned + ?Sized,
    {
        match error {
            nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
            nom::Err::Error(context)     => nom::Err::Error(nom_context_to_owned(context)),
            nom::Err::Failure(context)   => nom::Err::Failure(nom_context_to_owned(context)),
        }
    }

    fn nom_context_to_owned<I, E>(context: nom::Context<&I, E>) -> nom::Context<I::Owned, E>
    where
        I: ToOwned + ?Sized,
    {
        match context {
            nom::Context::Code(input, kind) => nom::Context::Code(input.to_owned(), kind),
        }
    }
}

#[cfg(feature = "parsing")]
pub use convenience_parsing_functions::*;
