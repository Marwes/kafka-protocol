#[cfg(feature = "regenerate")]
mod regenerate {
    use std::{
        borrow::Cow,
        collections::BTreeSet,
        fmt, fs,
        io::{self, Write},
        str,
    };

    use combine::{
        between, many, many1, optional,
        parser::{char::spaces, range},
        token, EasyParser, ParseError, Parser, RangeStream,
    };
    use itertools::Itertools;
    use pretty::DocAllocator;

    type DocBuilder<'i> = pretty::DocBuilder<'i, pretty::Arena<'i, ()>>;
    type Arena<'i> = &'i pretty::Arena<'i, ()>;

    const RECORD_DEF: &str = r#"RecordSet => baseOffset batchLength partitionLeaderEpoch magic crc attributes lastOffsetDelta firstTimestamp maxTimestamp producerId producerEpoch baseSequence records
    baseOffset => int64
    batchLength => int32
    partitionLeaderEpoch => int32
    magic => int8
    crc => int32
    attributes => int16
    lastOffsetDelta => int32
    firstTimestamp => int64
    maxTimestamp => int64
    producerId => int64
    producerEpoch => int16
    baseSequence => int32
    records => [Record]
        Record => length attributes timestampDelta offsetDelta key value Headers
            length => varint
            attributes => int8
            timestampDelta => varint
            offsetDelta => varint
            key => varbytes
            value => varbytes
            Headers => [:Header]
                Header => headerKey Value
                    headerKey => varstring
                    Value => varbytes"#;

    macro_rules! chain {
        ($alloc: expr, $first: expr, $($rest: expr),* $(,)?) => {{
            #[allow(unused_mut)]
            let mut doc = ::pretty::DocBuilder($alloc, $first.into());
            $(
                doc = doc.append($rest);
            )*
            doc
        }}
    }

    #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
    struct Param {
        name: &'static str,
        default: &'static str,
    }

    struct TyDef<'i> {
        name: Cow<'i, str>,
        params: &'static [Param],
    }

    impl<'i> From<&'i str> for TyDef<'i> {
        fn from(name: &'i str) -> Self {
            TyDef {
                name: name.into(),
                params: &[],
            }
        }
    }

    impl<'i> From<String> for TyDef<'i> {
        fn from(name: String) -> Self {
            TyDef {
                name: name.into(),
                params: &[],
            }
        }
    }

    impl<'i> TyDef<'i> {
        fn with_lifetime(name: impl Into<Cow<'i, str>>) -> Self {
            TyDef {
                name: name.into(),
                params: &[Param {
                    name: "'i",
                    default: "",
                }],
            }
        }

        fn to_doc(self, arena: Arena<'i>) -> DocBuilder<'i> {
            arena.text(self.name)
        }
    }

    #[derive(Debug, PartialEq)]
    enum Size {
        S8,
        S16,
        S32,
        S64,
    }

    impl fmt::Display for Size {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::S8 => f.write_str("8"),
                Self::S16 => f.write_str("16"),
                Self::S32 => f.write_str("32"),
                Self::S64 => f.write_str("64"),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum Type {
        ApiKey,
        ErrorCode,
        Acks,
        VarInt,
        VarString,
        VarBytes,
        Int(Size),
        UInt(Size),
        NullableBytes,
        NullableString,
        String,
        Bytes,
        Bool,
        Array,
        Records,
    }

    impl Type {
        fn new(name: &str) -> Type {
            Type::new_(name).unwrap_or_else(|| panic!("Unexpected type: {}", name))
        }

        fn new_(name: &str) -> Option<Type> {
            Some(match name {
                _ if name.starts_with("int") || name.starts_with("INT") => {
                    Type::Int(match &name[3..] {
                        "8" => Size::S8,
                        "16" => Size::S16,
                        "32" => Size::S32,
                        "64" => Size::S64,
                        _ => panic!("Unexpected type {}", name),
                    })
                }
                _ if name.starts_with("uint") || name.starts_with("UINT") => {
                    Type::UInt(match &name[3..] {
                        "8" => Size::S8,
                        "16" => Size::S16,
                        "32" => Size::S32,
                        "64" => Size::S64,
                        _ => panic!("Unexpected type {}", name),
                    })
                }

                "varstring" => Type::VarString,
                "varbytes" => Type::VarBytes,
                "varint" => Type::VarInt,
                "BYTES" => Type::Bytes,
                "NULLABLE_BYTES" => Type::NullableBytes,
                "STRING" => Type::String,
                "NULLABLE_STRING" => Type::NullableString,
                "BOOLEAN" => Type::Bool,
                _ if name.starts_with("ARRAY") => Type::Array, // TODO
                "RECORDS" => Type::Records,                    // TODO
                _ => return None,
            })
        }

        fn generate_parser<'i>(&self, arena: Arena<'i>) -> DocBuilder<'i> {
            match self {
                Type::ErrorCode => arena.text("be_i16().and_then(|i| ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))"),
                Type::ApiKey =>  arena.text("be_i16().and_then(|i| ApiKey::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))"),
                Type::Acks => arena.text("be_i16().and_then(|i| Acks::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))"),
                Type::Int(s) => arena.text(format!(
                    "be_i{}()",
                    s
                )),
                Type::UInt(s)  => arena.text(format!(
                    "be_u{}()",
                    s
                )),
                Type::VarString => arena.text(format!("varstring()")),
                Type::VarBytes => arena.text(format!("varbytes()")),
                Type::VarInt => arena.text(format!("varint()")),
                Type::Bytes => arena.text(format!("bytes()")),
                Type::NullableBytes => arena.text(format!("nullable_bytes()")),
                Type::String => arena.text(format!("string()")),
                Type::NullableString => arena.text(format!("nullable_string()")),
                Type::Bool => arena.text(format!("any().map(|b| b != 0)")),
                Type::Array => arena.text(format!("bytes()")),
                Type::Records => arena.text(format!("R::parser()")),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum KafkaParser<'i> {
        Atom(Type),
        Array {
            varlen: bool,
            elem_name: &'i str,
            elem: Box<KafkaParser<'i>>,
        },
        Composite {
            needed_params: BTreeSet<Param>,
            parsers: Vec<KafkaParserField<'i>>,
        },
    }

    #[derive(Debug, PartialEq)]
    struct KafkaParserField<'i> {
        name: &'i str,
        parser: KafkaParser<'i>,
    }

    #[derive(Debug, PartialEq)]
    struct Rule<'i> {
        name: &'i str,
        version: Option<i32>,
        production: Vec<Elem<'i>>,
        inner: Vec<Rule<'i>>,
    }

    #[derive(Clone, PartialEq, Debug)]
    enum Elem<'i> {
        Multi { varlen: bool, name: &'i str },
        Ident(&'i str),
    }

    impl<'i> Elem<'i> {
        fn name(&self) -> &'i str {
            match *self {
                Elem::Multi { name, .. } | Elem::Ident(name) => name,
            }
        }
    }

    fn write_ty<'i>(arena: Arena<'i>, field: &str, ty: &Type) -> Option<DocBuilder<'i>> {
        ty_def(field, ty).map(|def| def.to_doc(arena))
    }

    fn ty_def<'i>(field: &str, ty: &Type) -> Option<TyDef<'i>> {
        match field {
            "error_code" => return Some("ErrorCode".into()),
            "api_key" => return Some("ApiKey".into()),
            "acks" => return Some("Acks".into()),
            _ => (),
        }
        Some(match ty {
            Type::Int(s) => format!("i{}", s).into(),
            Type::UInt(s) => format!("u{}", s).into(),
            Type::VarInt => "i32".into(),
            Type::Bytes | Type::VarBytes => TyDef::with_lifetime("&'i [u8]"),
            Type::NullableBytes => TyDef::with_lifetime("Option<&'i [u8]>"),
            Type::String | Type::VarString => TyDef::with_lifetime("&'i str"),
            Type::NullableString => TyDef::with_lifetime("Option<&'i str>"),
            Type::Bool => "bool".into(),
            Type::Array => TyDef::with_lifetime("&'i [u8]"),
            Type::Records => TyDef {
                name: "Option<RecordBatch<R>>".into(),
                params: &[Param {
                    name: "R",
                    default: "", // "Vec<Record<'i>>",
                }],
            },
            _ => return None,
        })
    }

    impl<'i> KafkaParserField<'i> {
        fn snakecase_name(&self) -> String {
            inflector::cases::snakecase::to_snake_case(self.name)
        }

        fn write_ty(&self, arena: Arena<'i>) -> DocBuilder<'i> {
            self.parser.write_ty(self.name, arena)
        }
    }

    impl<'i> KafkaParser<'i> {
        fn write_ty(&self, name: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
            match self {
                KafkaParser::Composite { .. } => chain![
                    arena,
                    inflector::cases::pascalcase::to_pascal_case(name),
                    self.lifetime()
                ],
                KafkaParser::Array {
                    elem, elem_name, ..
                } => {
                    chain![arena, "Vec<", elem.write_ty(elem_name, arena), ">"]
                }
                KafkaParser::Atom(t) => write_ty(arena, name, t).unwrap_or_else(|| {
                    arena.text(inflector::cases::pascalcase::to_pascal_case(name))
                }),
            }
        }

        fn generate(&self, name: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
            match self {
                KafkaParser::Atom(typ) => chain![
                    arena,
                    typ.generate_parser(arena),
                    ".expected(\"",
                    name,
                    "\")",
                ],
                KafkaParser::Array {
                    varlen,
                    elem,
                    elem_name,
                } => chain![
                    arena,
                    if *varlen { "vararray" } else { "array" },
                    "(||",
                    elem.generate(elem_name, arena),
                    ".expected(\"",
                    name,
                    "\"),",
                    ")",
                ],
                KafkaParser::Composite { parsers, .. } => {
                    if parsers.is_empty() {
                        return chain![arena, arena.line_(), "value(", name, "{})",];
                    }

                    let name = inflector::cases::pascalcase::to_pascal_case(name);

                    chain![
                        arena,
                        arena.line_(),
                        "(",
                        chain![
                            arena,
                            arena.line_(),
                            arena.intersperse(
                                parsers.iter().map(|parser| {
                                    parser.parser.generate(&parser.name, arena).append(",")
                                }),
                                arena.line()
                            ),
                        ]
                        .nest(4),
                        arena.line_(),
                        ")",
                        ".map(|(",
                        arena.intersperse(
                            parsers.iter().map(|e| e.snakecase_name()),
                            arena.text(",")
                        ),
                        ",)| {",
                        chain![
                            arena,
                            name,
                            "{",
                            arena.line(),
                            arena.intersperse(
                                parsers.iter().map(|e| e.snakecase_name()),
                                arena.text(",")
                            ),
                            arena.line(),
                            "}",
                        ],
                        "})",
                    ]
                }
            }
        }

        fn generate_struct(
            &self,
            name: &'i str,
            out: &mut Vec<DocBuilder<'i>>,
            arena: Arena<'i>,
        ) -> DocBuilder<'i> {
            match self {
                KafkaParser::Array {
                    elem, elem_name, ..
                } => elem.generate_struct(elem_name, out, arena),
                KafkaParser::Composite { parsers, .. } => chain![
                    arena,
                    "#[derive(Clone, Debug, PartialEq)]",
                    arena.line_(),
                    "pub struct ",
                    inflector::cases::pascalcase::to_pascal_case(name),
                    self.params_definition(),
                    " {",
                    KafkaParser::generate_fields(parsers, out, arena),
                    arena.line_(),
                    "}",
                    arena.line_(),
                    arena.line_(),
                    "impl",
                    self.lifetime(),
                    " crate::Encode for ",
                    inflector::cases::pascalcase::to_pascal_case(name),
                    self.lifetime(),
                    " where ",
                    arena.concat(
                        self.non_lifetime_params()
                            .map(|param| { chain![arena, param.name, ": Encode,"] })
                    ),
                    "{",
                    chain![
                        arena,
                        arena.line_(),
                        "fn encode_len(&self) -> usize {",
                        chain![
                            arena,
                            arena.line_(),
                            if parsers.is_empty() {
                                arena.text("0")
                            } else {
                                arena.intersperse(
                                    parsers.iter().map(|elem| {
                                        chain![
                                            arena,
                                            "self.",
                                            elem.snakecase_name(),
                                            ".encode_len()",
                                        ]
                                    }),
                                    arena.text(" + "),
                                )
                            }
                        ]
                        .nest(4),
                        "}",
                        arena.line_(),
                        "fn encode(&self, ",
                        if parsers.is_empty() { "_" } else { "writer" },
                        ": &mut impl Buffer) {",
                        chain![
                            arena,
                            arena.line_(),
                            arena.intersperse(
                                parsers.iter().map(|elem| {
                                    chain![
                                        arena,
                                        "self.",
                                        elem.snakecase_name(),
                                        ".encode(writer);",
                                    ]
                                }),
                                arena.line()
                            ),
                        ]
                        .nest(4),
                        "}",
                    ]
                    .nest(4),
                    "}",
                ],
                _ => arena.nil(),
            }
        }

        fn generate_fields(
            parsers: &[KafkaParserField<'i>],
            out: &mut Vec<DocBuilder<'i>>,
            arena: Arena<'i>,
        ) -> DocBuilder<'i> {
            chain![
                arena,
                arena.line_(),
                arena.intersperse(
                    parsers.iter().map(|parser_field| {
                        let struct_doc =
                            parser_field
                                .parser
                                .generate_struct(&parser_field.name, out, arena);
                        dbg!(&parser_field.parser, &parser_field.parser.lifetime());
                        out.push(struct_doc);
                        chain![
                            arena,
                            "pub ",
                            parser_field.snakecase_name(),
                            ":",
                            arena.line(),
                            parser_field.write_ty(arena),
                            ",",
                        ]
                        .group()
                    }),
                    arena.line()
                ),
            ]
            .nest(4)
        }

        fn needed_params(&self) -> BTreeSet<Param> {
            match self {
                KafkaParser::Composite { needed_params, .. } => needed_params.clone(),
                KafkaParser::Array { elem, .. } => elem.needed_params(),
                KafkaParser::Atom(_) => BTreeSet::new(),
            }
        }

        fn non_lifetime_params(&self) -> impl Iterator<Item = Param> {
            self.needed_params()
                .iter()
                .cloned()
                .filter(|param| param.name != "'i")
                .collect::<Vec<_>>()
                .into_iter()
        }

        fn params_definition(&self) -> String {
            let params = self.needed_params();
            if params.is_empty() {
                "".into()
            } else {
                format!(
                    "<{}>",
                    params
                        .iter()
                        .map(|p| if p.default.is_empty() {
                            p.name.to_string()
                        } else {
                            format!("{} = {}", p.name, p.default)
                        })
                        .format(", ")
                )
            }
        }

        fn lifetime(&self) -> String {
            let params = self.needed_params();
            if params.is_empty() {
                "".into()
            } else {
                format!("<{}>", params.iter().map(|p| p.name).format(", "))
            }
        }
    }

    impl<'i> Rule<'i> {
        fn as_parser_top(&self) -> KafkaParser<'i> {
            self.as_parser(true)
        }

        fn as_parser(&self, top: bool) -> KafkaParser<'i> {
            let mut parsers: Vec<_> = self
                .production
                .iter()
                .map(|elem| KafkaParserField {
                    name: elem.name(),
                    parser: match *elem {
                        Elem::Multi { varlen, name: i } => {
                            let inner = self
                                .inner
                                .iter()
                                .find(|rule| rule.name == i)
                                .unwrap_or_else(|| panic!("Missing inner rule: {}", i));
                            let t = inner.as_parser(false);
                            KafkaParser::Array {
                                varlen,
                                elem_name: i,
                                elem: Box::new(t),
                            }
                        }
                        Elem::Ident(i) => match self.inner.iter().find(|rule| rule.name == i) {
                            Some(inner_rule) => inner_rule.as_parser(false),
                            None => KafkaParser::Atom(self.as_type(i)),
                        },
                    },
                })
                .collect();

            if parsers.len() == 1 && !top {
                parsers.pop().unwrap().parser
            } else {
                KafkaParser::Composite {
                    parsers,
                    needed_params: self.needed_params(),
                }
            }
        }

        fn as_type(&self, field: &str) -> Type {
            match self.name {
                "error_code" => Type::ErrorCode,
                "api_key" => Type::ApiKey,
                "acks" => Type::Acks,
                _ => Type::new(field),
            }
        }

        fn generate_fn(&self, parser: &KafkaParser, out: &mut impl io::Write) -> io::Result<()> {
            let arena = pretty::Arena::new();

            let name = self.name.replace(" ", "");

            let fn_doc = chain![
                &arena,
                "use super::*;",
                arena.line_(),
                "pub fn ",
                inflector::cases::snakecase::to_snake_case(&name),
                "<'i, ",
                arena.concat(self.non_lifetime_params().map(|param| {
                    arena
                        .text(param.name)
                        .append(": RecordBatchParser<I> + 'i, ")
                })),
                "I>() -> impl Parser<I, Output = ",
                &name,
                self.lifetime(),
                "> + 'i",
                arena.line_(),
                "where",
                chain![
                    &arena,
                    arena.line_(),
                    "I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,",
                    arena.line_(),
                    "I::Error: ParseError<I::Token, I::Range, I::Position>,",
                ]
                .nest(4),
                arena.line_(),
                "{",
                parser.generate(&name, &arena).nest(4),
                arena.line(),
                "}"
            ];

            let mut structs = Vec::new();
            let struct_doc = self.generate_struct(parser, &name, &mut structs, &arena);

            let doc = chain![
                &arena,
                fn_doc,
                arena.line_(),
                arena.line_(),
                struct_doc,
                arena.hardline(),
                arena.hardline(),
                arena.intersperse(structs, arena.hardline().append(arena.hardline())),
            ];
            writeln!(out, "{}", doc.1.pretty(80))
        }

        fn needed_params(&self) -> BTreeSet<Param> {
            self.production
                .iter()
                .flat_map(|e| {
                    Type::new_(e.name())
                        .and_then(|t| ty_def("", &t))
                        .map_or(&[][..], |def| def.params)
                })
                .cloned()
                .chain(self.inner.iter().flat_map(|rule| rule.needed_params()))
                .collect()
        }

        fn non_lifetime_params(&self) -> impl Iterator<Item = Param> {
            self.needed_params()
                .iter()
                .cloned()
                .filter(|param| param.name != "'i")
                .collect::<Vec<_>>()
                .into_iter()
        }

        fn lifetime(&self) -> String {
            let params = self.needed_params();
            if params.is_empty() {
                "".into()
            } else {
                format!("<{}>", params.iter().map(|p| p.name).format(", "))
            }
        }

        fn generate_struct(
            &self,
            parser: &KafkaParser<'i>,
            name: &'i str,
            out: &mut Vec<DocBuilder<'i>>,
            arena: Arena<'i>,
        ) -> DocBuilder<'i> {
            chain![
                arena,
                parser.generate_struct(name, out, arena),
                if let Some(version) = self.version {
                    chain![
                        arena,
                        arena.line(),
                        arena.line(),
                        "pub const VERSION: i16 = ",
                        arena.as_string(version),
                        ";",
                    ]
                } else {
                    arena.nil()
                }
            ]
        }
    }

    fn ident<'i, I>() -> impl Parser<I, Output = &'i str>
    where
        I: RangeStream<Token = char, Range = &'i str>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        range::take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '\'')
            .map(|s: &str| s.trim_end_matches('\''))
            .expected("identifier")
    }

    fn elem<'i, I>() -> impl Parser<I, Output = Elem<'i>>
    where
        I: RangeStream<Token = char, Range = &'i str>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        let ident_or_array = range::take_while1(|c: char| {
            c.is_alphanumeric() || c == '_' || c == '\'' || c == '(' || c == ')'
        })
        .map(|s: &str| s.trim_end_matches('\''))
        .expected("identifier or array");
        between(token('['), token(']'), (optional(token(':')), ident()))
            .map(|(var, elem)| Elem::Multi {
                varlen: var.is_some(),
                name: elem,
            })
            .or(ident_or_array.map(Elem::Ident))
    }

    fn production<'i, I>() -> impl Parser<I, Output = Vec<Elem<'i>>>
    where
        I: RangeStream<Token = char, Range = &'i str>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    {
        (
            many(elem().skip(optional(token(' ')))),
            optional(token(' ')),
        )
            .map(|(elems, _)| elems)
    }

    combine::parser! {
    fn rule['i, I]()(I) -> (usize, Rule<'i>)
    where [
        I: RangeStream<Token = char, Range = &'i str>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
    ]
    {
        (
            range::take_while(|c: char| c == ' '),
            range::take_while1(|c: char| c != '=' && c != '(').map(str::trim).map(|s: &str| s.trim_end_matches('\'')),
            optional(between(
                    token('('),
                    token(')'),
                    range::range("Version:")
                        .skip(spaces())
                        .with(combine::from_str(range::take_while(|c: char| c.is_digit(10)))),
                )
            ),
            spaces(),
            range::range("=>").skip(spaces()),
            production(),
        ).skip(optional(token('\n')))
            .map(|(level, name, version, _, _, production): (&str, _, _, _, _, _)| (level.len(), Rule { name, version, production, inner: Vec::new(), }))
    }
    }

    fn parse_rule(input: &str) -> io::Result<Rule<'_>> {
        let (rules, rest) = many1(rule())
            .easy_parse(&input[..])
            .map_err(|err| err.map_position(|p| p.translate_position(&input[..])))
            .unwrap_or_else(|err| panic!("{}\nInput is {} len", err, input.len()));
        assert!(rest.trim().is_empty(), "{}", rest);

        fn fixup<'i>(
            current: &mut Vec<Rule<'i>>,
            level: usize,
            rules: &mut impl Iterator<Item = (usize, Rule<'i>)>,
        ) -> Option<(usize, Rule<'i>)> {
            let mut x = rules.next();
            loop {
                let (next_level, next) = x?;
                x = if next_level == level || current.is_empty() {
                    current.push(next);
                    fixup(current, level, rules)
                } else if next_level > level {
                    let current = &mut current.last_mut().expect("Rule").inner;
                    current.push(next);
                    fixup(current, next_level, rules)
                } else {
                    return Some((next_level, next));
                }
            }
        }

        let mut current = Vec::new();
        // eprintln!("{:#?}", rules);
        let mut iter = Vec::into_iter(rules);
        fixup(&mut current, 0, &mut iter);
        assert_eq!(current.len(), 1, "Fixup did not merge all rules into one");
        assert!(iter.next().is_none());
        let rule = current.pop().unwrap();
        Ok(rule)
    }

    fn merge_rule<'a>(mut l_rule: Rule<'a>, r_rule: Rule<'a>) -> Rule<'a> {
        if true {
            return r_rule;
        }
        if l_rule == r_rule {
            return l_rule;
        }
        assert_eq!(l_rule.name, r_rule.name);

        {
            let mut l_iter = l_rule.production.iter().peekable();
            let mut r_iter = r_rule.production.iter().peekable();
            let mut production = Vec::new();

            while let Some(l) = l_iter.next() {
                if let Some(r) = r_iter.next() {
                    if l == r {
                        production.push(l.clone());
                    } else {
                        while r_iter.peek().is_some() && r_iter.peek() != Some(&l) {
                            production.push(r_iter.next().unwrap().clone());
                        }
                        production.push(l.clone());
                    }
                } else {
                    production.push(l.clone());
                }
            }
            production.extend(r_iter.cloned());

            l_rule.production = production;
        }
        for (l, r) in l_rule.inner.iter_mut().zip(r_rule.inner) {
            match (l, r) {
                (_, _) => (),
            }
        }

        l_rule
    }

    fn generate_parsers() -> Result<(), Box<dyn std::error::Error>> {
        let inputs = serde_json::from_str::<Vec<String>>(&fs::read_to_string(
            "kafka_request_responses.json",
        )?)
        .unwrap();

        let rules = inputs
            .iter()
            .map(|s| s.as_str())
            .chain(Some(RECORD_DEF))
            .enumerate()
            .map(|(_, input)| -> io::Result<_> {
                // eprintln!("Input {}: {}", i, input);
                parse_rule(input)
            })
            .collect::<io::Result<Vec<_>>>()?;

        let mut parser_out = io::BufWriter::new(fs::File::create("src/parser.rs")?);
        writeln!(parser_out, "use super::*;")?;

        let mut calls = std::collections::BTreeMap::<_, (Option<_>, Option<_>)>::new();

        let parser_filter = std::env::var("PARSER_FILTER").ok();
        println!("cargo:rerun-if-env-changed=PARSER_FILTER");
        let rules = rules
            .into_iter()
            .filter(|rule| {
                parser_filter
                    .as_ref()
                    .map_or(true, |f| rule.name.contains(f))
            })
            .group_by(|rule| rule.name)
            .into_iter()
            .map(|(_, mut group)| {
                let mut merged_rule = group.next().unwrap();
                for rule in group {
                    merged_rule = merge_rule(merged_rule, rule);
                }
                merged_rule
            })
            .collect::<Vec<_>>();
        for rule in &rules {
            let parser = rule.as_parser_top();
            let entry = calls
                .entry(
                    rule.name
                        .trim_end_matches(" Request")
                        .trim_end_matches(" Response"),
                )
                .or_default();
            if rule.name.ends_with("Request") {
                entry.0 = Some(rule);
            } else {
                entry.1 = Some(rule);
            }
            let type_name = rule.name.replace(" ", "");
            let name = inflector::cases::snakecase::to_snake_case(&type_name);
            let mut out = io::BufWriter::new(
                fs::File::create(format!("src/parser/{}.rs", name))
                    .map_err(|err| format!("Unable to create module {}: {}", name, err))?,
            );
            // eprintln!("{:#?}", rule);
            let mut s = Vec::new();
            rule.generate_fn(&parser, &mut s)?;
            write!(out, "{}", str::from_utf8(&s).unwrap())?;

            writeln!(parser_out, "pub mod {};", name)?;
            writeln!(
                parser_out,
                "pub use self::{name}::{{{type_name}, {name}}};",
                name = name,
                type_name = type_name
            )?;
        }

        writeln!(
            parser_out,
            "{}",
            r#"impl<I> Client<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{"#
        )?;
        for (call, request, response) in calls
            .into_iter()
            .filter(|(call, _)| *call != "Response Header" && *call != "Request Header")
            .filter_map(|(call, (request, response))| Some((call, request?, response?)))
        {
            let base_type_name = call.replace(" ", "");
            let name = inflector::cases::snakecase::to_snake_case(&base_type_name);

            writeln!(
                parser_out,
                "pub async fn {name}<'i{ty_params}>(&'i mut self, request: {base_type_name}Request{request_lt}) -> io::Result<{base_type_name}Response{response_lt}> where {where_bounds} {{",
                name = name,
                ty_params = request.needed_params()
                    .iter().chain(response.needed_params().iter())
                    .filter(|param| param.name != "'i")
                    .map(|p| format!(", {}", p.name))
                    .format(""),
                base_type_name = base_type_name,
                request_lt = request.lifetime().replace("'i", "'_"),
                response_lt = response.lifetime(),
                where_bounds = request.non_lifetime_params().map(|param| {
                        format!("{}: Encode,", param.name)
                    })
                    .chain(
                        response.non_lifetime_params().map(|param| format!("{}: RecordBatchParser<combine::stream::easy::Stream<&'i [u8]>> + 'i,", param.name))
                    )
                    .format(" ")
            )?;
            writeln!(parser_out, "    self.call(request, ApiKey::{base_type_name}, {name}_request::VERSION, {name}_response()).await", name = name, base_type_name = base_type_name)?;
            writeln!(parser_out, "}}")?;
        }
        writeln!(parser_out, "}}")?;
        Ok(())
    }

    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        println!("cargo:rerun-if-changed=kafka_request_responses.json");
        generate_parsers().map_err(|err| format!("Unable to generate parser: {}", err))?;

        {
            println!("cargo:rerun-if-changed=kafka_errors.txt");
            let kafka_errors = fs::read_to_string("kafka_errors.txt")?;
            let mut out = io::BufWriter::new(fs::File::create("src/error.rs")?);

            writeln!(out, "use std::convert::TryFrom;")?;

            writeln!(out, "#[derive(Clone, Copy, Eq, PartialEq, Debug)]")?;
            writeln!(out, "pub enum ErrorCode {{")?;
            let iter = || {
                kafka_errors.lines().map(|line| {
                    let mut s = line.split('\t');
                    let name = s.next().expect("name");
                    let name = inflector::cases::pascalcase::to_pascal_case(name);
                    let number = s.next().expect("number");
                    let _retriable = s.next().expect("retriable");
                    let doc = s.next();
                    (name, number, doc)
                })
            };
            for (name, number, doc) in iter() {
                if let Some(doc) = doc {
                    writeln!(out, "    /// {}", doc)?;
                }
                writeln!(out, "    {name} = {number},", name = name, number = number)?;
            }
            writeln!(out, "}}")?;

            writeln!(out, "impl TryFrom<i16> for ErrorCode {{")?;
            writeln!(out, "    type Error = &'static str;")?;
            writeln!(
                out,
                "    fn try_from(i: i16) -> Result<Self, Self::Error> {{"
            )?;
            writeln!(out, "        Ok(match i {{")?;
            for (name, number, _doc) in iter() {
                writeln!(out, "            {} => ErrorCode::{},", number, name)?;
            }
            writeln!(out, r#"            _ => return Err("Invalid ErrorCode")"#)?;
            writeln!(out, "        }})")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }
        {
            println!("cargo:rerun-if-changed=api_keys.txt");
            let api_keys = fs::read_to_string("api_keys.txt")?;
            let mut out = io::BufWriter::new(fs::File::create("src/api_key.rs")?);

            let iter = || {
                api_keys.lines().map(|line| {
                    let mut s = line.split('\t');
                    let name = s.next().expect("name");
                    let number = s.next().expect("number");
                    (name, number)
                })
            };

            writeln!(out, "use std::convert::TryFrom;")?;
            writeln!(out, "#[derive(Clone, Copy, Eq, PartialEq, Debug)]")?;
            writeln!(out, "pub enum ApiKey {{")?;
            for (name, number) in iter() {
                writeln!(out, "    {name} = {number},", name = name, number = number)?;
            }
            writeln!(out, "}}")?;

            writeln!(out, "impl TryFrom<i16> for ApiKey {{")?;
            writeln!(out, "    type Error = &'static str;")?;
            writeln!(
                out,
                "    fn try_from(i: i16) -> Result<Self, Self::Error> {{"
            )?;
            writeln!(out, "        Ok(match i {{")?;
            for (name, number) in iter() {
                writeln!(out, "            {} => ApiKey::{},", number, name)?;
            }
            writeln!(out, r#"            _ => return Err("Invalid ApiKey")"#)?;
            writeln!(out, "        }})")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        std::process::Command::new("cargo")
            .arg("fmt")
            .status()
            .unwrap();

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "regenerate")]
    regenerate::main()?;
    if std::env::var("PARSER_FILTER").is_ok() {
        panic!();
    }

    Ok(())
}
