use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const NEXT_WINDOW: &'static str = "next-window";
    #[cfg(feature = "use_cmd_alias")]
    const NEXT_WINDOW: &'static str = "next";

    /// Move to the next window in the session
    ///
    /// # Manual
    ///
    /// tmux ^0.9:
    /// ```text
    /// tmux next-window [-a] [-t target-session]
    /// (alias: next)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux next-window [-t target-session]
    /// (alias: next)
    /// ```
    pub fn next_window(
        &mut self,
        alert: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.command(TmuxInterface::NEXT_WINDOW, &args)?;
        Ok(output)
    }
}
