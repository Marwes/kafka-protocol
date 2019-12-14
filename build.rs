use std::{
    fs,
    io::{self, Write},
};

fn main() -> io::Result<()> {
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
