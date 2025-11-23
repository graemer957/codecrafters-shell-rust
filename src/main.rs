use anyhow::Result;
use codecrafters_shell::run_repl;
use std::io;

fn main() -> Result<()> {
    let code = run_repl(io::stdin().lock(), io::stdout().lock(), io::stderr().lock())?;
    std::process::exit(code)
}
