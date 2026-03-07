// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type NewW<'a> = NewWindow<'a>;

/// Create new window
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// new-window [-abdkPS] [-c start-directory] [-e environment] [-F format] [-n window-name]
/// [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux >=3.0:
/// ```text
/// new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name] [-t
/// target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux >=1.7:
/// ```text
/// new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
/// [shell-command]
/// (alias: neww)
/// ```
///
/// tmux >=1.5:
/// ```text
/// new-window [-adkP] [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
///
/// tmux >=0.8:
/// ```text
/// new-window [-d] [-n window-name] [-t target-window] [command]
/// (alias: neww)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NewWindow<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub after: bool,

    /// `[-b]`
    #[cfg(feature = "tmux_3_2")]
    pub before: bool,

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-k]`
    #[cfg(feature = "tmux_1_5")]
    pub kill: bool,

    /// `[-P]`
    #[cfg(feature = "tmux_1_5")]
    pub print: bool,

    /// `[-S]`
    #[cfg(feature = "tmux_3_2")]
    pub select: bool,

    /// `[-c start-directory]`
    #[cfg(feature = "tmux_1_7")]
    pub start_directory: Option<Cow<'a, str>>,

    /// `[-e environment]`
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<Cow<'a, str>>,

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    /// `[-n window-name]`
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> NewWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub fn after(mut self) -> Self {
        self.after = true;
        self
    }

    /// `[-b]`
    #[cfg(feature = "tmux_3_2")]
    pub fn before(mut self) -> Self {
        self.before = true;
        self
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-k]`
    #[cfg(feature = "tmux_1_5")]
    pub fn kill(mut self) -> Self {
        self.kill = true;
        self
    }

    /// `[-P]`
    #[cfg(feature = "tmux_1_5")]
    pub fn print(mut self) -> Self {
        self.print = true;
        self
    }

    /// `[-S]`
    #[cfg(feature = "tmux_3_2")]
    pub fn select(mut self) -> Self {
        self.select = true;
        self
    }

    /// `[-c start-directory]`
    #[cfg(feature = "tmux_1_7")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-e environment]`
    #[cfg(feature = "tmux_3_0")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, environment: S) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-n window-name]`
    #[cfg(feature = "tmux_0_8")]
    pub fn window_name<S: Into<Cow<'a, str>>>(mut self, window_name: S) -> Self {
        self.window_name = Some(window_name.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(NEW_WINDOW);

        // `[-a]`
        #[cfg(feature = "tmux_1_5")]
        if self.after {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-b]`
        #[cfg(feature = "tmux_3_2")]
        if self.before {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-k]`
        #[cfg(feature = "tmux_1_5")]
        if self.kill {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-P]`
        #[cfg(feature = "tmux_1_5")]
        if self.print {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-S]`
        #[cfg(feature = "tmux_3_2")]
        if self.select {
            cmd.push_flag(S_UPPERCASE_KEY);
        }

        // `[-c start-directory]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(C_LOWERCASE_KEY, start_directory);
        }

        // `[-e environment]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(environment) = self.environment {
            cmd.push_option(E_LOWERCASE_KEY, environment);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-n window-name]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(window_name) = self.window_name {
            cmd.push_option(N_LOWERCASE_KEY, window_name);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[shell-command]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
