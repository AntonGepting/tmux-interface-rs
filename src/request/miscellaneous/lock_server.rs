use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

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
