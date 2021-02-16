use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const CLEAR_HISTORY: &'static str = "clear-history";
    #[cfg(feature = "use_cmd_alias")]
    const CLEAR_HISTORY: &'static str = "clearhist";

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
    pub fn clear_history(&mut self, target_pane: Option<&'a str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_pane) = target_pane {
            args.extend_from_slice(&[t_KEY, &target_pane])
        }
        let output = self.command(TmuxInterface::CLEAR_HISTORY, &args)?;
        Ok(output)
    }
}
