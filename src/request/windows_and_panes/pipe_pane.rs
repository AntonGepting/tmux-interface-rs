use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Pipe output sent by the program in target-pane to a shell command or vice versa
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux pipe-pane [-o] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct PipePane<'a> {
    /// [-I] - stdin is connected
    pub stdout: Option<bool>,
    /// [-O] - stdout is connected
    pub stdin: Option<bool>,
    /// [-o] - only open a new pipe if no previous pipe exists
    pub open: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a> PipePane<'a> {
    pub fn new() -> PipePane<'a> {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const PIPE_PANE: &'static str = "pipe-pane";

    /// Pipe output sent by the program in target-pane to a shell command or vice versa
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux pipe-pane [-o] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn pipe_pane(&mut self, pipe_pane: Option<&PipePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(pipe_pane) = pipe_pane {
            if pipe_pane.stdout.unwrap_or(false) {
                args.push(I_KEY);
            }
            if pipe_pane.stdin.unwrap_or(false) {
                args.push(O_KEY);
            }
            if pipe_pane.open.unwrap_or(false) {
                args.push(o_KEY);
            }
            if let Some(s) = pipe_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = pipe_pane.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::PIPE_PANE, &args)?;
        Ok(output)
    }

    /// Pipe output sent by the program in target-pane to a shell command or vice versa
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux pipe-pane [-o] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn pipe_pane(
        &mut self,
        open: Option<bool>,
        target_pane: Option<&'a str>,
        shell_command: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if open.unwrap_or(false) {
            args.push(o_KEY);
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = shell_command {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::PIPE_PANE, &args)?;
        Ok(output)
    }
}
