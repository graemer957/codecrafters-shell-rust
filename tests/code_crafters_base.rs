use crate::common::{EXPECTED_PROMPT, check_stderr, check_stdout, run_shell};
use anyhow::Result;

mod common;

#[test]
fn print_a_prompt_oo8() -> Result<()> {
    let output = run_shell("")?;
    assert_eq!(output.stdout, EXPECTED_PROMPT);

    Ok(())
}

#[test]
fn handle_invalid_commands_cz2() -> Result<()> {
    check_stderr(
        "invalid_blueberry_command\n",
        "invalid_blueberry_command: not found\n",
    )
}

#[test]
fn implement_a_repl_ff0() -> Result<()> {
    let input = concat!(
        "invalid_command_1\n",
        "invalid_command_2\n",
        "invalid_command_3\n",
        "invalid_command_4\n",
        "invalid_command_5\n"
    );
    let expected = concat!(
        "invalid_command_1: not found\n",
        "invalid_command_2: not found\n",
        "invalid_command_3: not found\n",
        "invalid_command_4: not found\n",
        "invalid_command_5: not found\n"
    );
    check_stderr(input, expected)
}

#[test]
fn implement_exit_pn5() -> Result<()> {
    let input = concat!("invalid_orange_command\n", "exit\n");
    let expected = "invalid_orange_command: not found\n";
    let output = run_shell(input)?;

    assert_eq!(output.stderr, expected);
    assert_eq!(output.code, 0);

    Ok(())
}

#[test]
fn implement_echo_iz3() -> Result<()> {
    let input = concat!(
        "echo apple blueberry\n",
        "echo raspberry orange blueberry\n"
    );
    let expected = concat!("apple blueberry\n", "raspberry orange blueberry\n");
    check_stdout(input, expected)
}

#[test]
fn implement_type_ez5() -> Result<()> {
    let input = concat!("type echo\n", "type exit\n", "type type\n");
    let expected = concat!(
        "echo is a shell builtin\n",
        "exit is a shell builtin\n",
        "type is a shell builtin\n"
    );
    check_stdout(input, expected)?;

    let input = concat!(
        "type invalid_pineapple_command\n",
        "type invalid_grape_command\n"
    );
    let expected = concat!(
        "invalid_pineapple_command: not found\n",
        "invalid_grape_command: not found\n"
    );
    check_stderr(input, expected)
}

#[test]
fn locate_executable_files_mg5() -> Result<()> {
    // NOTE:
    // 1. Assumes `cat`, `cp` and `mkdir` filesystem locations
    // 2. CodeCrafters injects custom binary in random directory to validate `PATH` environment
    //    variable searching, which we are skipping for sake of ease 😇
    let input = concat!("type cat\n", "type cp\n", "type mkdir\n");
    let expected = concat!(
        "cat is /usr/bin/cat\n",
        "cp is /usr/bin/cp\n",
        "mkdir is /usr/bin/mkdir\n"
    );
    check_stdout(input, expected)?;

    let input = concat!(
        "type invalid_orange_command\n",
        "type invalid_raspberry_command\n"
    );
    let expected = concat!(
        "invalid_orange_command: not found\n",
        "invalid_raspberry_command: not found\n"
    );
    check_stderr(input, expected)
}

#[test]
fn run_a_program_ip1() -> Result<()> {
    // NOTE:
    // 1. When running in the CodeCrafters CI, they have some custom executable's they place in
    // random locations to validate `PATH` searching. These custom executables then output the name
    // of the program invoked and arguments passed.
    //
    // Given the custom `PATH` they set, I consider this more of an end-to-end test and am choosing
    // to KISS (keep it simple silly) here
    //
    // 2. Our shell does **not** capture `stdout` or `stderr` for child processes, which I believe
    //    is the correct decision for TTY detection, buffering, performance and interactivity.
    //    Ergo, it is not possible for us to write a test to check for the output of an external
    //    command. Instead we can check there is no errors and subsequent commands work.

    let input = concat!("ls\n", "echo done\n");
    let output = run_shell(input)?;

    assert_eq!(output.stdout.replace(EXPECTED_PROMPT, ""), "done\n");
    assert!(output.stderr.is_empty());

    Ok(())
}
