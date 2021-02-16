use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LOCK_SESSION: &'static str = "lock-session";
    #[cfg(feature = "use_cmd_alias")]
    const LOCK_SESSION: &'static str = "locks";

    /// Lock all clients attached to `target-session`
    /// # Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// tmux lock-session [-t target-session]
    /// (alias: locks)
    /// ```
    pub fn lock_session(&mut self, target_session: Option<&'a str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        let output = self.command(TmuxInterface::LOCK_SESSION, &args)?;
        Ok(output)
    }
}
