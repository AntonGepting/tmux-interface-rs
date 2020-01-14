use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const LOCK_CLIENT: &'static str = "lock-client";

    /// Lock `target-client`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux lock-client [-t target-client]
    /// (alias: lockc)
    /// ```
    pub fn lock_client(&mut self, target_client: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LOCK_CLIENT, &args)?;
        Ok(output)
    }
}
