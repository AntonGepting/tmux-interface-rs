use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Reactivate a pane in which the command has exited
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux respawn-pane [-k] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct RespawnPane<'a, T: Display> {
    /// [-k] - any existing command is killed
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> RespawnPane<'a, T> {
    pub fn new() -> RespawnPane<'a, T> {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct RespawnPaneBuilder<'a, T: Display> {
    pub kill: Option<bool>,
    pub start_directory: Option<&'a str>,
    pub environment: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub shell_command: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> RespawnPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    pub fn start_directory(&mut self, start_directory: &'a str) -> &mut Self {
        self.start_directory = Some(start_directory);
        self
    }

    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> RespawnPane<'a, T> {
        RespawnPane {
            kill: self.kill,
            start_directory: self.start_directory,
            environment: self.environment,
            target_pane: self.target_pane,
            shell_command: self.shell_command,
        }
    }
}

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
    pub fn respawn_pane<T: Display>(
        &mut self,
        respawn_pane: Option<&RespawnPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(target_pane) = respawn_pane.target_pane {
                s = target_pane.to_string();
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
    pub fn respawn_pane<T: Display>(
        &mut self,
        kill: Option<bool>,
        start_directory: Option<&'a str>,
        target_pane: Option<&'a T>,
        shell_command: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if kill.unwrap_or(false) {
            args.push(k_KEY);
        }
        if let Some(s) = start_directory {
            args.extend_from_slice(&[c_KEY, &s])
        }
        if let Some(target_pane) = target_pane {
            s = target_pane.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = shell_command {
            args.push(s)
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_PANE, &args)?;
        Ok(output)
    }
}
