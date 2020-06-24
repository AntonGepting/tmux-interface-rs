use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_MESSAGES: &'static str = "show-messages";

    /// Show client messages or server information
    ///
    /// # Manual
    ///
    /// tmux ^2.2:
    /// ```text
    /// tmux show-messages [-JT] [-t target-client]
    /// (alias: showmsgs)
    /// ```
    ///
    /// tmux ^1.9:
    /// ```text
    /// tmux show-messages [-IJT] [-t target-client]
    /// (alias: showmsgs)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux show-messages [-t target-client]
    /// (alias: showmsgs)
    /// ```
    pub fn show_messages(
        &mut self,
        jobs: Option<bool>,
        terminal: Option<bool>,
        target_client: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if jobs.unwrap_or(false) {
            args.push(J_KEY);
        }
        if terminal.unwrap_or(false) {
            args.push(T_KEY);
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.command(TmuxInterface::SHOW_MESSAGES, &args)?;
        Ok(output)
    }
}
