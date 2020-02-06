use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;

impl<'a> TmuxInterface<'a> {
    const LIST_WINDOWS: &'static str = "list-windows";

    // XXX: better return type
    /// List windows on the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-windows [-a] [-F format] [-t target-session]
    /// (alias: lsw)
    /// ```
    pub fn list_windows(
        &mut self,
        all: Option<bool>,
        format: Option<&str>,
        target_session: Option<&TargetSession>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        if let Some(target_session) = target_session {
            s = target_session.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_WINDOWS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
