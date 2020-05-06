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
#[derive(Default, Debug)]
pub struct RespawnPane<'a, T: Display> {
    /// [-k] - any existing command is killed
    #[cfg(feature = "tmux_1_5")]
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<&'a T>,
    /// [shell-command] - shell-command
    #[cfg(feature = "tmux_2_6")]
    pub shell_command: Option<&'a str>,
}

impl<'a, T: Display + Default> RespawnPane<'a, T> {
    pub fn new() -> RespawnPane<'a, T> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct RespawnPaneBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_1_5")]
    pub kill: Option<bool>,
    #[cfg(feature = "tmux_2_6")]
    pub start_directory: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<&'a str>,
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<&'a T>,
    #[cfg(feature = "tmux_2_6")]
    pub shell_command: Option<&'a str>,
}

impl<'a, T: Display + Default> RespawnPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn start_directory(&mut self, start_directory: &'a str) -> &mut Self {
        self.start_directory = Some(start_directory);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> RespawnPane<'a, T> {
        RespawnPane {
            #[cfg(feature = "tmux_1_5")]
            kill: self.kill,
            #[cfg(feature = "tmux_2_6")]
            start_directory: self.start_directory,
            #[cfg(feature = "tmux_3_0")]
            environment: self.environment,
            #[cfg(feature = "tmux_1_5")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_2_6")]
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
    pub fn respawn_pane<T: Display>(
        &mut self,
        respawn_pane: Option<&RespawnPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(respawn_pane) = respawn_pane {
            #[cfg(feature = "tmux_1_5")]
            if respawn_pane.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = respawn_pane.start_directory {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(s) = respawn_pane.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            #[cfg(feature = "tmux_1_5")]
            if let Some(target_pane) = respawn_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = respawn_pane.shell_command {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_PANE, &args)?;
        Ok(output)
    }
}
