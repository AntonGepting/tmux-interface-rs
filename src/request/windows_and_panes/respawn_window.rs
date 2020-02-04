use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Reactivate a window in which the command has exited
///
/// # Manual
///
/// tmux X.X
/// ```text
/// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
/// [shell-command]
/// (alias: respawnw)
///
/// tmux 2.6
/// ```text
/// tmux respawn-window [-k] [-c start-directory] [-t target-window] [shell-command]
/// (alias: respawnw)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct RespawnWindow<'a, T: Display> {
    /// [-k] - any existing command is killed
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_window: Option<&'a T>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> RespawnWindow<'a, T> {
    pub fn new() -> RespawnWindow<'a, T> {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const RESPAWN_WINDOW: &'static str = "respawn-window";

    /// Reactivate a window in which the command has exited
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
    /// [shell-command]
    /// (alias: respawnw)
    ///
    /// tmux 2.6
    /// ```text
    /// tmux respawn-window [-k] [-c start-directory] [-t target-window] [shell-command]
    /// (alias: respawnw)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn respawn_window<T: Display>(
        &mut self,
        respawn_window: Option<&RespawnWindow<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(respawn_window) = respawn_window {
            if respawn_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = respawn_window.start_directory {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = respawn_window.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            if let Some(target_window) = respawn_window.target_window {
                s = target_window.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = respawn_window.shell_command {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_WINDOW, &args)?;
        Ok(output)
    }

    /// Reactivate a window in which the command has exited
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
    /// [shell-command]
    /// (alias: respawnw)
    ///
    /// tmux 2.6
    /// ```text
    /// tmux respawn-window [-k] [-c start-directory] [-t target-window] [shell-command]
    /// (alias: respawnw)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn respawn_window(
        &mut self,
        kill: Option<bool>,
        start_directory: Option<&'a str>,
        target_window: Option<&'a str>,
        shell_command: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if kill.unwrap_or(false) {
            args.push(k_KEY);
        }
        if let Some(s) = start_directory {
            args.extend_from_slice(&[c_KEY, &s])
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = shell_command {
            args.push(s)
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_WINDOW, &args)?;
        Ok(output)
    }
}
