use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_CLIENTS: &'static str = "list-clients";

    /// List all clients attached to the server
    ///
    /// # Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux list-clients [-F format] [-t target-session]
    /// (alias: lsc)
    ///
    /// ```
    /// tmux ^1.5:
    /// ```text
    /// tmux list-clients [-t target-session]
    /// (alias: lsc)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-clients
    /// (alias: lsc)
    /// ```
    pub fn list_clients(
        &mut self,
        format: Option<&str>,
        target_session: Option<&'a TargetSession<'a>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        if let Some(target_session) = target_session {
            s = target_session.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_CLIENTS, &args)?;
        Ok(output)
    }
}
