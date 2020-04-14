use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
    pub fn clock_mode<T: Display>(&mut self, target_pane: Option<&'a T>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_pane) = target_pane {
            s = target_pane.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::CLOCK_MODE, &args)?;
        Ok(output)
    }
}
