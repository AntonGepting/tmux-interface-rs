use crate::error::Error;
use crate::tmux_interface::*;
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
    pub fn send_prefix(
        &mut self,
        secondary: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if secondary.unwrap_or(false) {
            args.push(_2_KEY);
        }
        if let Some(target_pane) = target_pane {
            args.extend_from_slice(&[t_KEY, &target_pane])
        }
        let output = self.command(TmuxInterface::SEND_PREFIX, &args)?;
        Ok(output)
    }
}
