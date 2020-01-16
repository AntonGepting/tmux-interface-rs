use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const KILL_SERVER: &'static str = "kill-server";

    /// Kill the tmux server and clients and destroy all sessions
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-server
    /// ```
    pub fn kill_server(&mut self) -> Result<Output, Error> {
        let output = self.subcommand(TmuxInterface::KILL_SERVER, &[""])?;
        Ok(output)
    }
}
