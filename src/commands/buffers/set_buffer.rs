use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SET_BUFFER: &'static str = "set-buffer";

    /// Set the contents of the specified buffer to data.
    ///
    /// # Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux set-buffer [-a] [-b buffer-name] [-n new-buffer-name] data
    /// (alias: setb)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux set-buffer [-b buffer-index] data
    /// (alias: setb)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux set-buffer [-b buffer-index] [-t target-session] data
    /// (alias: setb)
    /// ```
    pub fn set_buffer(
        &mut self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        new_buffer_name: Option<&str>,
        data: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        if let Some(s) = new_buffer_name {
            args.extend_from_slice(&[n_KEY, &s])
        }
        args.push(data);
        let output = self.command(TmuxInterface::SET_BUFFER, &args)?;
        Ok(output)
    }
}
