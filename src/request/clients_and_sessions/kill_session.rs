use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

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
        target_session: Option<&'a TargetSession<'a>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if clear_alerts.unwrap_or(false) {
            args.push(C_KEY);
        }
        if let Some(target_session) = target_session {
            s = target_session.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_SESSION, &args)?;
        Ok(output)
    }
}
