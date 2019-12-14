use std::{
    fs,
    io::{self, Write},
    str,
};

use combine::{
    between, many, many1, optional,
    parser::{char::spaces, range, token::value},
    sep_by1, token, EasyParser, ParseError, Parser, RangeStream,
};
use pretty::DocAllocator;

type DocBuilder<'i> = pretty::DocBuilder<'i, pretty::Arena<'i, ()>>;
type Arena<'i> = &'i pretty::Arena<'i, ()>;

macro_rules! chain {
    ($alloc: expr; $first: expr, $($rest: expr),+ $(,)?) => {{
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
    production: Production<'i>,
}

#[derive(Debug)]
enum Elem<'i> {
    Optional(&'i str),
    Ident(&'i str),
}

impl<'i> Elem<'i> {
    fn name(&self) -> &'i str {
        match *self {
            Elem::Optional(i) | Elem::Ident(i) => i,
        }
    }
}

#[derive(Debug)]
enum Production<'i> {
    Seq(Vec<Elem<'i>>, Vec<Rule<'i>>),
    Leaf(&'i str),
}

fn write_parser<'i>(i: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
    match i {
        _ if i.starts_with("INT") => arena.text(format!(
            "be_i{}(),",
            i.trim_start_matches(char::is_alphabetic)
        )),
        _ if i.starts_with("UINT") => arena.text(format!(
            "be_u{}(),",
            i.trim_start_matches(char::is_alphabetic)
        )),
        "BYTES" => arena.text(format!("bytes(),")),
        "NULLABLE_BYTES" => arena.text(format!("nullable_bytes(),")),
        "STRING" => arena.text(format!("string(),")),
        "NULLABLE_STRING" => arena.text(format!("nullable_string(),")),
        "BOOLEAN" => arena.text(format!("any().map(|b| b != 0),")),
        _ if i.starts_with("ARRAY") => arena.text(format!("array(),")), // TODO
        "RECORDS" => arena.text(format!("nullable_bytes(),")),          // TODO
        _ => arena.text(format!("    {}(),", i)),
    }
}

fn write_ty<'i>(i: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
    match i {
        _ if i.starts_with("INT") => {
            arena.text(format!("i{}", i.trim_start_matches(char::is_alphabetic)))
        }
        _ if i.starts_with("UINT") => {
            arena.text(format!("u{}", i.trim_start_matches(char::is_alphabetic)))
        }
        "BYTES" => arena.text("&'i [u8]"),
        "NULLABLE_BYTES" => arena.text("Option<&'i [u8]>"),
        "STRING" => arena.text("&'i str"),
        "NULLABLE_STRING" => arena.text("Option<&'i str>"),
        "BOOLEAN" => arena.text("bool"),
        _ if i.starts_with("ARRAY") => arena.text(format!("array(),")), // TODO
        "RECORDS" => arena.text("Option<&'i [u8]>"),                    // TODO
        _ => arena.text(format!("{}", i)),
    }
}

fn write_field<'i>(name: &'i str, i: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
    chain![arena;
        name,
        ":",
        arena.line(),
        write_ty(i, arena),
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
            "fn ",
            inflector::cases::snakecase::to_snake_case(&name),
            "<'i, I>() -> impl Parser<I, Output = ",
            &name,
            "<'i>",
            ">",
            arena.line_(),
            "where",
            chain![&arena;
                arena.line_(),
                "I: RangeStream<Token = u8, Range = &'i [u8]>,",
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
            arena.intersperse(structs, arena.hardline())
        ];
        writeln!(out, "{}", doc.1.pretty(80))
    }

    fn generate(&self, name: &'i str, arena: Arena<'i>) -> DocBuilder<'i> {
        let name = inflector::cases::pascalcase::to_pascal_case(name);
        match &self.production {
            Production::Seq(elems, inner) => chain![arena;
                arena.line_(),
                "(",
                chain![arena;
                    arena.line_(),
                    arena.intersperse(elems.iter().map(|elem| {
                        match *elem {
                            Elem::Optional(i) => {
                                let inner = inner
                                    .iter()
                                    .find(|rule| rule.name == i)
                                    .unwrap_or_else(|| panic!("Missing inner rule: {}", i));
                                chain![arena;
                                    "optional(",
                                    inner.generate(i, arena),
                                    ")",
                                ]
                            }
                            Elem::Ident(i) => {
                                let inner = match inner
                                    .iter()
                                    .find(|rule| rule.name == i)
                                    .unwrap_or_else(|| panic!("Missing inner rule: {}", i))
                                    .production
                                {
                                    Production::Leaf(i) => i,
                                    _ => panic!(),
                                };
                                write_parser(inner, arena)
                            }
                        }
                    }), arena.line()),
                ].nest(4),
                arena.line_(),
                ")",
                ".map(|(",
                arena.intersperse(elems.iter().map(|e| e.name()), arena.text(",")),
                ")| {",
                chain![arena;
                    name,
                    "{",
                    arena.line(),
                    arena.intersperse(elems.iter().map(|e| e.name()), arena.text(",")),
                    arena.line(),
                    "}",
                ],
                "})",
            ],
            Production::Leaf(i) => write_parser(i, arena),
        }
    }

    fn generate_struct(
        &self,
        name: &'i str,
        out: &mut Vec<DocBuilder<'i>>,
        arena: Arena<'i>,
    ) -> DocBuilder<'i> {
        chain![arena;
            "pub struct ",
            inflector::cases::pascalcase::to_pascal_case(name),
            "<'i> {",
            arena.line_(),
            self.generate_fields(out, arena),
            arena.line_(),
            "}",
        ]
    }

    fn generate_fields(&self, out: &mut Vec<DocBuilder<'i>>, arena: Arena<'i>) -> DocBuilder<'i> {
        match &self.production {
            Production::Seq(elems, inner) => chain![arena;
                arena.line_(),
                arena.intersperse(elems.iter().map(|elem| {
                    match *elem {
                        Elem::Optional(i) => {
                            let inner = inner
                                .iter()
                                .find(|rule| rule.name == i)
                                .unwrap_or_else(|| panic!("Missing inner rule: {}", i));
                            let struct_doc = inner.generate_struct(i, out, arena);
                            out.push(struct_doc);
                            chain![arena;
                                i,
                                ":",
                                arena.line(),
                                "Option<",
                                inflector::cases::pascalcase::to_pascal_case(i),
                                "<'i>",
                                ">",
                            ].group()
                        }
                        Elem::Ident(i) => {
                            let inner = match inner
                                .iter()
                                .find(|rule| rule.name == i)
                                .unwrap_or_else(|| panic!("Missing inner rule: {}", i))
                                .production
                            {
                                Production::Leaf(i) => i,
                                _ => panic!(),
                            };
                            write_field(i, inner, arena)
                        }
                    }
                }), arena.line()),
            ]
            .nest(4),
            Production::Leaf(i) => write_field(i, i, arena),
        }
    }
}

fn elem<'i, I>() -> impl Parser<I, Output = Elem<'i>>
where
    I: RangeStream<Token = char, Range = &'i str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    let ident =
        || range::take_while1(|c: char| c.is_alphanumeric() || c == '_').expected("identifier");
    between(token('['), token(']'), ident())
        .map(Elem::Optional)
        .or(ident().map(Elem::Ident))
}

fn production<'i, I>() -> impl Parser<I, Output = Production<'i>>
where
    I: RangeStream<Token = char, Range = &'i str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        elem(),
        optional((
            token(' '),
            many1(elem().skip(optional(token(' ')))),
            spaces(),
            many(rule().skip(spaces())),
        )),
        optional(token(' ')),
    )
        .map(|(first, elems, _)| match elems {
            Some((_, elems, _, inner)) => {
                let mut elems: Vec<_> = elems;
                elems.insert(0, first);
                Production::Seq(elems, inner)
            }
            None => match first {
                Elem::Ident(i) | Elem::Optional(i) => Production::Leaf(i),
            },
        })
}

combine::parser! {
fn rule['i, I]()(I) -> Rule<'i>
where [
    I: RangeStream<Token = char, Range = &'i str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
]
{
    (
        range::take_while1(|c: char| c != '=' && c != '(').map(|s| { eprintln!("{:?}", s); str::trim(s) }),
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
    )
        .map(|(name, version, _, _, production)| Rule { name, version, production })
}
}

fn main() -> io::Result<()> {
    {
        let mut out = io::BufWriter::new(fs::File::create("src/parser.rs")?);
        for (i, input) in serde_json::from_str::<Vec<String>>(&fs::read_to_string(
            "kafka_request_responses.json",
        )?)
        .unwrap()
        .into_iter()
        .enumerate()
        {
            eprintln!("Input {}: {}", i, input);
            let (rule, _) = rule()
                .easy_parse(&input[..])
                .map_err(|err| err.map_position(|p| p.translate_position(&input[..])))
                .unwrap_or_else(|err| panic!("{}", err));

            eprintln!("{:#?}", rule);
            let mut s = Vec::new();
            rule.generate_fn(&mut s)?;
            write!(out, "{}", str::from_utf8(&s).unwrap())?;
        }
    }

    {
        println!("cargo:rerun-if-changed=kafka_errors.txt");
        let kafka_errors = fs::read_to_string("kafka_errors.txt")?;
        let mut out = io::BufWriter::new(fs::File::create("src/error.rs")?);

        writeln!(out, "#[derive(Eq, PartialEq, Debug)]")?;
        writeln!(out, "pub enum ErrorCode {{")?;
        let iter = kafka_errors.lines().map(|line| {
            let mut s = line.split('\t');
            let name = s.next().expect("name");
            let number = s.next().expect("number");
            let _retriable = s.next().expect("retriable");
            let doc = s.next();
            (name, number, doc)
        });
        for (name, number, doc) in iter {
            if let Some(doc) = doc {
                writeln!(out, "    /// {}", doc)?;
            }
            writeln!(
                out,
                "    {name} = {number},",
                name = inflector::cases::pascalcase::to_pascal_case(name),
                number = number
            )?;
        }
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

    Ok(())
}
