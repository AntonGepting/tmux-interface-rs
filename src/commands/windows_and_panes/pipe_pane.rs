use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Pipe output sent by the program in target-pane to a shell command or vice versa
///
/// # Manual
///
/// tmux ^2.7:
/// ```text
/// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux pipe-pane [-o] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux ^1.1:
/// ```text
/// tmux pipe-pane [-o] [-t target-pane] [command]
/// (alias: pipep)
/// ```
#[derive(Default, Debug)]
pub struct PipePane<'a> {
    /// [-I] - stdin is connected
    #[cfg(feature = "tmux_2_7")]
    pub stdout: Option<bool>,
    /// [-O] - stdout is connected
    #[cfg(feature = "tmux_2_7")]
    pub stdin: Option<bool>,
    /// [-o] - only open a new pipe if no previous pipe exists
    #[cfg(feature = "tmux_1_1")]
    pub open: Option<bool>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_1_1")]
    pub target_pane: Option<&'a str>,
    /// [shell-command] - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a> PipePane<'a> {
    pub fn new() -> PipePane<'a> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct PipePaneBuilder<'a> {
    #[cfg(feature = "tmux_2_7")]
    pub stdout: Option<bool>,
    #[cfg(feature = "tmux_2_7")]
    pub stdin: Option<bool>,
    #[cfg(feature = "tmux_1_1")]
    pub open: Option<bool>,
    #[cfg(feature = "tmux_1_1")]
    pub target_pane: Option<&'a str>,
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a> PipePaneBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn stdout(&mut self) -> &mut Self {
        self.stdout = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn stdin(&mut self) -> &mut Self {
        self.stdin = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn open(&mut self) -> &mut Self {
        self.open = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> PipePane<'a> {
        PipePane {
            #[cfg(feature = "tmux_2_7")]
            stdout: self.stdout,
            #[cfg(feature = "tmux_2_7")]
            stdin: self.stdin,
            #[cfg(feature = "tmux_1_1")]
            open: self.open,
            #[cfg(feature = "tmux_1_1")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_1_2")]
            shell_command: self.shell_command,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const PIPE_PANE: &'static str = "pipe-pane";

    /// Pipe output sent by the program in target-pane to a shell command or vice versa
    ///
    /// # Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux pipe-pane [-o] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    ///
    /// tmux ^1.1:
    /// ```text
    /// tmux pipe-pane [-o] [-t target-pane] [command]
    /// (alias: pipep)
    /// ```
    pub fn pipe_pane(
        &mut self,
        pipe_pane: Option<&PipePane>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(pipe_pane) = pipe_pane {
            #[cfg(feature = "tmux_2_7")]
            if pipe_pane.stdout.unwrap_or(false) {
                args.push(I_KEY);
            }
            #[cfg(feature = "tmux_2_7")]
            if pipe_pane.stdin.unwrap_or(false) {
                args.push(O_KEY);
            }
            #[cfg(feature = "tmux_1_1")]
            if pipe_pane.open.unwrap_or(false) {
                args.push(o_KEY);
            }
            #[cfg(feature = "tmux_1_1")]
            if let Some(target_pane) = pipe_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(feature = "tmux_1_2")]
            if let Some(shell_command) = pipe_pane.shell_command {
                args.push(&shell_command)
            }
        }
        let output = self.command(TmuxInterface::PIPE_PANE, &args)?;
        Ok(output)
    }
}
