use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const KILL_PANE: &'static str = "kill-pane";

    /// Destroy the given pane
    ///
    /// # Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// tmux kill-pane [-a] [-t target-pane]
    /// (alias: killp)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux kill-pane [-t target-pane]
    /// (alias: killp)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux kill-pane [-p pane-index] [-t target-window]
    /// (alias: killp)
    /// ```
    pub fn kill_pane<T: Display>(
        &mut self,
        all: Option<bool>,
        target_pane: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let target;
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(target_pane) = target_pane {
            target = target_pane.to_string();
            args.extend_from_slice(&[t_KEY, &target])
        }
        let output = self.subcommand(TmuxInterface::KILL_PANE, &args)?;
        Ok(output)
    }
}
