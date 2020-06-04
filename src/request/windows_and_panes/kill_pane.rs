use crate::error::Error;
use crate::tmux_interface::*;
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
    pub fn kill_pane(
        &mut self,
        all: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(target_pane) = target_pane {
            args.extend_from_slice(&[t_KEY, &target_pane])
        }
        let output = self.subcommand(TmuxInterface::KILL_PANE, &args)?;
        Ok(output)
    }
}
