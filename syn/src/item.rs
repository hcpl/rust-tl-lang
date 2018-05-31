use super::{BitIndex, Comment, Id, Ident, ParameterizedPath, Path, Type};
use span::Span;
use token::{Brace, Bracket, Paren, SlashSlash};


/// Top-level entities in TL schema that occupy whole lines.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub enum Item {
    Combinator(ItemCombinator),
    Delimiter(ItemDelimiter),
    Layer(ItemLayer),
    Comment(ItemComment),
}

/// A TL combinator item: `inputMediaPhoto#8f2ab2ec id:InputPhoto = InputMedia;`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ItemCombinator {
    pub name: Path,
    pub combinator_id: Option<CombinatorId>,
    pub opt_params: Vec<OptParam>,
    pub params: Vec<Param>,
    pub equals_token: TLToken![=],
    pub result_type: ParameterizedPath,
    pub semicolon_token: TLToken![;],
}

/// A TL combinator id: `#1cb5c415`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct CombinatorId {
    pub hash_token: TLToken![#],
    pub id: Id,
}

/// An optional field declaration: `{X:Type}`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct OptParam {
    pub brace_token: Brace,
    pub var_idents: Vec<Ident>,
    pub colon_token: TLToken![:],
    pub ty: Type,
}

/// A required field declaration.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub enum Param {
    Conditional(ParamConditional),
    Repeated(ParamRepeated),
    WithParen(ParamWithParen),
    TypeOnly(ParamTypeOnly),
}

/// A possibly conditional field: `bg_color:int`, `report_spam:flags.0?true`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ParamConditional {
    pub var_ident: Ident,
    pub colon_token: TLToken![:],
    pub conditional_param_def: Option<ConditionalParamDef>,
    pub ty: Type,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ConditionalParamDef {
    pub var_ident: Ident,
    pub bit_selector: Option<BitSelector>,
    pub question_token: TLToken![?],
}

/// Selects a bit from a `#` parameter.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct BitSelector {
    pub dot_token: TLToken![.],
    pub bit_index: BitIndex,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ParamRepeated {
    pub param_repeated_ident: Option<ParamRepeatedIdent>,
    pub multiplicity: Option<Multiplicity>,
    pub bracket_token: Bracket,
    pub params: Vec<Param>,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ParamRepeatedIdent {
    pub var_ident: Ident,
    pub colon_token: TLToken![:],
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct Multiplicity {
    pub term: Ident,  // FIXME: actually, it can be any term here
    pub asterisk_token: TLToken![*],
}

/// A declaration enclosed in parentheses that may have multiple fields.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ParamWithParen {
    pub paren_token: Paren,
    pub var_idents: Vec<Ident>,
    pub colon_token: TLToken![:],
    pub ty: Type,
}

/// A field with a bare type.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ParamTypeOnly {
    pub ty: Type,
}

/// A delimiter item.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ItemDelimiter {
    pub delimiter: Delimiter,
}

/// Divides sections of declarations of TL combinators.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub enum Delimiter {
    Types(DelimiterTypes),
    Functions(DelimiterFunctions),
}

/// A `---types---` delimiter.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct DelimiterTypes {
    pub span: Span,
}

/// A `---functions---` delimiter.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct DelimiterFunctions {
    pub span: Span,
}

/// A layer item: `// LAYER 78`.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct ItemLayer {
    pub slash_slash_token: SlashSlash,
    pub layer_token: TLToken![LAYER],
    pub layer_span: Span,
    pub layer: u32,
}

/// A comment item.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
#[cfg_attr(feature = "hash-impls", derive(Hash))]
pub struct ItemComment {
    pub comment: Comment,
}


#[cfg(feature = "eq-impls")]
mod eq_impls {
    use super::*;

    impl Eq for DelimiterTypes {}
    impl Eq for DelimiterFunctions {}
    impl Eq for ItemLayer {}

    impl PartialEq for DelimiterTypes {
        fn eq(&self, _other: &DelimiterTypes) -> bool {
            true
        }
    }

    impl PartialEq for DelimiterFunctions {
        fn eq(&self, _other: &DelimiterFunctions) -> bool {
            true
        }
    }

    impl PartialEq for ItemLayer {
        fn eq(&self, other: &ItemLayer) -> bool {
            self.layer == other.layer
        }
    }
}


#[cfg(feature = "hash-impls")]
mod hash_impls {
    use std::hash::{Hash, Hasher};

    use super::*;

    impl Hash for DelimiterTypes {
        fn hash<H: Hasher>(&self, _state: &mut H) {
            // No state to hash -- do nothing
        }
    }

    impl Hash for DelimiterFunctions {
        fn hash<H: Hasher>(&self, _state: &mut H) {
            // No state to hash -- do nothing
        }
    }

    impl Hash for ItemLayer {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.layer.hash(state);
        }
    }
}


mod spanned {
    use super::*;
    use span::Span;
    use spanned::Spanned;
    use spanned::private::Sealed;

    impl Sealed for  Item {}
    impl Sealed for  ItemCombinator {}
    impl Sealed for  CombinatorId {}
    impl Sealed for  OptParam {}
    impl Sealed for  Param {}
    impl Sealed for  ParamConditional {}
    impl Sealed for  ConditionalParamDef {}
    impl Sealed for  BitSelector {}
    impl Sealed for  ParamRepeated {}
    impl Sealed for  ParamRepeatedIdent {}
    impl Sealed for  Multiplicity {}
    impl Sealed for  ParamWithParen {}
    impl Sealed for  ParamTypeOnly {}
    impl Sealed for  ItemDelimiter {}
    impl Sealed for  Delimiter {}
    impl Sealed for  DelimiterTypes {}
    impl Sealed for  DelimiterFunctions {}
    impl Sealed for  ItemLayer {}
    impl Sealed for  ItemComment {}

    impl Spanned for Item {
        fn span(&self) -> Span {
            match *self {
                Item::Combinator(ref t) => t.span(),
                Item::Delimiter(ref t) => t.span(),
                Item::Layer(ref t) => t.span(),
                Item::Comment(ref t) => t.span(),
            }
        }
    }

    impl Spanned for ItemCombinator {
        fn span(&self) -> Span {
            self.name.span()
                .to(self.combinator_id.span())
                .to(self.opt_params.span())
                .to(self.params.span())
                .to(self.equals_token.span())
                .to(self.result_type.span())
                .to(self.semicolon_token.span())
        }
    }

    impl Spanned for CombinatorId {
        fn span(&self) -> Span {
            self.hash_token.span()
                .to(self.id.span())
        }
    }

    impl Spanned for OptParam {
        fn span(&self) -> Span {
            self.brace_token.span()
                .to(self.var_idents.span())
                .to(self.colon_token.span())
                .to(self.ty.span())
        }
    }

    impl Spanned for Param {
        fn span(&self) -> Span {
            match *self {
                Param::Conditional(ref t) => t.span(),
                Param::Repeated(ref t) => t.span(),
                Param::WithParen(ref t) => t.span(),
                Param::TypeOnly(ref t) => t.span(),
            }
        }
    }

    impl Spanned for ParamConditional {
        fn span(&self) -> Span {
            self.var_ident.span()
                .to(self.colon_token.span())
                .to(self.conditional_param_def.span())
                .to(self.ty.span())
        }
    }

    impl Spanned for ConditionalParamDef {
        fn span(&self) -> Span {
            self.var_ident.span()
                .to(self.bit_selector.span())
                .to(self.question_token.span())
        }
    }

    impl Spanned for BitSelector {
        fn span(&self) -> Span {
            self.dot_token.span()
                .to(self.bit_index.span())
        }
    }

    impl Spanned for ParamRepeated {
        fn span(&self) -> Span {
            self.param_repeated_ident.span()
                .to(self.multiplicity.span())
                .to(self.bracket_token.span())
                .to(self.params.span())
        }
    }

    impl Spanned for ParamRepeatedIdent {
        fn span(&self) -> Span {
            self.var_ident.span()
                .to(self.colon_token.span())
        }
    }

    impl Spanned for Multiplicity {
        fn span(&self) -> Span {
            self.term.span()
                .to(self.asterisk_token.span())
        }
    }

    impl Spanned for ParamWithParen {
        fn span(&self) -> Span {
            self.paren_token.span()
                .to(self.var_idents.span())
                .to(self.colon_token.span())
                .to(self.ty.span())
        }
    }

    impl Spanned for ParamTypeOnly {
        fn span(&self) -> Span {
            self.ty.span()
        }
    }

    impl Spanned for ItemDelimiter {
        fn span(&self) -> Span {
            self.delimiter.span()
        }
    }

    impl Spanned for Delimiter {
        fn span(&self) -> Span {
            match *self {
                Delimiter::Types(ref t) => t.span(),
                Delimiter::Functions(ref t) => t.span(),
            }
        }
    }

    impl Spanned for DelimiterTypes {
        fn span(&self) -> Span {
            self.span
        }
    }

    impl Spanned for DelimiterFunctions {
        fn span(&self) -> Span {
            self.span
        }
    }

    impl Spanned for ItemLayer {
        fn span(&self) -> Span {
            self.slash_slash_token.span()
                .to(self.layer_token.span())
                .to(self.layer_span)
        }
    }

    impl Spanned for ItemComment {
        fn span(&self) -> Span {
            self.comment.span()
        }
    }
}


#[cfg(feature = "parsing")]
mod parsing {
    use nom;

    use super::*;
    use cursor::Cursor;
    use synom::Synom;
    use synom::private::Sealed;
    use utils::parsing::is_decimal_digit;

    impl Sealed for  Item {}
    impl Sealed for  ItemCombinator {}
    impl Sealed for  CombinatorId {}
    impl Sealed for  OptParam {}
    impl Sealed for  Param {}
    impl Sealed for  ParamConditional {}
    impl Sealed for  ConditionalParamDef {}
    impl Sealed for  BitSelector {}
    impl Sealed for  ParamRepeated {}
    impl Sealed for  ParamRepeatedIdent {}
    impl Sealed for  Multiplicity {}
    impl Sealed for  ParamWithParen {}
    impl Sealed for  ParamTypeOnly {}
    impl Sealed for  ItemDelimiter {}
    impl Sealed for  Delimiter {}
    impl Sealed for  DelimiterTypes {}
    impl Sealed for  DelimiterFunctions {}
    impl Sealed for  ItemLayer {}
    impl Sealed for  ItemComment {}

    impl Synom for Item {
        named!(parse_cursor(Cursor) -> Item, alt_complete!(
            tlsyn!(ItemCombinator) => { Item::Combinator }
            |
            tlsyn!(ItemDelimiter) => { Item::Delimiter }
            |
            tlsyn!(ItemLayer) => { Item::Layer }
            |
            tlsyn!(ItemComment) => { Item::Comment }
        ));
    }

    impl Synom for ItemCombinator {
        named!(parse_cursor(Cursor) -> ItemCombinator, do_parse!(
            call!(nom::space0) >>
            name: tlsyn!(Path) >>
            combinator_id: opt!(tlsyn!(CombinatorId)) >>
            call!(nom::space0) >>
            opt_params: many0!(with_afterspace!(tlsyn!(OptParam))) >>
            call!(nom::space0) >>
            params: many0!(with_afterspace!(tlsyn!(Param))) >>
            call!(nom::space0) >>
            equals_token: tlpunct!(=) >>
            call!(nom::space0) >>
            result_type: tlsyn!(ParameterizedPath) >>
            call!(nom::space0) >>
            semicolon_token: tlpunct!(;) >>
            call!(nom::space0) >>

            (ItemCombinator {
                name, combinator_id, opt_params, params, equals_token, result_type, semicolon_token,
            })
        ));
    }

    impl Synom for CombinatorId {
        named!(parse_cursor(Cursor) -> CombinatorId, do_parse!(
            hash_token: tlpunct!(#) >>
            id: tlsyn!(Id) >>
            (CombinatorId { hash_token, id })
        ));
    }

    impl Synom for OptParam {
        named!(parse_cursor(Cursor) -> OptParam, do_parse!(
            opt_param: braces!(do_parse!(
                call!(nom::space0) >>
                var_idents: many1!(with_afterspace!(tlsyn!(Ident))) >>
                call!(nom::space0) >>
                colon_token: tlpunct!(:) >>
                call!(nom::space0) >>
                ty: tlsyn!(Type) >>
                call!(nom::space0) >>

                (var_idents, colon_token, ty)
            )) >>

            (OptParam {
                brace_token: opt_param.0,
                var_idents: (opt_param.1).0,
                colon_token: (opt_param.1).1,
                ty: (opt_param.1).2,
            })
        ));
    }

    impl Synom for Param {
        named!(parse_cursor(Cursor) -> Param, alt_complete!(
            tlsyn!(ParamConditional) => { Param::Conditional }
            |
            tlsyn!(ParamRepeated) => { Param::Repeated }
            |
            tlsyn!(ParamWithParen) => { Param::WithParen }
            |
            tlsyn!(ParamTypeOnly) => { Param::TypeOnly }
        ));
    }

    impl Synom for ParamConditional {
        named!(parse_cursor(Cursor) -> ParamConditional, do_parse!(
            var_ident: tlsyn!(Ident) >>
            colon_token: tlpunct!(:) >>
            conditional_param_def: opt!(tlsyn!(ConditionalParamDef)) >>
            ty: tlsyn!(Type) >>

            (ParamConditional { var_ident, colon_token, conditional_param_def, ty })
        ));
    }

    impl Synom for ConditionalParamDef {
        named!(parse_cursor(Cursor) -> ConditionalParamDef, do_parse!(
            var_ident: tlsyn!(Ident) >>
            bit_selector: opt!(tlsyn!(BitSelector)) >>
            question_token: tlpunct!(?) >>

            (ConditionalParamDef { var_ident, bit_selector, question_token })
        ));
    }

    impl Synom for BitSelector {
        named!(parse_cursor(Cursor) -> BitSelector, do_parse!(
            dot_token: tlpunct!(.) >>
            bit_index: tlsyn!(BitIndex) >>

            (BitSelector { dot_token, bit_index })
        ));
    }

    impl Synom for ParamRepeated {
        named!(parse_cursor(Cursor) -> ParamRepeated, do_parse!(
            param_repeated_ident: opt!(tlsyn!(ParamRepeatedIdent)) >>
            multiplicity: opt!(tlsyn!(Multiplicity)) >>
            params: brackets!(many0!(with_afterspace!(tlsyn!(Param)))) >>

            (ParamRepeated {
                param_repeated_ident,
                multiplicity,
                bracket_token: params.0,
                params: params.1,
            })
        ));
    }

    impl Synom for ParamRepeatedIdent {
        named!(parse_cursor(Cursor) -> ParamRepeatedIdent, do_parse!(
            var_ident: tlsyn!(Ident) >>
            colon_token: tlpunct!(:) >>

            (ParamRepeatedIdent { var_ident, colon_token })
        ));
    }

    impl Synom for Multiplicity {
        named!(parse_cursor(Cursor) -> Multiplicity, do_parse!(
            term: tlsyn!(Ident) >>
            asterisk_token: tlpunct!(*) >>

            (Multiplicity { term, asterisk_token })
        ));
    }

    impl Synom for ParamWithParen {
        named!(parse_cursor(Cursor) -> ParamWithParen, do_parse!(
            param: parens!(do_parse!(
                call!(nom::space0) >>
                var_idents: many1!(with_afterspace!(tlsyn!(Ident))) >>
                call!(nom::space0) >>
                colon_token: tlpunct!(:) >>
                call!(nom::space0) >>
                ty: tlsyn!(Type) >>
                call!(nom::space0) >>

                (var_idents, colon_token, ty)
            )) >>

            (ParamWithParen {
                paren_token: param.0,
                var_idents: (param.1).0,
                colon_token: (param.1).1,
                ty: (param.1).2,
            })
        ));
    }

    impl Synom for ParamTypeOnly {
        named!(parse_cursor(Cursor) -> ParamTypeOnly, do_parse!(
            ty: tlsyn!(Type) >>

            (ParamTypeOnly { ty })
        ));
    }

    impl Synom for ItemDelimiter {
        named!(parse_cursor(Cursor) -> ItemDelimiter, do_parse!(
            delimiter: tlsyn!(Delimiter) >>
            (ItemDelimiter { delimiter })
        ));
    }

    impl Synom for Delimiter {
        named!(parse_cursor(Cursor) -> Delimiter, alt_complete!(
            tlsyn!(DelimiterTypes) => { Delimiter::Types }
            |
            tlsyn!(DelimiterFunctions) => { Delimiter::Functions }
        ));
    }

    impl Synom for DelimiterTypes {
        named!(parse_cursor(Cursor) -> DelimiterTypes, do_parse!(
            types_cursor: tag!("---types---") >>
            span: value!(types_cursor.span()) >>

            (DelimiterTypes { span })
        ));
    }

    impl Synom for DelimiterFunctions {
        named!(parse_cursor(Cursor) -> DelimiterFunctions, do_parse!(
            functions_cursor: tag!("---functions---") >>
            span: value!(functions_cursor.span()) >>

            (DelimiterFunctions { span })
        ));
    }

    impl Synom for ItemLayer {
        named!(parse_cursor(Cursor) -> ItemLayer, do_parse!(
            call!(nom::space0) >>
            slash_slash_token: tlsyn!(SlashSlash) >>
            call!(nom::space0) >>
            layer_token: tlkeyword!(LAYER) >>
            call!(nom::space0) >>
            layer_cursor: take_while!(is_decimal_digit) >>
            layer_span: value!(layer_cursor.span()) >>
            layer: map_res!(value!(layer_cursor.to_str()), str::parse) >>
            call!(nom::space0) >>

            (ItemLayer { slash_slash_token, layer_token, layer_span, layer })
        ));
    }

    impl Synom for ItemComment {
        named!(parse_cursor(Cursor) -> ItemComment, do_parse!(
            comment: tlsyn!(Comment) >>
            (ItemComment { comment })
        ));
    }
}


#[cfg(feature = "printing")]
mod printing {
    use std::fmt;

    use super::*;
    use print::{Print, print_slice_with_separator};
    use print::private::Sealed;

    impl Sealed for  Item {}
    impl Sealed for  ItemCombinator {}
    impl Sealed for  CombinatorId {}
    impl Sealed for  OptParam {}
    impl Sealed for  Param {}
    impl Sealed for  ParamConditional {}
    impl Sealed for  ConditionalParamDef {}
    impl Sealed for  BitSelector {}
    impl Sealed for  ParamRepeated {}
    impl Sealed for  ParamRepeatedIdent {}
    impl Sealed for  Multiplicity {}
    impl Sealed for  ParamWithParen {}
    impl Sealed for  ParamTypeOnly {}
    impl Sealed for  ItemDelimiter {}
    impl Sealed for  Delimiter {}
    impl Sealed for  DelimiterTypes {}
    impl Sealed for  DelimiterFunctions {}
    impl Sealed for  ItemLayer {}
    impl Sealed for  ItemComment {}

    impl Print for Item {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Item::Combinator(ref t) => t.print(f),
                Item::Layer(ref t) => t.print(f),
                Item::Delimiter(ref t) => t.print(f),
                Item::Comment(ref t) => t.print(f),
            }
        }
    }

    impl Print for ItemCombinator {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.name.print(f)?;
            self.combinator_id.print(f)?;
            f.write_str(" ")?;
            if let Some(()) = print_slice_with_separator(&self.opt_params, " ", f)? {
                f.write_str(" ")?;
            }
            if let Some(()) = print_slice_with_separator(&self.params, " ", f)? {
                f.write_str(" ")?;
            }
            self.equals_token.print(f)?;
            f.write_str(" ")?;
            self.result_type.print(f)?;
            self.semicolon_token.print(f)?;

            Ok(())
        }
    }

    impl Print for CombinatorId {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.hash_token.print(f)?;
            self.id.print(f)?;

            Ok(())
        }
    }

    impl Print for OptParam {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Brace::print(f, |f| {
                print_slice_with_separator(&self.var_idents, " ", f)?;
                self.colon_token.print(f)?;
                self.ty.print(f)?;
                Ok(())
            })
        }
    }

    impl Print for Param {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Param::Conditional(ref t) => t.print(f),
                Param::Repeated(ref t) => t.print(f),
                Param::WithParen(ref t) => t.print(f),
                Param::TypeOnly(ref t) => t.print(f),
            }
        }
    }

    impl Print for ParamConditional {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.var_ident.print(f)?;
            self.colon_token.print(f)?;
            self.conditional_param_def.print(f)?;
            self.ty.print(f)?;

            Ok(())
        }
    }

    impl Print for ConditionalParamDef {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.var_ident.print(f)?;
            self.bit_selector.print(f)?;
            self.question_token.print(f)?;

            Ok(())
        }
    }

    impl Print for BitSelector {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.dot_token.print(f)?;
            self.bit_index.print(f)?;

            Ok(())
        }
    }

    impl Print for ParamRepeated {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.param_repeated_ident.print(f)?;
            self.multiplicity.print(f)?;
            Bracket::print(f, |f| {
                print_slice_with_separator(&self.params, " ", f)?;
                Ok(())
            })?;

            Ok(())
        }
    }

    impl Print for ParamRepeatedIdent {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.var_ident.print(f)?;
            self.colon_token.print(f)?;

            Ok(())
        }
    }

    impl Print for Multiplicity {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.term.print(f)?;
            self.asterisk_token.print(f)?;

            Ok(())
        }
    }

    impl Print for ParamWithParen {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Paren::print(f, |f| {
                print_slice_with_separator(&self.var_idents, " ", f)?;
                self.colon_token.print(f)?;
                self.ty.print(f)?;
                Ok(())
            })?;

            Ok(())
        }
    }

    impl Print for ParamTypeOnly {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.ty.print(f)
        }
    }

    impl Print for ItemDelimiter {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.delimiter.print(f)
        }
    }

    impl Print for Delimiter {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Delimiter::Types(ref t) => t.print(f),
                Delimiter::Functions(ref t) => t.print(f),
            }
        }
    }

    impl Print for DelimiterTypes {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("---types---")
        }
    }

    impl Print for DelimiterFunctions {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("---functions---")
        }
    }

    impl Print for ItemLayer {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.slash_slash_token.print(f)?;
            f.write_str(" ")?;
            self.layer_token.print(f)?;
            f.write_str(" ")?;
            fmt::Display::fmt(&self.layer, f)?;

            Ok(())
        }
    }

    impl Print for ItemComment {
        fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.comment.print(f)
        }
    }
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "eq-impls")]
    use super::*;
    #[cfg(feature = "eq-impls")]
    use span::Span;
    #[cfg(feature = "eq-impls")]
    use utils::tests::test_span_permutations;


    #[cfg(feature = "eq-impls")]
    fn test_delimiter_types_span_permutations<FT, FA1, FA2>(
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FT: Fn(&DelimiterTypes, &DelimiterTypes) -> bool,
        FA1: Fn(&DelimiterTypes, &DelimiterTypes),
        FA2: Fn(&DelimiterTypes, &DelimiterTypes),
    {
        test_span_permutations(
            |span1| DelimiterTypes { span: span1 },
            |span2| DelimiterTypes { span: span2 },
            &test_eq,
            &assert_when_eq,
            &assert_when_ne,
        );
    }

    #[cfg(feature = "eq-impls")]
    fn test_delimiter_functions_span_permutations<FT, FA1, FA2>(
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FT: Fn(&DelimiterFunctions, &DelimiterFunctions) -> bool,
        FA1: Fn(&DelimiterFunctions, &DelimiterFunctions),
        FA2: Fn(&DelimiterFunctions, &DelimiterFunctions),
    {
        test_span_permutations(
            |span1| DelimiterFunctions { span: span1 },
            |span2| DelimiterFunctions { span: span2 },
            &test_eq,
            &assert_when_eq,
            &assert_when_ne,
        );
    }

    #[cfg(feature = "eq-impls")]
    fn test_item_layer_span_permutations<FT, FA1, FA2>(
        test_eq: FT,
        assert_when_eq: FA1,
        assert_when_ne: FA2,
    )
    where
        FT: Fn(&ItemLayer, &ItemLayer) -> bool,
        FA1: Fn(&ItemLayer, &ItemLayer),
        FA2: Fn(&ItemLayer, &ItemLayer),
    {
        fn new_item_layer(span: Span, layer: u32) -> ItemLayer {
            ItemLayer {
                slash_slash_token: SlashSlash(span),
                layer_token: TLToken![LAYER](span),
                layer_span: span,
                layer,
            }
        }

        let layers = [1, 57, 68, 74, 78];

        for layer1 in &layers {
            for layer2 in &layers {
                test_span_permutations(
                    |span1| new_item_layer(span1, *layer1),
                    |span2| new_item_layer(span2, *layer2),
                    &test_eq,
                    &assert_when_eq,
                    &assert_when_ne,
                );
            }
        }
    }

    #[cfg(feature = "eq-impls")]
    mod eq_does_not_depend_on_span {
        use super::{
            test_delimiter_types_span_permutations,
            test_delimiter_functions_span_permutations,
            test_item_layer_span_permutations,
        };

        #[test]
        fn delimiter_types() {
            test_delimiter_types_span_permutations(
                |_, _| true,
                |x, y| any_debug_assert_eq!(x, y),
                |_, _| unreachable!("Stateless syntax tree nodes must all be equal to each other"),
            );
        }

        #[test]
        fn delimiter_functions() {
            test_delimiter_functions_span_permutations(
                |_, _| true,
                |x, y| any_debug_assert_eq!(x, y),
                |_, _| unreachable!("Stateless syntax tree nodes must all be equal to each other"),
            );
        }

        #[test]
        fn item_layer() {
            test_item_layer_span_permutations(
                |x, y| x.layer == y.layer,
                |x, y| any_debug_assert_eq!(x, y),
                |x, y| any_debug_assert_ne!(x, y),
            );
        }
    }

    #[cfg(all(feature = "eq-impls", feature = "hash-impls"))]
    mod eq_hash_property {
        use utils::tests::get_hasher_state;

        use super::{
            test_delimiter_types_span_permutations,
            test_delimiter_functions_span_permutations,
            test_item_layer_span_permutations,
        };

        #[test]
        fn delimiter_types() {
            test_delimiter_types_span_permutations(
                |x, y| x == y,
                |x, y| any_debug_assert_eq!(get_hasher_state(x), get_hasher_state(y)),
                |x, y| any_debug_assert_ne!(get_hasher_state(x), get_hasher_state(y)),
            );
        }

        #[test]
        fn delimiter_functions() {
            test_delimiter_functions_span_permutations(
                |x, y| x == y,
                |x, y| any_debug_assert_eq!(get_hasher_state(x), get_hasher_state(y)),
                |x, y| any_debug_assert_ne!(get_hasher_state(x), get_hasher_state(y)),
            );
        }

        #[test]
        fn item_layer() {
            test_item_layer_span_permutations(
                |x, y| x == y,
                |x, y| any_debug_assert_eq!(get_hasher_state(x), get_hasher_state(y)),
                |x, y| any_debug_assert_ne!(get_hasher_state(x), get_hasher_state(y)),
            );
        }
    }
}
