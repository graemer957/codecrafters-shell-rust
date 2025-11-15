use anyhow::Result;
use std::{fmt::Display, io::Write, str::FromStr};

pub enum Builtin {
    Echo,
    Exit,
    Type,
}

impl FromStr for Builtin {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Self::Echo),
            "exit" => Ok(Self::Exit),
            "type" => Ok(Self::Type),
            _ => Err(()),
        }
    }
}

pub fn exit(status: i32) -> ! {
    std::process::exit(status);
}

pub fn echo<W, T>(mut writer: W, iterator: T) -> Result<()>
where
    W: Write,
    T: Iterator,
    T::Item: Display,
{
    let mut peekable = iterator.peekable();

    while let Some(word) = peekable.next() {
        write!(writer, "{word}")?;

        if peekable.peek().is_some() {
            write!(writer, " ")?;
        }
    }

    writeln!(writer)?;
    Ok(())
}
