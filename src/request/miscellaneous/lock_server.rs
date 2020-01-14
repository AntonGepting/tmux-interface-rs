use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Miscellaneous" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS)
impl<'a> TmuxInterface<'a> {
    const LOCK_SERVER: &'static str = "lock-server";

    /// # Manual
    ///
    /// ```text
    /// tmux lock-server
    /// (alias: lock)
    /// ```
    pub fn lock_server(&mut self) -> Result<Output, Error> {
        let output = self.subcommand(TmuxInterface::LOCK_SERVER, &[])?;
        Ok(output)
    }
}
