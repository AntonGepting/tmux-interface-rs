use crate::error::Error;
use crate::tmux_interface::*;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const LIST_SESSIONS: &'static str = "list-sessions";

    // XXX: better result return?
    /// List all sessions managed by the server
    /// # Manual
    ///
    /// ```text
    /// tmux list-sessions [-F format]
    /// (alias: ls)
    /// ```
    pub fn list_sessions(&mut self, format: Option<&str>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LIST_SESSIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
