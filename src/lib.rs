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
