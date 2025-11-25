use crate::common::{check_stderr, check_stdout, run_shell};
use anyhow::Result;

mod common;

// These tests validate functionality the CodeCrafters test suite does not

#[test]
fn exit_invalid_code() -> Result<()> {
    let output = run_shell("exit invalid\n")?;
    assert_eq!(output.code, 0);

    Ok(())
}

#[test]
fn exit_with_code() -> Result<()> {
    let output = run_shell("exit 42\n")?;
    assert_eq!(output.code, 42);

    Ok(())
}

#[test]
fn type_no_arg() -> Result<()> {
    check_stderr("type\n", "no parameter passed for `type`\n")
}

#[test]
fn whitespace_only() -> Result<()> {
    check_stdout("\n", "$ ")
}
