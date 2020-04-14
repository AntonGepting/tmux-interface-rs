use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LOCK_SESSION: &'static str = "lock-session";

    /// Lock all clients attached to `target-session`
    /// # Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// tmux lock-session [-t target-session]
    /// (alias: locks)
    /// ```
    pub fn lock_session(
        &mut self,
        target_session: Option<&'a TargetSession<'a>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_session) = target_session {
            s = target_session.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LOCK_SESSION, &args)?;
        Ok(output)
    }
}
