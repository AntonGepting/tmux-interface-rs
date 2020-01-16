use crate::error::Error;
use crate::tmux_interface::*;
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
    pub fn unlink_window(
        &mut self,
        k: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if k.unwrap_or(false) {
            args.push(k_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s]);
        }
        let output = self.subcommand(TmuxInterface::UNLINK_WINDOW, &args)?;
        Ok(output)
    }
}
