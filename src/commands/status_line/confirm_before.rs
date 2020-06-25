use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const CONFIRM_BEFORE: &'static str = "confirm-before";

    /// # Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux confirm-before [-p prompt] [-t target-client] command
    /// (alias: confirm)
    /// ```
    ///
    /// tmux ^0.9:
    /// ```text
    /// tmux confirm-before [-t target-client] command
    /// (alias: confirm)
    /// ```
    pub fn confirm_before(
        &mut self,
        prompt: Option<&str>,
        target_client: Option<&str>,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = prompt {
            args.extend_from_slice(&[p_KEY, &s])
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(command);
        let output = self.command(TmuxInterface::CONFIRM_BEFORE, &args)?;
        Ok(output)
    }
}