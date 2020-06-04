use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const RENAME_WINDOW: &'static str = "rename-window";

    /// Rename the current window, or the window at target-window if specified, to new-name
    ///
    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux rename-window [-t target-window] new-name
    /// (alias: renamew)
    /// ```
    pub fn rename_window(
        &mut self,
        target_window: Option<&'a str>,
        new_name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_window) = target_window {
            args.extend_from_slice(&[t_KEY, &target_window])
        }
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_WINDOW, &args)?;
        Ok(output)
    }
}
