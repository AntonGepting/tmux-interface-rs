use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Buffers" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS)
impl<'a> TmuxInterface<'a> {
    const SAVE_BUFFER: &'static str = "save-buffer";

    /// Save the contents of the specified paste buffer to path.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-name] path
    /// (alias: saveb)
    /// ```
    pub fn save_buffer(
        &mut self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        path: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SAVE_BUFFER, &args)?;
        Ok(output)
    }
}
