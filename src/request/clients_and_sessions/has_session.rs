use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;

impl<'a> TmuxInterface<'a> {
    const HAS_SESSION: &'static str = "has-session";

    // XXX: better result return?
    /// Report if the specified session exist
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux has-session [-t target-session]
    /// (alias: has)
    /// ```
    pub fn has_session(
        &mut self,
        target_session: Option<&TargetSession<'a>>,
    ) -> Result<bool, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_session) = target_session {
            s = target_session.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::HAS_SESSION, &args)?;
        Ok(output.status.success())
    }
}
