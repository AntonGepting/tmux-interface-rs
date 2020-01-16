use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_BUFFER: &'static str = "show-buffer";

    /// Display the contents of the specified buffer.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux show-buffer [-b buffer-name]
    /// (alias: showb)
    /// ```
    pub fn show_buffer(&mut self, buffer_name: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SHOW_BUFFER, &args)?;
        Ok(output)
    }
}
