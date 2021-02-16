use crate::error::Error;
use crate::tmux_interface::*;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const HAS_SESSION: &'static str = "has-session";
    #[cfg(feature = "use_cmd_alias")]
    const HAS_SESSION: &'static str = "has";

    // XXX: better result return?
    /// Report if the specified session exist
    ///
    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux has-session [-t target-session]
    /// (alias: has)
    /// ```
    pub fn has_session(&mut self, target_session: Option<&'a str>) -> Result<bool, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        let output = self.command(TmuxInterface::HAS_SESSION, &args)?;
        Ok(output.status.success())
    }
}
