use super::{BitIndex, Comment, Id, Ident, ParameterizedPath, Path, Type};
use synom::Synom;
use token::{Brace, Bracket, Paren};
use utils::is_decimal_digit;


#[derive(Debug)]
pub enum Item {
    Comment(ItemComment),
    Delimiter(ItemDelimiter),
    Combinator(ItemCombinator),
    Layer(ItemLayer),
}

#[derive(Debug)]
pub struct ItemComment {
    pub comment: Comment,
}

#[derive(Debug)]
pub struct ItemDelimiter {
    pub delimiter: Delimiter,
}

/// Divides sections of declarations of TL combinators.
#[derive(Debug)]
pub enum Delimiter {
    Types(DelimiterTypes),
    Functions(DelimiterFunctions),
}

/// A `---types---` delimiter.
#[derive(Debug)]
pub struct DelimiterTypes;

/// A `---functions---` delimiter.
#[derive(Debug)]
pub struct DelimiterFunctions;

#[derive(Debug)]
pub struct ItemCombinator {
    pub name: Path,
    pub combinator_id: Option<CombinatorId>,
    pub opt_params: Vec<OptParam>,
    pub params: Vec<Param>,
    pub equals_token: TLToken![=],
    pub result_type: ParameterizedPath,
    pub semicolon_token: TLToken![;],
}

#[derive(Debug)]
pub struct CombinatorId {
    pub hash_token: TLToken![#],
    pub id: Id,
}

#[derive(Debug)]
pub struct OptParam {
    pub brace_token: Brace,
    pub var_idents: Vec<Ident>,
    pub colon_token: TLToken![:],
    pub ty: Type,
}

#[derive(Debug)]
pub enum Param {
    Conditional(ParamConditional),
    Repeated(ParamRepeated),
    WithParen(ParamWithParen),
    TypeOnly(ParamTypeOnly),
}

#[derive(Debug)]
pub struct ParamConditional {
    pub var_ident: Ident,
    pub colon_token: TLToken![:],
    pub conditional_param_def: Option<ConditionalParamDef>,
    pub ty: Type,
}

#[derive(Debug)]
pub struct ConditionalParamDef {
    pub var_ident: Ident,
    pub bit_selector: Option<BitSelector>,
    pub question_token: TLToken![?],
}

#[derive(Debug)]
pub struct BitSelector {
    pub dot_token: TLToken![.],
    pub bit_index: BitIndex,
}

#[derive(Debug)]
pub struct ParamRepeated {
    pub param_repeated_ident: Option<ParamRepeatedIdent>,
    pub multiplicity: Option<Multiplicity>,
    pub bracket_token: Bracket,
    pub params: Vec<Param>,
}

#[derive(Debug)]
pub struct ParamRepeatedIdent {
    pub var_ident: Ident,
    pub colon_token: TLToken![:],
}

#[derive(Debug)]
pub struct Multiplicity {
    pub term: Ident,  // FIXME: actually, it can be any term here
    pub asterisk_token: TLToken![*],
}

#[derive(Debug)]
pub struct ParamWithParen {
    pub paren_token: Paren,
    pub var_idents: Vec<Ident>,
    pub colon_token: TLToken![:],
    pub ty: Type,
}

#[derive(Debug)]
pub struct ParamTypeOnly {
    pub ty: Type,
}

#[derive(Debug)]
pub struct ItemLayer {
    pub layer: u32,
}


impl Synom for Item {
    named!(parse_str(&str) -> Item, alt_complete!(
        tlsyn!(ItemComment) => { Item::Comment }
        |
        tlsyn!(ItemDelimiter) => { Item::Delimiter }
        |
        tlsyn!(ItemCombinator) => { Item::Combinator }
        |
        tlsyn!(ItemLayer) => { Item::Layer }
    ));
}

impl Synom for ItemComment {
    named!(parse_str(&str) -> ItemComment, do_parse!(
        comment: tlsyn!(Comment) >>
        (ItemComment { comment })
    ));
}

impl Synom for ItemDelimiter {
    named!(parse_str(&str) -> ItemDelimiter, do_parse!(
        delimiter: tlsyn!(Delimiter) >>
        (ItemDelimiter { delimiter })
    ));
}

impl Synom for Delimiter {
    named!(parse_str(&str) -> Delimiter, alt_complete!(
        tlsyn!(DelimiterTypes) => { Delimiter::Types }
        |
        tlsyn!(DelimiterFunctions) => { Delimiter::Functions }
    ));
}

// FIXME: Spanning
impl Synom for DelimiterTypes {
    named!(parse_str(&str) -> DelimiterTypes, do_parse!(
        tag!("---types---") >>
        (DelimiterTypes)
    ));
}

// FIXME: Spanning
impl Synom for DelimiterFunctions {
    named!(parse_str(&str) -> DelimiterFunctions, do_parse!(
        tag!("---functions---") >>
        (DelimiterFunctions)
    ));
}

impl Synom for ItemCombinator {
    named!(parse_str(&str) -> ItemCombinator, sp!(do_parse!(
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

impl Synom for CombinatorId {
    named!(parse_str(&str) -> CombinatorId, do_parse!(
        hash_token: tlpunct!(#) >>
        id: tlsyn!(Id) >>
        (CombinatorId { hash_token, id })
    ));
}

impl Synom for OptParam {
    named!(parse_str(&str) -> OptParam, do_parse!(
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

impl Synom for Param {
    named!(parse_str(&str) -> Param , alt_complete!(
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
    named!(parse_str(&str) -> ParamConditional, do_parse!(
        var_ident: tlsyn!(Ident) >>
        colon_token: tlpunct!(:) >>
        conditional_param_def: opt!(tlsyn!(ConditionalParamDef)) >>
        ty: tlsyn!(Type) >>

        (ParamConditional { var_ident, colon_token, conditional_param_def, ty })
    ));
}

impl Synom for ConditionalParamDef {
    named!(parse_str(&str) -> ConditionalParamDef, do_parse!(
        var_ident: tlsyn!(Ident) >>
        bit_selector: opt!(tlsyn!(BitSelector)) >>
        question_token: tlpunct!(?) >>

        (ConditionalParamDef { var_ident, bit_selector, question_token })
    ));
}

impl Synom for BitSelector {
    named!(parse_str(&str) -> BitSelector, do_parse!(
        dot_token: tlpunct!(.) >>
        bit_index: tlsyn!(BitIndex) >>

        (BitSelector { dot_token, bit_index })
    ));
}

impl Synom for ParamRepeated {
    named!(parse_str(&str) -> ParamRepeated, do_parse!(
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

impl Synom for ParamRepeatedIdent {
    named!(parse_str(&str) -> ParamRepeatedIdent, do_parse!(
        var_ident: tlsyn!(Ident) >>
        colon_token: tlpunct!(:) >>

        (ParamRepeatedIdent { var_ident, colon_token })
    ));
}

impl Synom for Multiplicity {
    named!(parse_str(&str) -> Multiplicity, do_parse!(
        term: tlsyn!(Ident) >>
        asterisk_token: tlpunct!(*) >>

        (Multiplicity { term, asterisk_token })
    ));
}

impl Synom for ParamWithParen {
    named!(parse_str(&str) -> ParamWithParen, do_parse!(
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

impl Synom for ParamTypeOnly {
    named!(parse_str(&str) -> ParamTypeOnly, do_parse!(
        ty: tlsyn!(Type) >>

        (ParamTypeOnly { ty })
    ));
}

// FIXME: Spanning
impl Synom for ItemLayer {
    named!(parse_str(&str) -> ItemLayer, sp!(do_parse!(
        tag!("//") >>
        tag!("LAYER") >>
        layer: map_res!(take_while!(is_decimal_digit), str::parse) >>

        (ItemLayer { layer })
    )));
}
