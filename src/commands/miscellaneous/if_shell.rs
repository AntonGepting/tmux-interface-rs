use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
use std::marker::PhantomData;

/// Structure for conditional commands executing
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux if-shell [-b] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.6:
/// ```text
/// tmux if-shell shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux if-shell shell-command command
/// (alias: if)
/// ```
#[derive(Default, Debug)]
pub struct IfShell<'a> {
    /// [-b] - run in the background
    #[cfg(feature = "tmux_1_8")]
    pub backgroud: Option<bool>,
    /// [-F -] not execute but considered success if neither empty nor zero
    #[cfg(feature = "tmux_2_0")]
    pub not_execute: Option<bool>,
    /// [-t target-pane -] specify the target-pane
    #[cfg(feature = "tmux_1_8")]
    pub target_pane: Option<&'a str>,
    // shell-command
    //pub shell_command: &'a str,
    // command
    //pub command: &'a str,
    /// [command] - specify the second command
    #[cfg(feature = "tmux_1_6")]
    pub second_command: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    pub _phantom: PhantomData<&'a str>,
}

impl<'a> IfShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct IfShellBuilder<'a> {
    #[cfg(feature = "tmux_1_8")]
    pub backgroud: Option<bool>,
    #[cfg(feature = "tmux_2_0")]
    pub not_execute: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub target_pane: Option<&'a str>,
    //pub shell_command: &'a str,
    //pub command: &'a str,
    #[cfg(feature = "tmux_1_6")]
    pub second_command: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    pub _phantom: PhantomData<&'a str>,
}

impl<'a> IfShellBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn backgroud(&mut self) -> &mut Self {
        self.backgroud = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn not_execute(&mut self) -> &mut Self {
        self.not_execute = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn second_command(&mut self, second_command: &'a str) -> &mut Self {
        self.second_command = Some(second_command);
        self
    }

    pub fn build(&self) -> IfShell<'a> {
        IfShell {
            #[cfg(feature = "tmux_1_8")]
            backgroud: self.backgroud,
            #[cfg(feature = "tmux_2_0")]
            not_execute: self.not_execute,
            #[cfg(feature = "tmux_1_8")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_1_6")]
            second_command: self.second_command,
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
            _phantom: PhantomData,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const IF_SHELL: &'static str = "if-shell";

    /// # Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
    /// (alias: if)
    /// ```
    ///
    /// tmux ^1.8:
    /// ```text
    /// tmux if-shell [-b] [-t target-pane] shell-command command [command]
    /// (alias: if)
    /// ```
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux if-shell shell-command command [command]
    /// (alias: if)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux if-shell shell-command command
    /// (alias: if)
    /// ```
    pub fn if_shell(
        &mut self,
        if_shell: Option<&IfShell>,
        shell_command: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        #[cfg(feature = "tmux_1_8")]
        if let Some(if_shell) = if_shell {
            #[cfg(feature = "tmux_1_8")]
            if if_shell.backgroud.unwrap_or(false) {
                args.push(b_KEY);
            }
            #[cfg(feature = "tmux_2_0")]
            if if_shell.not_execute.unwrap_or(false) {
                args.push(F_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if let Some(target_pane) = if_shell.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
        }
        args.push(shell_command);
        args.push(command);
        #[cfg(feature = "tmux_1_6")]
        if let Some(if_shell) = if_shell {
            #[cfg(feature = "tmux_1_6")]
            if let Some(second_command) = if_shell.second_command {
                args.push(&second_command)
            }
        }
        let output = self.command(TmuxInterface::IF_SHELL, &args)?;
        Ok(output)
    }
}
