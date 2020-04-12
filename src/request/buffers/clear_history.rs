use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const CLEAR_HISTORY: &'static str = "clear-history";

    /// Remove and free the history for the specified pane.
    ///
    /// # Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux clear-history [-t target-pane]
    /// (alias: clearhist)
    /// ```
    ///
    /// tmux ^0.9:
    /// ```text
    /// tmux clear-history [-p pane-index] [-t target-window]
    /// (alias: clearhist)
    /// ```
    pub fn clear_history<T: Display>(&mut self, target_pane: Option<&T>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_pane) = target_pane {
            s = target_pane.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::CLEAR_HISTORY, &args)?;
        Ok(output)
    }
}
