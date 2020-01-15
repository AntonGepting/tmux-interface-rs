use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Reactivate a pane in which the command has exited
///
/// # Manual
///
/// tmux X.X
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux 2.6
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct RespawnPane<'a> {
    /// [-k] - any existing command is killed
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a> RespawnPane<'a> {
    pub fn new() -> RespawnPane<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const RESPAWN_PANE: &'static str = "respawn-pane";

    /// Reactivate a pane in which the command has exited
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
    /// (alias: respawnp)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
    /// (alias: respawnp)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn respawn_pane(&mut self, respawn_pane: Option<&RespawnPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(respawn_pane) = respawn_pane {
            if respawn_pane.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = respawn_pane.start_directory {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = respawn_pane.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            if let Some(s) = respawn_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = respawn_pane.shell_command {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_PANE, &args)?;
        Ok(output)
    }

    /// Reactivate a pane in which the command has exited
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
    /// (alias: respawnp)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
    /// (alias: respawnp)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn respawn_pane(
        &mut self,
        kill: Option<bool>,
        start_directory: Option<&'a str>,
        target_pane: Option<&'a str>,
        shell_command: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if kill.unwrap_or(false) {
            args.push(k_KEY);
        }
        if let Some(s) = start_directory {
            args.extend_from_slice(&[c_KEY, &s])
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = shell_command {
            args.push(s)
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_PANE, &args)?;
        Ok(output)
    }
}
