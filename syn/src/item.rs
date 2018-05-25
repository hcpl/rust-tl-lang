#[cfg(feature = "printing")]
use std::fmt;

use super::{BitIndex, Comment, Id, Ident, ParameterizedPath, Path, Type};
#[cfg(feature = "parsing")]
use cursor::Cursor;
#[cfg(feature = "printing")]
use print::{Print, print_slice_with_separator};
use span::Span;
use spanned::Spanned;
#[cfg(feature = "parsing")]
use synom::Synom;
use token::{Brace, Bracket, Paren, SlashSlash};
#[cfg(feature = "parsing")]
use utils::is_decimal_digit;


#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub enum Item {
    Combinator(ItemCombinator),
    Delimiter(ItemDelimiter),
    Layer(ItemLayer),
    Comment(ItemComment),
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ItemComment {
    pub comment: Comment,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ItemDelimiter {
    pub delimiter: Delimiter,
}

/// Divides sections of declarations of TL combinators.
#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
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

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ItemCombinator {
    pub name: Path,
    pub combinator_id: Option<CombinatorId>,
    pub opt_params: Vec<OptParam>,
    pub params: Vec<Param>,
    pub equals_token: TLToken![=],
    pub result_type: ParameterizedPath,
    pub semicolon_token: TLToken![;],
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct CombinatorId {
    pub hash_token: TLToken![#],
    pub id: Id,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct OptParam {
    pub brace_token: Brace,
    pub var_idents: Vec<Ident>,
    pub colon_token: TLToken![:],
    pub ty: Type,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub enum Param {
    Conditional(ParamConditional),
    Repeated(ParamRepeated),
    WithParen(ParamWithParen),
    TypeOnly(ParamTypeOnly),
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ParamConditional {
    pub var_ident: Ident,
    pub colon_token: TLToken![:],
    pub conditional_param_def: Option<ConditionalParamDef>,
    pub ty: Type,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ConditionalParamDef {
    pub var_ident: Ident,
    pub bit_selector: Option<BitSelector>,
    pub question_token: TLToken![?],
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct BitSelector {
    pub dot_token: TLToken![.],
    pub bit_index: BitIndex,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ParamRepeated {
    pub param_repeated_ident: Option<ParamRepeatedIdent>,
    pub multiplicity: Option<Multiplicity>,
    pub bracket_token: Bracket,
    pub params: Vec<Param>,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ParamRepeatedIdent {
    pub var_ident: Ident,
    pub colon_token: TLToken![:],
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct Multiplicity {
    pub term: Ident,  // FIXME: actually, it can be any term here
    pub asterisk_token: TLToken![*],
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ParamWithParen {
    pub paren_token: Paren,
    pub var_idents: Vec<Ident>,
    pub colon_token: TLToken![:],
    pub ty: Type,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[cfg_attr(feature = "eq-impls", derive(Eq, PartialEq))]
pub struct ParamTypeOnly {
    pub ty: Type,
}

#[cfg_attr(feature = "clone-impls", derive(Clone))]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct ItemLayer {
    pub slash_slash_token: SlashSlash,
    pub layer_token: TLToken![LAYER],
    pub layer_span: Span,
    pub layer: u32,
}


#[cfg(feature = "eq-impls")]
impl Eq for DelimiterTypes {}
#[cfg(feature = "eq-impls")]
impl Eq for DelimiterFunctions {}
#[cfg(feature = "eq-impls")]
impl Eq for ItemLayer {}

#[cfg(feature = "eq-impls")]
impl PartialEq for DelimiterTypes {
    fn eq(&self, _other: &DelimiterTypes) -> bool {
        true
    }
}

#[cfg(feature = "eq-impls")]
impl PartialEq for DelimiterFunctions {
    fn eq(&self, _other: &DelimiterFunctions) -> bool {
        true
    }
}


#[cfg(feature = "eq-impls")]
impl PartialEq for ItemLayer {
    fn eq(&self, other: &ItemLayer) -> bool {
        self.layer == other.layer
    }
}


#[cfg(feature = "parsing")]
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

#[cfg(feature = "parsing")]
impl Synom for ItemComment {
    named!(parse_cursor(Cursor) -> ItemComment, do_parse!(
        comment: tlsyn!(Comment) >>
        (ItemComment { comment })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ItemDelimiter {
    named!(parse_cursor(Cursor) -> ItemDelimiter, do_parse!(
        delimiter: tlsyn!(Delimiter) >>
        (ItemDelimiter { delimiter })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for Delimiter {
    named!(parse_cursor(Cursor) -> Delimiter, alt_complete!(
        tlsyn!(DelimiterTypes) => { Delimiter::Types }
        |
        tlsyn!(DelimiterFunctions) => { Delimiter::Functions }
    ));
}

#[cfg(feature = "parsing")]
impl Synom for DelimiterTypes {
    named!(parse_cursor(Cursor) -> DelimiterTypes, do_parse!(
        types_cursor: tag!("---types---") >>
        span: value!(types_cursor.span()) >>

        (DelimiterTypes { span })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for DelimiterFunctions {
    named!(parse_cursor(Cursor) -> DelimiterFunctions, do_parse!(
        functions_cursor: tag!("---functions---") >>
        span: value!(functions_cursor.span()) >>

        (DelimiterFunctions { span })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ItemCombinator {
    named!(parse_cursor(Cursor) -> ItemCombinator, sp!(do_parse!(
        name: tlsyn!(Path) >>
        combinator_id: opt!(tlsyn!(CombinatorId)) >>
        opt_params: sp!(many0!(tlsyn!(OptParam))) >>
        params: many0!(sp!(tlsyn!(Param))) >>
        equals_token: tlpunct!(=) >>
        result_type: tlsyn!(ParameterizedPath) >>
        semicolon_token: tlpunct!(;) >>

        (ItemCombinator {
            name, combinator_id, opt_params, params, equals_token, result_type, semicolon_token,
        })
    )));
}

#[cfg(feature = "parsing")]
impl Synom for CombinatorId {
    named!(parse_cursor(Cursor) -> CombinatorId, do_parse!(
        hash_token: tlpunct!(#) >>
        id: tlsyn!(Id) >>
        (CombinatorId { hash_token, id })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for OptParam {
    named!(parse_cursor(Cursor) -> OptParam, do_parse!(
        opt_param: braces!(tuple!(
            sp!(many1!(tlsyn!(Ident))),
            tlpunct!(:),
            tlsyn!(Type)
        )) >>

        (OptParam {
            brace_token: opt_param.0,
            var_idents: (opt_param.1).0,
            colon_token: (opt_param.1).1,
            ty: (opt_param.1).2,
        })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for Param {
    named!(parse_cursor(Cursor) -> Param , alt_complete!(
        tlsyn!(ParamConditional) => { Param::Conditional }
        |
        tlsyn!(ParamRepeated) => { Param::Repeated }
        |
        tlsyn!(ParamWithParen) => { Param::WithParen }
        |
        tlsyn!(ParamTypeOnly) => { Param::TypeOnly }
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ParamConditional {
    named!(parse_cursor(Cursor) -> ParamConditional, do_parse!(
        var_ident: tlsyn!(Ident) >>
        colon_token: tlpunct!(:) >>
        conditional_param_def: opt!(tlsyn!(ConditionalParamDef)) >>
        ty: tlsyn!(Type) >>

        (ParamConditional { var_ident, colon_token, conditional_param_def, ty })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ConditionalParamDef {
    named!(parse_cursor(Cursor) -> ConditionalParamDef, do_parse!(
        var_ident: tlsyn!(Ident) >>
        bit_selector: opt!(tlsyn!(BitSelector)) >>
        question_token: tlpunct!(?) >>

        (ConditionalParamDef { var_ident, bit_selector, question_token })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for BitSelector {
    named!(parse_cursor(Cursor) -> BitSelector, do_parse!(
        dot_token: tlpunct!(.) >>
        bit_index: tlsyn!(BitIndex) >>

        (BitSelector { dot_token, bit_index })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ParamRepeated {
    named!(parse_cursor(Cursor) -> ParamRepeated, do_parse!(
        param_repeated_ident: opt!(tlsyn!(ParamRepeatedIdent)) >>
        multiplicity: opt!(tlsyn!(Multiplicity)) >>
        params: brackets!(sp!(many0!(tlsyn!(Param)))) >>

        (ParamRepeated {
            param_repeated_ident,
            multiplicity,
            bracket_token: params.0,
            params: params.1,
        })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ParamRepeatedIdent {
    named!(parse_cursor(Cursor) -> ParamRepeatedIdent, do_parse!(
        var_ident: tlsyn!(Ident) >>
        colon_token: tlpunct!(:) >>

        (ParamRepeatedIdent { var_ident, colon_token })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for Multiplicity {
    named!(parse_cursor(Cursor) -> Multiplicity, do_parse!(
        term: tlsyn!(Ident) >>
        asterisk_token: tlpunct!(*) >>

        (Multiplicity { term, asterisk_token })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ParamWithParen {
    named!(parse_cursor(Cursor) -> ParamWithParen, do_parse!(
        param: parens!(tuple!(
            sp!(many1!(tlsyn!(Ident))),
            tlpunct!(:),
            tlsyn!(Type)
        )) >>

        (ParamWithParen {
            paren_token: param.0,
            var_idents: (param.1).0,
            colon_token: (param.1).1,
            ty: (param.1).2,
        })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ParamTypeOnly {
    named!(parse_cursor(Cursor) -> ParamTypeOnly, do_parse!(
        ty: tlsyn!(Type) >>

        (ParamTypeOnly { ty })
    ));
}

#[cfg(feature = "parsing")]
impl Synom for ItemLayer {
    named!(parse_cursor(Cursor) -> ItemLayer, sp!(do_parse!(
        slash_slash_token: tlsyn!(SlashSlash) >>
        layer_token: tlkeyword!(LAYER) >>
        layer_cursor: take_while!(is_decimal_digit) >>
        layer_span: value!(layer_cursor.span()) >>
        layer: map_res!(value!(layer_cursor.to_str()), str::parse) >>

        (ItemLayer { slash_slash_token, layer_token, layer_span, layer })
    )));
}


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

impl Spanned for ItemComment {
    fn span(&self) -> Span {
        self.comment.span()
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

impl Spanned for ItemLayer {
    fn span(&self) -> Span {
        self.slash_slash_token.span()
            .to(self.layer_token.span())
            .to(self.layer_span)
    }
}


#[cfg(feature = "printing")]
impl Print for Item {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Item::Comment(ref t) => t.print(f),
            Item::Delimiter(ref t) => t.print(f),
            Item::Combinator(ref t) => t.print(f),
            Item::Layer(ref t) => t.print(f),
        }
    }
}

#[cfg(feature = "printing")]
impl Print for ItemComment {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.comment.print(f)
    }
}

#[cfg(feature = "printing")]
impl Print for ItemDelimiter {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.delimiter.print(f)
    }
}

#[cfg(feature = "printing")]
impl Print for Delimiter {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Delimiter::Types(ref t) => t.print(f),
            Delimiter::Functions(ref t) => t.print(f),
        }
    }
}

#[cfg(feature = "printing")]
impl Print for DelimiterTypes {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("---types---")
    }
}

#[cfg(feature = "printing")]
impl Print for DelimiterFunctions {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("---functions---")
    }
}

#[cfg(feature = "printing")]
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

#[cfg(feature = "printing")]
impl Print for CombinatorId {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.hash_token.print(f)?;
        self.id.print(f)?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
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

#[cfg(feature = "printing")]
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

#[cfg(feature = "printing")]
impl Print for ParamConditional {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.var_ident.print(f)?;
        self.colon_token.print(f)?;
        self.conditional_param_def.print(f)?;
        self.ty.print(f)?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
impl Print for ConditionalParamDef {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.var_ident.print(f)?;
        self.bit_selector.print(f)?;
        self.question_token.print(f)?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
impl Print for BitSelector {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.dot_token.print(f)?;
        self.bit_index.print(f)?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
impl Print for ParamRepeated {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.param_repeated_ident.print(f)?;
        self.multiplicity.print(f)?;
        Brace::print(f, |f| {
            print_slice_with_separator(&self.params, " ", f)?;
            Ok(())
        })?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
impl Print for ParamRepeatedIdent {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.var_ident.print(f)?;
        self.colon_token.print(f)?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
impl Print for Multiplicity {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.term.print(f)?;
        self.asterisk_token.print(f)?;

        Ok(())
    }
}

#[cfg(feature = "printing")]
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

#[cfg(feature = "printing")]
impl Print for ParamTypeOnly {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ty.print(f)
    }
}

#[cfg(feature = "printing")]
impl Print for ItemLayer {
    fn print(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("//")?;
        f.write_str(" ")?;
        f.write_str("LAYER")?;
        fmt::Display::fmt(&self.layer, f)?;

        Ok(())
    }
}
