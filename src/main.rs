use anyhow::Result;
use codecrafters_shell::{executor, parser::Parser, prompt};
use std::io;

// Clippy suggest locking `stdin` each iteration of the loop, which does not feel correct to me
#[allow(clippy::significant_drop_tightening)]
fn main() -> Result<()> {
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let mut stdin = io::stdin().lock();

    loop {
        prompt::display(&mut stdout)?;

        // Recoverable errors are written to stderr within parse/execute.
        // Unrecoverable errors (I/O failures) propagate and exit the shell.
        let command = Parser::parse(&mut stderr, &mut stdin)?;
        executor::execute(&mut stdout, &mut stderr, command)?;
    }
}
