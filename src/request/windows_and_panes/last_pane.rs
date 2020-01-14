use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const LAST_PANE: &'static str = "last-pane";

    /// Select the last (previously selected) pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux last-pane [-deZ] [-t target-window]
    /// (alias: lastp)
    /// ```
    pub fn last_pane(
        &mut self,
        disable: Option<bool>,
        enable: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if disable.unwrap_or(false) {
            args.push(d_KEY);
        }
        if enable.unwrap_or(false) {
            args.push(e_KEY);
        }
        if keep_zoomed.unwrap_or(false) {
            args.push(e_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LAST_PANE, &args)?;
        Ok(output)
    }
}
