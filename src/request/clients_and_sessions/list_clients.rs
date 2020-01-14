use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const LIST_CLIENTS: &'static str = "list-clients";

    /// List all clients attached to the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-clients [-F format] [-t target-session]
    /// (alias: lsc)
    /// ```
    pub fn list_clients(
        &mut self,
        format: Option<&str>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_CLIENTS, &args)?;
        Ok(output)
    }
}
