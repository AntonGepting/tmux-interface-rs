use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for creating new window, using `tmux new-window` command
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
/// target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
/// [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux new-window [-adk] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux new-window [-dk] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux new-window [-dk] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux new-window [-d] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
#[derive(Debug, Clone)]
pub struct NewWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for NewWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(NEW_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> NewWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` - new window is inserted at the next index up from the specified target-window
    #[cfg(feature = "tmux_1_3")]
    pub fn add(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-d]` - the session does not make the new window the current window
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-k]` - destroy if already exists
    #[cfg(feature = "tmux_1_0")]
    pub fn kill(&mut self) -> &mut Self {
        self.0.push_flag(K_LOWERCASE_KEY);
        self
    }

    /// `[-P]` - print information about the new window after it has been created
    #[cfg(feature = "tmux_1_5")]
    pub fn print(&mut self) -> &mut Self {
        self.0.push_flag(P_UPPERCASE_KEY);
        self
    }

    /// `[-c start-directory]` - start-directory
    #[cfg(feature = "tmux_1_7")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(&mut self, start_directory: S) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, start_directory);
        self
    }

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_0")]
    pub fn environment<S: Into<Cow<'a, str>>>(&mut self, environment: S) -> &mut Self {
        self.0.push_option(E_LOWERCASE_KEY, environment);
        self
    }

    /// `[-F format]` - format
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    /// `[-n window-name]` - window-name
    #[cfg(feature = "tmux_0_8")]
    pub fn window_name<S: Into<Cow<'a, str>>>(&mut self, window_name: S) -> &mut Self {
        self.0.push_option(N_LOWERCASE_KEY, window_name);
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for NewWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(NEW_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for NewWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(NEW_WINDOW)),
            ..Default::default()
        })
    }
}
