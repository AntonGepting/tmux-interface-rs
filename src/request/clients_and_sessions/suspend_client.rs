use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const SUSPEND_CLIENT: &'static str = "suspend-client";

    /// Suspend a client by sending SIGTSTP (tty stop)
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux suspend-client [-t target-client]
    /// (alias: suspendc)
    /// ```
    pub fn suspend_client(&mut self, target_client: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SUSPEND_CLIENT, &args)?;
        Ok(output)
    }
}
