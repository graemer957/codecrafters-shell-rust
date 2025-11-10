use anyhow::Result;
use std::io::Write;

/// Displays the prompt for the user
///
/// Does little right now, but could handle PS1, customisations and superuser, etc...
///
/// # Errors
///
/// Will return `Err` if there is an issue displaying the prompt
pub fn display<T>(mut writer: T) -> Result<()>
where
    T: Write,
{
    write!(writer, "$ ")?;
    writer.flush()?;

    Ok(())
}
