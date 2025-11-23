use anyhow::{Result, anyhow};
use codecrafters_shell::run_repl;
use std::io::Cursor;

#[derive(Debug)]
struct Output {
    stdout: String,
    stderr: String,
}

fn run_shell(input: &str) -> Result<Output> {
    let stdin = Cursor::new(input.as_bytes());
    let mut stdout = Cursor::new(Vec::new());
    let mut stderr = Cursor::new(Vec::new());

    run_repl(stdin, &mut stdout, &mut stderr)?;

    Ok(Output {
        stdout: String::from_utf8(stdout.into_inner())?,
        stderr: String::from_utf8(stderr.into_inner())?,
    })
}

pub fn check_stdout(input: &str, expected: &str) -> Result<()> {
    let output = run_shell(input)?;
    // assert!(output.stderr.is_empty());

    if output.stdout == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "stdout does not match expectation: '{expected}'. Output is: {output:?}"
        ))
    }
}

pub fn check_stderr(input: &str, expected: &str) -> Result<()> {
    let output = run_shell(input)?;
    // assert!(output.stdout.is_empty());

    if output.stderr == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "stderr does not match expectation: '{expected}'. Output is: {output:?}"
        ))
    }
}
