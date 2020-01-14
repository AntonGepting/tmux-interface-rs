use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const KILL_SESSION: &'static str = "kill-session";

    /// Destroy the given session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-session [-aC] [-t target-session]
    /// ```
    pub fn kill_session(
        &mut self,
        all: Option<bool>,
        clear_alerts: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if clear_alerts.unwrap_or(false) {
            args.push(C_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_SESSION, &args)?;
        Ok(output)
    }
}
