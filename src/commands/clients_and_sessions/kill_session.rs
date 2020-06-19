use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const KILL_SESSION: &'static str = "kill-session";

    /// Destroy the given session
    ///
    /// # Manual
    ///
    /// tmux ^2.2:
    /// ```text
    /// tmux kill-session [-aC] [-t target-session]
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux kill-session [-a] [-t target-session]
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux kill-session [-t target-session]
    /// ```
    pub fn kill_session(
        &mut self,
        all: Option<bool>,
        clear_alerts: Option<bool>,
        target_session: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if clear_alerts.unwrap_or(false) {
            args.push(C_KEY);
        }
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        let output = self.subcommand(TmuxInterface::KILL_SESSION, &args)?;
        Ok(output)
    }
}
