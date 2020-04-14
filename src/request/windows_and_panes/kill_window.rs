use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const KILL_WINDOW: &'static str = "kill-window";

    /// Kill the current window or the window at target-window, removing it from any sessions
    /// to which it is linked
    ///
    /// # Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    pub fn kill_window<T: Display>(
        &mut self,
        all: Option<bool>,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s]);
        }
        let output = self.subcommand(TmuxInterface::KILL_WINDOW, &args)?;
        Ok(output)
    }
}
