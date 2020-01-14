use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const NEXT_WINDOW: &'static str = "next-window";

    /// Move to the next window in the session
    ///
    /// # Manual
    ///
    /// tmux next-window [-a] [-t target-session]
    /// (alias: next)
    pub fn next_window(
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
        let output = self.subcommand(TmuxInterface::NEXT_WINDOW, &args)?;
        Ok(output)
    }
}
