use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const UNLINK_WINDOW: &'static str = "unlink-window";

    /// Unlink `target-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux unlink-window [-k] [-t target-window]
    /// (alias: unlinkw)
    /// ```
    pub fn unlink_window<T: Display>(
        &mut self,
        k: Option<bool>,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if k.unwrap_or(false) {
            args.push(k_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s]);
        }
        let output = self.subcommand(TmuxInterface::UNLINK_WINDOW, &args)?;
        Ok(output)
    }
}
