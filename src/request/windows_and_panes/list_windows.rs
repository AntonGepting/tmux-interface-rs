use crate::error::Error;
use crate::tmux_interface::*;
//use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
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
        target_session: Option<&str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LIST_WINDOWS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
