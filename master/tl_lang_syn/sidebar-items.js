initSidebarItems({"enum":[["Comment","A single-line or multiline comment."],["Delimiter","Divides sections of declarations of TL combinators."],["GenericArguments","Generic arguments for parameterized paths."],["Item","Top-level entities in TL schema that occupy whole lines."],["Param","A required field declaration."],["SafeParameterizedPath","A dot-separated list of identifiers with optional generic arguments that spans over a single token tree group."],["Type","The possible types that can appear in TL declarations."]],"fn":[["parse_file","Load the content of the entire file into the memory and parse it."],["parse_file_str","Parse the content of a file of TL language schema."],["parse_str","Parse a string of TL language schema into the chosen syntax tree node."]],"mod":[["cursor","A cheaply copyable cursor into a `&str` supporting efiicient traversal."],["error",""],["print","Common printing facility for syntax tree nodes."],["punctuated","A punctuated sequence of syntax tree nodes separated by punctuation."],["span","A region of source text."],["spanned","A trait that can provide the `Span` of the complete contents of a syntax tree node."],["synom","Parsing interface for parsing a token stream into a syntax tree node."],["token","Tokens representing TL language punctuation, keywords, and delimiters."]],"struct":[["AngleBracketedGenericArguments","A comma-separated list of generic arguments enclosed in angle tokens."],["BitIndex","An index pointing to the n-th bit of a `#` value (or, an `u32` value)."],["BitSelector","Selects a bit from a `#` parameter."],["CombinatorId","A TL combinator id: `#1cb5c415`."],["CommentMultiLine","A `/*...*/` comment spanning multiple lines."],["CommentSingleLine","A `//...` comment spanning a single line."],["ConditionalParamDef",""],["DelimiterFunctions","A `---functions---` delimiter."],["DelimiterTypes","A `---types---` delimiter."],["File","A complete file of TL language source text."],["Id","A 32-bit number which identifies a TL combinator."],["Ident","An identifier: `channels`, `SendMessageAction`, `X`, etc.."],["ItemCombinator","A TL combinator item: `inputMediaPhoto#8f2ab2ec id:InputPhoto = InputMedia;`."],["ItemComment","A comment item."],["ItemDelimiter","A delimiter item."],["ItemLayer","A layer item: `// LAYER 78`."],["Multiplicity",""],["OptParam","An optional field declaration: `{X:Type}`."],["ParamConditional","A possibly conditional field: `bg_color:int`, `report_spam:flags.0?true`."],["ParamRepeated",""],["ParamRepeatedIdent",""],["ParamTypeOnly","A field with a bare type."],["ParamWithParen","A declaration enclosed in parentheses that may have multiple fields."],["ParameterizedPath","A dot-separated list of identifiers with optional generic arguments."],["Path","A dot-separated list of identifiers."],["SafeParameterizedPathParenthesized","An arbitrary parameterized path enclosed in parentheses."],["SafeParameterizedPathSpaceImmune","A parameterized path that spans a single token tree group."],["SpaceSeparatedGenericArguments","A space-separated list of generic arguments."],["TypeBare","A bare type parameter: `%(Tuple X n)`."],["TypeInt","A special type of integers in range from 0 to 2^31-1 inclusive: `#`."],["TypeParameterizedPath","A type represented by a safe parameterized path: `contacts.Link`, `messages.Chats`."],["TypeTypeParameter","A type parameter: `!X`."]]});