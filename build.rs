#[cfg(feature = "regenerate")]
mod regenerate {
    use std::{
        borrow::Cow,
        collections::BTreeSet,
        fs,
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
    ($alloc: expr; $first: expr, $($rest: expr),* $(,)?) => {{
            #[allow(unused_mut)]
            let mut doc = ::pretty::DocBuilder($alloc, $first.into());
            $(
                doc = doc.append($rest);
            )*
            doc
        }}
    }

    #[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
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

    #[derive(Debug)]
    struct Rule<'i> {
        name: &'i str,
        version: Option<i32>,
        production: Vec<Elem<'i>>,
        inner: Vec<Rule<'i>>,
    }

    #[derive(Debug)]
    enum Elem<'i> {
        Multi(bool, &'i str),
        Ident(&'i str),
    }

    impl<'i> Elem<'i> {
        fn snakecase_name(&self) -> String {
            inflector::cases::snakecase::to_snake_case(self.name())
        }
        fn name(&self) -> &'i str {
            match *self {
                Elem::Multi(_, i) | Elem::Ident(i) => i,
            }
        }
    }

    fn write_parser<'i>(field: &str, i: &'i str, arena: Arena<'i>) -> Option<DocBuilder<'i>> {
        match field {
            "error_code" => return Some(arena.text("be_i16().and_then(|i| ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))")),
            "api_key" => return Some(arena.text("be_i16().and_then(|i| ApiKey::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))")),
            "acks" => return Some(arena.text("be_i16().and_then(|i| Acks::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))")),
            _ => (),
        }
        Some(match i {
            _ if i.starts_with("int") || i.starts_with("INT") => arena.text(format!(
                "be_i{}()",
                i.trim_start_matches(char::is_alphabetic)
            )),
            _ if i.starts_with("uint") || i.starts_with("UINT") => arena.text(format!(
                "be_u{}()",
                i.trim_start_matches(char::is_alphabetic)
            )),
            "varstring" => arena.text(format!("varstring()")),
            "varbytes" => arena.text(format!("varbytes()")),
            "varint" => arena.text(format!("varint()")),
            "BYTES" => arena.text(format!("bytes()")),
            "NULLABLE_BYTES" => arena.text(format!("nullable_bytes()")),
            "STRING" => arena.text(format!("string()")),
            "NULLABLE_STRING" => arena.text(format!("nullable_string()")),
            "BOOLEAN" => arena.text(format!("any().map(|b| b != 0)")),
            _ if i.starts_with("ARRAY") => arena.text(format!("bytes()")), // TODO
            "RECORDS" => arena.text(format!("R::parser()")),               // TODO
            _ => return None,
        })
    }

    fn write_ty<'i>(arena: Arena<'i>, field: &str, ty: &'i str) -> Option<DocBuilder<'i>> {
        ty_def(field, ty).map(|def| def.to_doc(arena))
    }

    fn ty_def<'i>(field: &str, ty: &'i str) -> Option<TyDef<'i>> {
        match field {
            "error_code" => return Some("ErrorCode".into()),
            "api_key" => return Some("ApiKey".into()),
            "acks" => return Some("Acks".into()),
            _ => (),
        }
        Some(match ty {
            _ if ty.starts_with("int") || ty.starts_with("INT") => {
                format!("i{}", ty.trim_start_matches(char::is_alphabetic)).into()
            }
            _ if ty.starts_with("uint") || ty.starts_with("UINT") => {
                format!("u{}", ty.trim_start_matches(char::is_alphabetic)).into()
            }
            "varint" => "i32".into(),
            "BYTES" | "varbytes" => TyDef::with_lifetime("&'i [u8]"),
            "NULLABLE_BYTES" => TyDef::with_lifetime("Option<&'i [u8]>"),
            "STRING" | "varstring" => TyDef::with_lifetime("&'i str"),
            "NULLABLE_STRING" => TyDef::with_lifetime("Option<&'i str>"),
            "BOOLEAN" => "bool".into(),
            _ if ty.starts_with("ARRAY") => TyDef::with_lifetime("&'i [u8]"),
            "RECORDS" => TyDef {
                name: "Option<RecordBatch<R>>".into(),
                params: &[Param {
                    name: "R",
                    default: "", // "Vec<Record<'i>>",
                }],
            },
            _ => return None,
        })
    }

    fn write_field<'i>(name: &'i str, i: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
        chain![arena;
            "pub ",
            inflector::cases::snakecase::to_snake_case(&name),
            ":",
            arena.line(),
            write_ty(arena, name, i).unwrap_or_else(|| arena.as_string(i)),
            ",",
        ]
        .group()
    }

    impl<'i> Rule<'i> {
        fn generate_fn(&self, out: &mut impl io::Write) -> io::Result<()> {
            let arena = pretty::Arena::new();

            let name = self.name.replace(" ", "");

            let fn_doc = chain![&arena;
                "use super::*;",
                arena.line_(),
                "pub fn ",
                inflector::cases::snakecase::to_snake_case(&name),
                "<'i, ",
                arena.concat(
                    self.non_lifetime_params()
                    .map(|param| arena.text(param.name).append(": RecordBatchParser<I> + 'i, "))
                ),
                "I>() -> impl Parser<I, Output = ",
                &name,
                self.lifetime(),
                "> + 'i",
                arena.line_(),
                "where",
                chain![&arena;
                    arena.line_(),
                    "I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,",
                    arena.line_(),
                    "I::Error: ParseError<I::Token, I::Range, I::Position>,",
                ].nest(4),
                arena.line_(),
                "{",
                self.generate(&name, &arena).nest(4),
                arena.line(),
                "}"
            ];

            let mut structs = Vec::new();
            let struct_doc = self.generate_struct(&name, &mut structs, &arena);

            let doc = chain![&arena;
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

        fn generate(&self, name: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
            if let Some(doc) = self
                .production
                .first()
                .and_then(|prod| write_parser(name, prod.name(), arena))
            {
                return doc;
            }

            if self.production.is_empty() {
                return chain![arena;
                    arena.line_(),
                    "value(",
                    name,
                    "{})",
                ];
            }

            let name = inflector::cases::pascalcase::to_pascal_case(name);

            chain![arena;
                arena.line_(),
                "(",
                chain![arena;
                    arena.line_(),
                    arena.intersperse(self.production.iter().map(|elem| {
                        match *elem {
                            Elem::Multi(varlen, i) => {
                                let inner = self.inner
                                    .iter()
                                    .find(|rule| rule.name == i)
                                    .unwrap_or_else(|| panic!("Missing inner rule: {}", i));
                                chain![arena;
                                    if varlen {
                                        "vararray"
                                    } else {
                                        "array"
                                    },
                                    "(||",
                                    inner.generate(i, arena),
                                    ".expected(\"",
                                    i,
                                    "\"),",
                                    "),",
                                ]
                            }
                            Elem::Ident(i) => {
                                chain![arena;
                                    match self.inner
                                        .iter()
                                        .find(|rule| rule.name == i)
                                    {
                                        Some(inner_rule) =>
                                            write_parser(i, inner_rule.production.first().unwrap().name(), arena).unwrap_or_else(|| inner_rule.generate(i, arena)),
                                        None => write_parser(i, i, arena).unwrap_or_else(|| panic!("write_parser: {} {:#?}", i, self.inner)),
                                    },
                                    ".expected(\"",
                                    i,
                                    "\"),",
                                ]
                            }
                        }
                    }), arena.line()),
                ].nest(4),
                arena.line_(),
                ")",
                ".map(|(",
                arena.intersperse(self.production.iter().map(|e| e.snakecase_name()), arena.text(",")),
                ",)| {",
                chain![arena;
                    name,
                    "{",
                    arena.line(),
                    arena.intersperse(self.production.iter().map(|e| e.snakecase_name()), arena.text(",")),
                    arena.line(),
                    "}",
                ],
                "})",
            ]
        }

        fn needed_params(&self) -> BTreeSet<Param> {
            self.production
                .iter()
                .flat_map(|e| ty_def("", e.name()).map_or(&[][..], |def| def.params))
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

        fn generate_struct(
            &self,
            name: &'i str,
            out: &mut Vec<DocBuilder<'i>>,
            arena: Arena<'i>,
        ) -> DocBuilder<'i> {
            chain![arena;
                "#[derive(Clone, Debug, PartialEq)]",
                arena.line_(),
                "pub struct ",
                inflector::cases::pascalcase::to_pascal_case(name),
                self.params_definition(),
                " {",
                self.generate_fields(out, arena),
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
                arena.concat(self.non_lifetime_params().map(|param| {
                    chain![arena;
                        param.name,
                        ": Encode,"
                    ]
                })),
                "{",
                chain![arena;
                    arena.line_(),
                    "fn encode_len(&self) -> usize {",
                    chain![arena;
                        arena.line_(),
                        if self.production.is_empty() {
                            arena.text("0")
                        } else {
                            arena.intersperse(self.production.iter().map(|elem| {
                                chain![arena;
                                    "self.",
                                    elem.snakecase_name(),
                                    ".encode_len()",
                                ]
                            }), arena.text(" + "))
                        }
                    ].nest(4),
                    "}",
                    arena.line_(),
                    "fn encode(&self, ",
                    if self.production.is_empty() {
                        "_"
                    } else {
                        "writer"
                    },
                    ": &mut impl Buffer) {",
                    chain![arena;
                        arena.line_(),
                        arena.intersperse(self.production.iter().map(|elem| {
                            chain![arena;
                                "self.",
                                elem.snakecase_name(),
                                ".encode(writer);",
                            ]
                        }), arena.line()),
                    ].nest(4),
                    "}",
                ].nest(4),
                "}",
                if let Some(version) = self.version {
                    chain![arena;
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

        fn generate_fields(
            &self,
            out: &mut Vec<DocBuilder<'i>>,
            arena: Arena<'i>,
        ) -> DocBuilder<'i> {
            chain![arena;
                arena.line_(),
                arena.intersperse(self.production.iter().map(|elem| {
                    match *elem {
                        Elem::Multi(_, i) => {
                            let inner = self
                                .inner
                                .iter()
                                .find(|rule| rule.name == i)
                                .unwrap_or_else(|| panic!("Missing inner rule: {}", i));

                            if let Some(ty) = inner
                                .production
                                .first()
                                .and_then(|prod| write_ty(arena, i, prod.name()))
                            {
                                return chain![arena;
                                    "pub ",
                                    inflector::cases::snakecase::to_snake_case(i),
                                    ":",
                                    arena.line_(),
                                    "Vec<",
                                    ty,
                                    ">,",
                                ]
                            }

                            let struct_doc = inner.generate_struct(i, out, arena);
                            out.push(struct_doc);

                            chain![arena;
                                "pub ",
                                inflector::cases::snakecase::to_snake_case(i),
                                ":",
                                arena.line(),
                                "Vec<",
                                inflector::cases::pascalcase::to_pascal_case(i),
                                inner.lifetime(),
                                ">,",
                            ].group()
                        }
                        Elem::Ident(i) => {
                            match self.inner
                                .iter()
                                .find(|rule| rule.name == i)
                            {
                                Some(inner_rule) => {
                                    if let Some(ty) = inner_rule
                                        .production
                                        .first()
                                        .and_then(|prod| write_ty(arena, i, prod.name()))
                                    {
                                        return chain![arena;
                                            "pub ",
                                            inflector::cases::snakecase::to_snake_case(i),
                                            ":",
                                            arena.line_(),
                                            ty,
                                            ",",
                                        ]
                                    }

                                    let struct_doc = inner_rule.generate_struct(i, out, arena);
                                    out.push(struct_doc);

                                    chain![arena;
                                        "pub ",
                                        inflector::cases::snakecase::to_snake_case(i),
                                        ":",
                                        arena.line(),
                                        inflector::cases::pascalcase::to_pascal_case(i),
                                        inner_rule.lifetime(),
                                        ",",
                                    ].group()
                                }
                                None => write_field(i, i, arena),
                            }
                        }
                    }
                }), arena.line()),
            ]
            .nest(4)
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
            .map(|(var, elem)| Elem::Multi(var.is_some(), elem))
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
        eprintln!("{:#?}", rules);
        let mut iter = Vec::into_iter(rules);
        fixup(&mut current, 0, &mut iter);
        assert_eq!(current.len(), 1, "Fixup did not merge all rules into one");
        assert!(iter.next().is_none());
        let rule = current.pop().unwrap();
        Ok(rule)
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
            .map(|(i, input)| -> io::Result<_> {
                eprintln!("Input {}: {}", i, input);
                parse_rule(input)
            })
            .collect::<io::Result<Vec<_>>>()?;

        let mut parser_out = io::BufWriter::new(fs::File::create("src/parser.rs")?);
        writeln!(parser_out, "use super::*;")?;

        let mut calls = std::collections::BTreeMap::<_, (Option<_>, Option<_>)>::new();
        for rule in rules
            .iter()
            .group_by(|rule| rule.name)
            .into_iter()
            .map(|iter| iter.1.last().unwrap())
        {
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
            rule.generate_fn(&mut s)?;
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

    Ok(())
}
