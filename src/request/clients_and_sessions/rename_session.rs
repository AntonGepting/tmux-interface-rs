use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const RENAME_SESSION: &'static str = "rename-session";

    /// Rename the session to `new-name`
    ///
    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux rename-session [-t target-session] new-name
    /// (alias: rename)
    /// ```
    pub fn rename_session(
        &mut self,
        target_session: Option<&'a str>,
        new_name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_SESSION, &args)?;
        Ok(output)
    }
}
