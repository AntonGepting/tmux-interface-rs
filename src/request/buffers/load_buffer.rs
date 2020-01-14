use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Buffers" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS)
impl<'a> TmuxInterface<'a> {
    const LOAD_BUFFER: &'static str = "load-buffer";

    /// Load the contents of the specified paste buffer from path.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux load-buffer [-b buffer-name] path
    /// (alias: loadb)
    /// ```
    pub fn load_buffer(&mut self, buffer_name: Option<&str>, path: &str) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::LOAD_BUFFER, &args)?;
        Ok(output)
    }
}
