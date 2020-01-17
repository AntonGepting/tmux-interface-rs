use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LOCK_SESSION: &'static str = "lock-session";

    /// Lock all clients attached to `target-session`
    /// # Manual
    ///
    /// ```text
    /// tmux lock-session [-t target-session]
    /// (alias: locks)
    /// ```
    pub fn lock_session(&mut self, target_session: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LOCK_SESSION, &args)?;
        Ok(output)
    }
}
