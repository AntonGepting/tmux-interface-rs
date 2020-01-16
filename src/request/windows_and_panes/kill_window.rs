use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const KILL_WINDOW: &'static str = "kill-window";

    /// Kill the current window or the window at target-window, removing it from any sessions
    /// to which it is linked
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    pub fn kill_window(
        &mut self,
        all: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_WINDOW, &args)?;
        Ok(output)
    }
}
