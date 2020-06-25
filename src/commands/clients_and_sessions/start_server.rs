use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const START_SERVER: &'static str = "start-server";

    /// Start the tmux server, if not already running, without creating any sessions
    ///
    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux start-server
    /// (alias: start)
    /// ```
    pub fn start_server(&mut self) -> Result<Output, Error> {
        let output = self.command(TmuxInterface::START_SERVER, &[""])?;
        Ok(output)
    }
}