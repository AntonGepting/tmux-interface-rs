use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const LAST_WINDOW: &'static str = "last-window";

    /// Select the last (previously selected) window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux last-window [-t target-session]
    /// (alias: last)
    /// ```
    pub fn last_window(&mut self, target_session: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LAST_WINDOW, &args)?;
        Ok(output)
    }
}
