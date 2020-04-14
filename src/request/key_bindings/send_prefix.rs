use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SEND_PREFIX: &'static str = "send-prefix";

    /// # Manual
    ///
    /// tmux ^1.6
    /// ```text
    /// tmux send-prefix [-2] [-t target-pane]
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux send-prefix [-t target-pane]
    /// ```
    pub fn send_prefix<T: Display>(
        &mut self,
        secondary: Option<bool>,
        target_pane: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if secondary.unwrap_or(false) {
            args.push(_2_KEY);
        }
        if let Some(target_pane) = target_pane {
            s = target_pane.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SEND_PREFIX, &args)?;
        Ok(output)
    }
}
