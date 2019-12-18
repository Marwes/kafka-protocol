#[cfg(feature = "regenerate")]
mod regenerate {
    use std::{
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

    #[derive(Debug)]
    struct Rule<'i> {
        name: &'i str,
        version: Option<i32>,
        production: Vec<Elem<'i>>,
        inner: Vec<Rule<'i>>,
    }

    #[derive(Debug)]
    enum Elem<'i> {
        Multi(&'i str),
        Ident(&'i str),
    }

    impl<'i> Elem<'i> {
        fn name(&self) -> &'i str {
            match *self {
                Elem::Multi(i) | Elem::Ident(i) => i,
            }
        }
    }

    fn write_parser<'i>(field: &str, i: &'i str, arena: Arena<'i>) -> Option<DocBuilder<'i>> {
        match field {
            "error_code" => return Some(arena.text("be_i16().and_then(|i| ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))")),
            _ => (),
        }
        Some(match i {
            _ if i.starts_with("INT") => arena.text(format!(
                "be_i{}()",
                i.trim_start_matches(char::is_alphabetic)
            )),
            _ if i.starts_with("UINT") => arena.text(format!(
                "be_u{}()",
                i.trim_start_matches(char::is_alphabetic)
            )),
            "BYTES" => arena.text(format!("bytes()")),
            "NULLABLE_BYTES" => arena.text(format!("nullable_bytes()")),
            "STRING" => arena.text(format!("string()")),
            "NULLABLE_STRING" => arena.text(format!("nullable_string()")),
            "BOOLEAN" => arena.text(format!("any().map(|b| b != 0)")),
            _ if i.starts_with("ARRAY") => arena.text(format!("bytes()")), // TODO
            "RECORDS" => arena.text(format!("nullable_bytes()")),          // TODO
            _ => return None,
        })
    }

    fn write_ty<'i>(field: &str, ty: &'i str) -> Option<std::borrow::Cow<'i, str>> {
        match field {
            "error_code" => return Some("ErrorCode".into()),
            _ => (),
        }
        Some(match ty {
            _ if ty.starts_with("INT") => {
                format!("i{}", ty.trim_start_matches(char::is_alphabetic)).into()
            }
            _ if ty.starts_with("UINT") => {
                format!("u{}", ty.trim_start_matches(char::is_alphabetic)).into()
            }
            "BYTES" => "&'i [u8]".into(),
            "NULLABLE_BYTES" => "Option<&'i [u8]>".into(),
            "STRING" => "&'i str".into(),
            "NULLABLE_STRING" => "Option<&'i str>".into(),
            "BOOLEAN" => "bool".into(),
            _ if ty.starts_with("ARRAY") => format!("&'i [u8]").into(), // TODO
            "RECORDS" => "Option<&'i [u8]>".into(),                     // TODO
            _ => return None,
        })
    }

    fn write_field<'i>(name: &'i str, i: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
        chain![arena;
            "pub ",
            name,
            ":",
            arena.line(),
            write_ty(name, i).unwrap_or_else(|| format!("{}", i).into()),
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
                "<'i, I>() -> impl Parser<I, Output = ",
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
                            Elem::Multi(i) => {
                                let inner = self.inner
                                    .iter()
                                    .find(|rule| rule.name == i)
                                    .unwrap_or_else(|| panic!("Missing inner rule: {}", i));
                                chain![arena;
                                    "array(||",
                                    inner.generate(i, arena),
                                    ",",
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
                                    ","
                                ]
                            }
                        }
                    }), arena.line()),
                ].nest(4),
                arena.line_(),
                ")",
                ".map(|(",
                arena.intersperse(self.production.iter().map(|e| e.name()), arena.text(",")),
                ",)| {",
                chain![arena;
                    name,
                    "{",
                    arena.line(),
                    arena.intersperse(self.production.iter().map(|e| e.name()), arena.text(",")),
                    arena.line(),
                    "}",
                ],
                "})",
            ]
        }

        fn needs_lifetime(&self) -> bool {
            self.production
                .iter()
                .any(|e| write_ty("", e.name()).map_or(false, |n| n.contains("'i")))
                || self.inner.iter().any(|rule| rule.needs_lifetime())
        }
        fn lifetime(&self) -> &'static str {
            if self.needs_lifetime() {
                "<'i>"
            } else {
                ""
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
                self.lifetime(),
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
                                    elem.name(),
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
                    ": &mut impl bytes::BufMut) {",
                    chain![arena;
                        arena.line_(),
                        arena.intersperse(self.production.iter().map(|elem| {
                            chain![arena;
                                "self.",
                                elem.name(),
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
                        Elem::Multi(i) => {
                            let inner = self
                                .inner
                                .iter()
                                .find(|rule| rule.name == i)
                                .unwrap_or_else(|| panic!("Missing inner rule: {}", i));

                            if let Some(ty) = inner
                                .production
                                .first()
                                .and_then(|prod| write_ty(i, prod.name()))
                            {
                                return chain![arena;
                                    "pub ",
                                    i,
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
                                i,
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
                                        .and_then(|prod| write_ty(i, prod.name()))
                                    {
                                        return chain![arena;
                                            "pub ",
                                            i,
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
                                        i,
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
        between(token('['), token(']'), ident())
            .map(Elem::Multi)
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
            .unwrap_or_else(|err| panic!("{}", err));
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
        assert_eq!(current.len(), 1);
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
            .enumerate()
            .map(|(i, input)| -> io::Result<_> {
                eprintln!("Input {}: {}", i, input);
                parse_rule(input)
            })
            .collect::<io::Result<Vec<_>>>()?;

        let mut parser_out = io::BufWriter::new(fs::File::create("src/parser.rs")?);
        writeln!(parser_out, "use super::*;")?;

        for rule in rules
            .into_iter()
            .group_by(|rule| rule.name)
            .into_iter()
            .map(|iter| iter.1.last().unwrap())
        {
            let name = rule.name.replace(" ", "");
            let name = inflector::cases::snakecase::to_snake_case(&name);
            let mut out = io::BufWriter::new(
                fs::File::create(format!("src/parser/{}.rs", name))
                    .map_err(|err| format!("Unable to create module {}: {}", name, err))?,
            );
            // eprintln!("{:#?}", rule);
            let mut s = Vec::new();
            rule.generate_fn(&mut s)?;
            write!(out, "{}", str::from_utf8(&s).unwrap())?;

            writeln!(parser_out, "pub mod {};", name)?;
        }
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
            writeln!(out, "#[derive(Eq, PartialEq, Debug)]")?;
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
