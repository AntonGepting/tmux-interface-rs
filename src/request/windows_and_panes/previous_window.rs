use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const PREVIOUS_WINDOW: &'static str = "previous-window";

    /// Move to the previous window in the session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux previous-window [-a] [-t target-session]
    /// (alias: prev)
    /// ```
    pub fn previous_window(
        &mut self,
        alert: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::PREVIOUS_WINDOW, &args)?;
        Ok(output)
    }
}
