use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
    pub fn rename_window<T: Display>(
        &mut self,
        target_window: Option<&T>,
        new_name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_WINDOW, &args)?;
        Ok(output)
    }
}
