use anyhow::Result;
use std::{fmt::Display, io::Write};

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
