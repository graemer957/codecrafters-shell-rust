use crate::{builtins, command::Command, utils};
use anyhow::Result;
use std::{io::Write, path::PathBuf};

/// Takes the relevant action for the given `Command`
///
/// Writes success output to `stdout` and recoverable user errors, such as "command not found"
/// to `stderr`.
///
/// # Errors
///
/// Will return `Err` if there is an issue writing to `stdout` or `stderr`
pub fn execute<O, E>(mut stdout: O, mut stderr: E, command: Command) -> Result<()>
where
    O: Write,
    E: Write,
{
    match command {
        Command::Exit { status_code } => builtins::exit(status_code),
        Command::Echo { args } => builtins::echo(&mut stdout, args.iter())?,
        Command::Type { target } => {
            if Command::is_builtin(&target) {
                writeln!(stdout, "{target} is a shell builtin")?;
            } else {
                find_executable_and(&mut stderr, &target, |_, path| {
                    writeln!(stdout, "{target} is {}", path.display())?;

                    Ok(())
                })?;
            }
        }
        Command::External { program, args } => {
            find_executable_and(&mut stderr, &program, |stderr, path| {
                utils::run_program(stderr, path, args.iter())?;

                Ok(())
            })?;
        }
        Command::Noop => {}
    }

    Ok(())
}

fn find_executable_and<E, F>(mut stderr: E, name: &str, on_found: F) -> Result<()>
where
    E: Write,
    F: FnOnce(&mut E, &PathBuf) -> Result<()>,
{
    if let Some(path) = utils::find_executable(name)? {
        on_found(&mut stderr, &path)
    } else {
        writeln!(stderr, "{name}: not found")?;
        Ok(())
    }
}
