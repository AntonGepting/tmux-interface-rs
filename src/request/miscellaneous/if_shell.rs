use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for conditional commands executing
///
/// # Manual
///
/// ```text
/// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
#[derive(Default, Debug)]
pub struct IfShell<'a> {
    /// [-b] - run in the background
    pub backgroud: Option<bool>,
    /// [-F -] not execute but considered success if neither empty nor zero
    pub not_execute: Option<bool>,
    /// [-t target-pane -] specify the target-pane
    pub target_pane: Option<&'a str>,
    // shell-command
    //pub shell_command: &'a str,
    // command
    //pub command: &'a str,
    /// [command] - specify the second command
    pub second_command: Option<&'a str>,
}

impl<'a> IfShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Miscellaneous" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS)
impl<'a> TmuxInterface<'a> {
    const IF_SHELL: &'static str = "if-shell";

    /// # Manual
    ///
    /// ```text
    /// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
    /// (alias: if)
    /// ```
    pub fn if_shell(
        &mut self,
        if_shell: Option<&IfShell>,
        shell_command: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(if_shell) = if_shell {
            if if_shell.backgroud.unwrap_or(false) {
                args.push(b_KEY);
            }
            if if_shell.not_execute.unwrap_or(false) {
                args.push(F_KEY);
            }
            if let Some(s) = if_shell.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(shell_command);
        args.push(command);
        if let Some(if_shell) = if_shell {
            if let Some(s) = if_shell.second_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::IF_SHELL, &args)?;
        Ok(output)
    }
}
