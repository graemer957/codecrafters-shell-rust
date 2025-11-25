use anyhow::{Result, anyhow};
use codecrafters_shell::run_repl;
use std::io::Cursor;

/// The prompt we expect the user to see, duplicated here so we are testing **behaviour**
pub const EXPECTED_PROMPT: &str = "$ ";

#[derive(Debug)]
pub struct Output {
    /// Contents of what was written to `stdout`
    pub stdout: String,

    /// Contents of what was written to `stderr`
    pub stderr: String,

    /// Exit code from the REPL run
    pub code: i32,
}

pub fn run_shell(input: &str) -> Result<Output> {
    let stdin = Cursor::new(input.as_bytes());
    let mut stdout = Cursor::new(Vec::new());
    let mut stderr = Cursor::new(Vec::new());

    let code = run_repl(stdin, &mut stdout, &mut stderr)?;

    Ok(Output {
        stdout: String::from_utf8(stdout.into_inner())?,
        stderr: String::from_utf8(stderr.into_inner())?,
        code,
    })
}

pub fn check_stdout(input: &str, expected: &str) -> Result<()> {
    let output = run_shell(input)?;

    if strip_prompts(&output.stdout) == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "stdout does not match expectation: '{expected}'. Output is: {output:?}"
        ))
    }
}

fn strip_prompts(stdout: &str) -> String {
    let cleaned = stdout
        .lines()
        .map(|line| line.strip_prefix(EXPECTED_PROMPT).unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");

    // Preserve trailing newline if original had one
    if stdout.ends_with('\n') {
        format!("{cleaned}\n")
    } else {
        cleaned
    }
}

pub fn check_stderr(input: &str, expected: &str) -> Result<()> {
    let output = run_shell(input)?;

    if output.stderr == expected {
        Ok(())
    } else {
        Err(anyhow!(
            "stderr does not match expectation: '{expected}'. Output is: {output:?}"
        ))
    }
}
