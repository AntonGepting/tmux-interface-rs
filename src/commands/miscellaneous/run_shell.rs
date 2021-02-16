use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const RUN_SHELL: &'static str = "run-shell";
    #[cfg(feature = "use_cmd_alias")]
    const RUN_SHELL: &'static str = "run";

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
    pub fn run_shell(
        &mut self,
        backgroud: Option<bool>,
        target_pane: Option<&str>,
        shell_command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if backgroud.unwrap_or(false) {
            args.push(b_KEY);
        }
        if let Some(target_pane) = target_pane {
            args.extend_from_slice(&[t_KEY, &target_pane])
        }
        args.push(shell_command);
        let output = self.command(TmuxInterface::RUN_SHELL, &args)?;
        Ok(output)
    }
}
