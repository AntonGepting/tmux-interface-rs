use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const RUN_SHELL: &'static str = "run-shell";

    /// # Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// tmux run-shell [-b] [-t target-pane] shell-command
    /// (alias: run)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux run-shell shell-command
    /// (alias: run)
    /// ```
    ///
    /// tmux ^1.1:
    /// ```text
    /// tmux run-shell command
    /// (alias: run)
    /// ```
    pub fn run_shell<T: Display>(
        &mut self,
        backgroud: Option<bool>,
        target_pane: Option<&T>,
        shell_command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if backgroud.unwrap_or(false) {
            args.push(b_KEY);
        }
        if let Some(target_pane) = target_pane {
            s = target_pane.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(shell_command);
        let output = self.subcommand(TmuxInterface::RUN_SHELL, &args)?;
        Ok(output)
    }
}
