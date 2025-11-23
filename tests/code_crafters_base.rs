use crate::common::{check_stderr, check_stdout};
use anyhow::Result;

mod common;

#[test]
fn print_a_prompt_oo8() -> Result<()> {
    check_stdout("", "$ ")
}

#[test]
fn handle_invalid_commands_cz2() -> Result<()> {
    check_stderr(
        "invalid_blueberry_command",
        "invalid_blueberry_command: not found\n",
    )
}
