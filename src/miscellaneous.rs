use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;


/// # Manual
///
/// ```text
/// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
#[derive(Default)]
pub struct IfShell<'a> {
    pub backgroud: Option<bool>,                // [-b]
    pub not_execute: Option<bool>,              // [-F]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub shell_command: &'a str,                 // shell-command
    pub first_command: &'a str,                 // command
    pub second_command: Option<&'a str>         // [command]
}

impl<'a> IfShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}



/// Miscellaneous
impl<'a> TmuxInterface<'a> {


    const CLOCK_MODE: &'static str = "clock-mode";
    const IF_SHELL: &'static str = "if-shell";
    const LOCK_SERVER: &'static str = "lock-server";
    const RUN_SHELL: &'static str = "run-shell";
    const WAIT_FOR: &'static str = "wait-for";


    /// # Manual
    ///
    /// ```text
    /// tmux clock-mode [-t target-pane]
    /// ```
    pub fn clock_mode(&self, target_pane: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::CLOCK_MODE, &args)?;
        Ok(output.status.success())
    }


    /// # Manual
    ///
    /// ```text
    /// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
    /// (alias: if)
    /// ```
    pub fn if_shell(&self, if_shell: &IfShell) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if if_shell.backgroud.unwrap_or(false) { args.push(b_KEY); }
        if if_shell.not_execute.unwrap_or(false) { args.push(F_KEY); }
        if_shell.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(if_shell.shell_command);
        args.push(if_shell.first_command);
        if_shell.second_command.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::IF_SHELL, &args)?;
        Ok(output.status.success())
    }


    /// # Manual
    ///
    /// ```text
    /// tmux lock-server
    /// (alias: lock)
    /// ```
    pub fn lock_server(&self) -> Result<bool, TmuxInterfaceError> {
        let output = self.subcommand(TmuxInterface::LOCK_SERVER, &[])?;
        Ok(output.status.success())
    }


    /// # Manual
    ///
    /// ```text
    /// tmux run-shell [-b] [-t target-pane] shell-command
    /// (alias: run)
    /// ```
    pub fn run_shell(&self,
                     backgroud: Option<bool>,
                     target_pane: Option<&str>,
                     shell_command: &str
                     ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if backgroud.unwrap_or(false) { args.push(b_KEY); }
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(shell_command);
        let output = self.subcommand(TmuxInterface::RUN_SHELL, &args)?;
        Ok(output.status.success())
    }


    /// # Manual
    ///
    /// ```text
    /// tmux wait-for [-L | -S | -U] channel
    /// (alias: wait)
    /// ```
    pub fn wait_for(&self,
                    lock: Option<bool>,
                    prevent_exit: Option<bool>,
                    unlock: Option<bool>,
                    channel: &str
                    ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if lock.unwrap_or(false) { args.push(L_KEY); }
        if prevent_exit.unwrap_or(false) { args.push(S_KEY); }
        if unlock.unwrap_or(false) { args.push(U_KEY); }
        args.push(channel);
        let output = self.subcommand(TmuxInterface::WAIT_FOR, &args)?;
        Ok(output.status.success())
    }


}
