use crate::command::Command;
use anyhow::{anyhow, Result};
use std::io::{BufRead, Write};

pub struct Parser;

impl Parser {
    /// Attempts to parse the users input into a `Command`
    ///
    /// Writes recoverable user errors, such as a missing parameter, to `stderr`
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is an issue writing to `stderr` or reading from `stdin`
    pub fn parse<E, I>(mut stderr: E, mut stdin: I) -> Result<Command>
    where
        E: Write,
        I: BufRead,
    {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => Ok(Command::Exit { status_code: 0 }),
            Ok(_) => {
                let line = input.trim();
                let mut tokens = line.split_whitespace();
                let Some(command) = tokens.next() else {
                    return Ok(Command::Noop);
                };

                match command {
                    "exit" => {
                        let status = tokens
                            .next()
                            .map_or(0, |token| token.parse::<i32>().unwrap_or_default());

                        Ok(Command::Exit {
                            status_code: status,
                        })
                    }
                    "echo" => Ok(Command::Echo {
                        args: tokens.map(std::string::ToString::to_string).collect(),
                    }),
                    "type" => {
                        let Some(command) = tokens.next() else {
                            writeln!(stderr, "no parameter passed for `type`")?;
                            return Ok(Command::Noop);
                        };

                        Ok(Command::Type {
                            target: command.to_string(),
                        })
                    }
                    _ => Ok(Command::External {
                        program: command.to_string(),
                        args: tokens.map(std::string::ToString::to_string).collect(),
                    }),
                }
            }
            Err(error) => Err(anyhow!("fatal: {error}")),
        }
    }
}
