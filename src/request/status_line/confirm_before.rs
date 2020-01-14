use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Status line" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE)
impl<'a> TmuxInterface<'a> {
    const CONFIRM_BEFORE: &'static str = "confirm-before";

    /// # Manual
    ///
    /// ```text
    /// tmux confirm-before [-p prompt] [-t target-client] command
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
        let output = self.subcommand(TmuxInterface::CONFIRM_BEFORE, &args)?;
        Ok(output)
    }
}
