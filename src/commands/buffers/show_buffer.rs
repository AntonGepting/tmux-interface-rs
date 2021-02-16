use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const SHOW_BUFFER: &'static str = "show-buffer";
    #[cfg(feature = "use_cmd_alias")]
    const SHOW_BUFFER: &'static str = "showb";

    /// Display the contents of the specified buffer.
    ///
    /// # Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux show-buffer [-b buffer-name]
    /// (alias: showb)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux show-buffer [-b buffer-index] [-t target-session]
    /// (alias: showb)
    /// ```
    pub fn show_buffer(&mut self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        let output = self.command(TmuxInterface::SHOW_BUFFER, &args)?;
        Ok(output)
    }
}
