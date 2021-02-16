use crate::error::Error;
use crate::tmux_interface::*;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LIST_WINDOWS: &'static str = "list-windows";
    #[cfg(feature = "use_cmd_alias")]
    const LIST_WINDOWS: &'static str = "lsw";

    // XXX: better return type
    /// List windows on the server
    ///
    /// # Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux list-windows [-a] [-F format] [-t target-session]
    /// (alias: lsw)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux list-windows [-a] [-t target-session]
    /// (alias: lsw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-windows [-t target-session]
    /// (alias: lsw)
    /// ```
    pub fn list_windows(
        &mut self,
        all: Option<bool>,
        format: Option<&str>,
        target_session: Option<&'a str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        let output = self.command(TmuxInterface::LIST_WINDOWS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
