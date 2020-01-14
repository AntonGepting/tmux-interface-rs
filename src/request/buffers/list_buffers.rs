use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Buffers" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS)
impl<'a> TmuxInterface<'a> {
    const LIST_BUFFERS: &'static str = "list-buffers";
    /// List the global buffers.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-buffers [-F format]
    /// (alias: lsb)
    /// ```
    pub fn list_buffers(&mut self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_BUFFERS, &args)?;
        Ok(output)
    }
}
