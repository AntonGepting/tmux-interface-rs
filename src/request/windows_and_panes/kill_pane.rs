use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const KILL_PANE: &'static str = "kill-pane";

    /// Destroy the given pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-pane [-a] [-t target-pane]
    /// (alias: killp)
    /// ```
    pub fn kill_pane(
        &mut self,
        all: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_PANE, &args)?;
        Ok(output)
    }
}
