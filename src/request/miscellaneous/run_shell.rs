use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Miscellaneous" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS)
impl<'a> TmuxInterface<'a> {
    const RUN_SHELL: &'static str = "run-shell";

    /// # Manual
    ///
    /// ```text
    /// tmux run-shell [-b] [-t target-pane] shell-command
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
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(shell_command);
        let output = self.subcommand(TmuxInterface::RUN_SHELL, &args)?;
        Ok(output)
    }
}
