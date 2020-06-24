use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LOCK_SERVER: &'static str = "lock-server";

    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux lock-server
    /// (alias: lock)
    /// ```
    pub fn lock_server(&mut self) -> Result<Output, Error> {
        let output = self.command(TmuxInterface::LOCK_SERVER, &[])?;
        Ok(output)
    }
}
