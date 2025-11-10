use anyhow::{anyhow, Context, Result};
use std::{
    env,
    ffi::OsStr,
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process,
};

/// Searches for the executable in the paths from the `PATH` environment variable.
///
/// An executable is defined as (a) a file and (b) has the executable bit set.
///
/// # Returns
///
/// - `Ok(Some(PathBuf))` pointing to where the executable was found
/// - `Ok(None)` if the executable is not found (user typed wrong command)
/// - `Err(...)` is an unrecoverable error if `PATH` environment variable is not set
pub fn find_executable(name: &str) -> Result<Option<PathBuf>> {
    const EXECUTE_BIT: u32 = 0o111;

    let path_env = env::var_os("PATH").context("PATH environment variable not set")?;
    for mut path in env::split_paths(&path_env) {
        path.push(name);
        let Ok(metadata) = fs::metadata(&path) else {
            continue;
        };
        if metadata.is_file() && metadata.permissions().mode() & EXECUTE_BIT != 0 {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

pub fn run_program<E, T>(mut stderr: E, program: &Path, args: T) -> Result<()>
where
    E: Write,
    T: Iterator,
    T::Item: AsRef<OsStr>,
{
    // Originally I was using `program`, a fully qualified path to the executable, for
    // `Command::new`, but it seems that CodeCrafters requires arg0 to be the program name. I am
    // not sure what is technically correct for a shell
    let program_name = program
        .file_name()
        .ok_or_else(|| anyhow!("path should include the name of the executable"))?;

    // TODO: `ExitStatus` is ignored, but we could set `$?` in the future
    if let Err(err) = process::Command::new(program_name).args(args).status() {
        writeln!(stderr, "{err}")?;
    }

    Ok(())
}
