use crate::parser::Parser;
use anyhow::Result;
use std::io::{BufRead, Write};

/// Library for our shell
///
/// # High level logic
///
/// - `prompt::display()` shows the shell prompt
/// - `Parser` waits for user input and returns a `Command`
/// - `Executor` executes the `Command`
/// - Unrecoverable errors exit the shell (eg, failing to write the prompt)
/// - Recoverable errors are shown to the user
mod builtins;
pub mod command;
pub mod executor;
pub mod parser;
pub mod prompt;
mod utils;

/// Begin the REPL loop of the shell. The flow is prompt > parse > execute, repeat. The only
/// special case is for exiting the shell, which is handled here.
///
/// Recoverable user-errors are written to stderr within parsing or execution.
///
/// # Errors
///
/// Will return `Err` if an **unrecoverable** error (eg, I/O failure) occurs.
pub fn run_repl<I, O, E>(mut stdin: I, mut stdout: O, mut stderr: E) -> Result<i32>
where
    I: BufRead,
    O: Write,
    E: Write,
{
    loop {
        prompt::display(&mut stdout)?;

        let command = Parser::parse(&mut stderr, &mut stdin)?;
        if let command::Command::Exit { code } = command {
            return Ok(code);
        }
        executor::execute(&mut stdout, &mut stderr, command)?;
    }
}
