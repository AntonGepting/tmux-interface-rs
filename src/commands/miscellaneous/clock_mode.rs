use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const CLOCK_MODE: &'static str = "clock-mode";

    /// # Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux clock-mode [-t target-pane]
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux clock-mode [-t target-window]
    /// ```
    pub fn clock_mode(&mut self, target_pane: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_pane) = target_pane {
            args.extend_from_slice(&[t_KEY, &target_pane])
        }
        let output = self.command(TmuxInterface::CLOCK_MODE, &args)?;
        Ok(output)
    }
}
