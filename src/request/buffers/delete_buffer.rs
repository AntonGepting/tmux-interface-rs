use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const DELETE_BUFFER: &'static str = "delete-buffer";

    /// Delete the buffer named buffer-name, or the most recently added automatically named buffer
    /// if not specified.
    ///
    /// # Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux delete-buffer [-b buffer-name]
    /// (alias: deleteb)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux delete-buffer [-b buffer-index]
    /// (alias: deleteb)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux delete-buffer [-b buffer-index] [-t target-session]
    /// (alias: deleteb)
    /// ```
    pub fn delete_buffer(&mut self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::DELETE_BUFFER, &args)?;
        Ok(output)
    }
}
