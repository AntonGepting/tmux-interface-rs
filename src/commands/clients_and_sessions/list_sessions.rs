use crate::error::Error;
use crate::tmux_interface::*;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LIST_SESSIONS: &'static str = "list-sessions";
    #[cfg(feature = "use_cmd_alias")]
    const LIST_SESSIONS: &'static str = "ls";

    // XXX: better result return?
    /// List all sessions managed by the server
    /// # Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux list-sessions [-F format]
    /// (alias: ls)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-sessions
    /// (alias: ls)
    /// ```
    pub fn list_sessions(&mut self, format: Option<&str>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        let output = self.command(TmuxInterface::LIST_SESSIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
