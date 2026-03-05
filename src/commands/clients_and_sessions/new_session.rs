// auto-generated file
//

use crate::commands::constants::*;
#[cfg(feature = "tmux_3_2")]
use crate::ClientFlags;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type New<'a> = NewSession<'a>;

/// Create new session
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
/// [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=3.0:
/// ```text
/// new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=2.4:
/// ```text
/// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=2.1:
/// ```text
/// new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.9:
/// ```text
/// new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.8:
/// ```text
/// new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
/// [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.6:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
/// [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.2:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
/// (alias: new)
/// ```
///
/// tmux >=1.1:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
/// (alias: new)
/// ```
///
/// tmux >=0.8:
/// ```text
/// new-session [-d] [-n window-name] [-s session-name] [command]
/// (alias: new)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NewSession<'a> {
    /// `[-A]`
    #[cfg(feature = "tmux_1_8")]
    pub attach: bool,

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-D]`
    #[cfg(feature = "tmux_1_8")]
    pub detach_other: bool,

    /// `[-E]`
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: bool,

    /// `[-P]`
    #[cfg(feature = "tmux_1_8")]
    pub print: bool,

    /// `[-X]`
    #[cfg(feature = "tmux_3_0a")]
    pub parent_sighup: bool,

    /// `[-c start-directory]`
    #[cfg(feature = "tmux_1_9")]
    pub start_directory: Option<Cow<'a, str>>,

    /// `[-e environment]`
    #[cfg(feature = "tmux_3_2")]
    pub environment: Option<Vec<(Cow<'a, str>, Cow<'a, str>)>>,

    /// `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    pub flags: Option<ClientFlags>,

    /// `[-F format]`
    #[cfg(feature = "tmux_1_8")]
    pub format: Option<Cow<'a, str>>,

    /// `[-n window-name]`
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<Cow<'a, str>>,

    /// `[-s session-name]`
    #[cfg(feature = "tmux_0_8")]
    pub session_name: Option<Cow<'a, str>>,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    pub target_session: Option<Cow<'a, str>>,

    /// `[-t group-name]`
    #[cfg(feature = "tmux_2_4")]
    pub group_name: Option<Cow<'a, str>>,

    /// `[-x width]`
    #[cfg(feature = "tmux_1_5")]
    pub width: Option<usize>,

    /// `[-y height]`
    #[cfg(feature = "tmux_1_5")]
    pub height: Option<usize>,

    /// `[command]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub command: Option<Cow<'a, str>>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> NewSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-A]`
    #[cfg(feature = "tmux_1_8")]
    pub fn attach(mut self) -> Self {
        self.attach = true;
        self
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-D]`
    #[cfg(feature = "tmux_1_8")]
    pub fn detach_other(mut self) -> Self {
        self.detach_other = true;
        self
    }

    /// `[-E]`
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(mut self) -> Self {
        self.not_update_env = true;
        self
    }

    /// `[-P]`
    #[cfg(feature = "tmux_1_8")]
    pub fn print(mut self) -> Self {
        self.print = true;
        self
    }

    /// `[-X]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn parent_sighup(mut self) -> Self {
        self.parent_sighup = true;
        self
    }

    /// `[-c start-directory]`
    #[cfg(feature = "tmux_1_9")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-e start-directory]` - takes the form ‘VARIABLE=value’ and sets an environment variable
    /// for the newly created session; it may be specified multiple times.
    #[cfg(feature = "tmux_3_2")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, variable: S, value: S) -> Self {
        self.environment
            .get_or_insert(Vec::new())
            .push((variable.into(), value.into()));
        self
    }

    // XXX: refactor vec?
    /// `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(mut self, flags: ClientFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_8")]
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

    /// `[-s session-name]`
    #[cfg(feature = "tmux_0_8")]
    pub fn session_name<S: Into<Cow<'a, str>>>(mut self, session_name: S) -> Self {
        self.session_name = Some(session_name.into());
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-t group-name]`
    #[cfg(feature = "tmux_2_4")]
    pub fn group_name<S: Into<Cow<'a, str>>>(mut self, group_name: S) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    /// `[-x width]`
    #[cfg(feature = "tmux_1_5")]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// `[-y height]`
    #[cfg(feature = "tmux_1_5")]
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// `[command]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    // NOTE: old: NewSession::new()
    //      new: NewSessionBin::new()? or only TmuxCommand::new_session()
    //      TmuxBinCommand::new_session()
    //
    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(NEW_SESSION);

        // `[-A]`
        #[cfg(feature = "tmux_1_8")]
        if self.attach {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-D]`
        #[cfg(feature = "tmux_1_8")]
        if self.detach_other {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-E]`
        #[cfg(feature = "tmux_2_1")]
        if self.not_update_env {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-P]`
        #[cfg(feature = "tmux_1_8")]
        if self.print {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-X]`
        #[cfg(feature = "tmux_3_0a")]
        if self.parent_sighup {
            cmd.push_flag(X_UPPERCASE_KEY);
        }

        // `[-c start-directory]`
        #[cfg(feature = "tmux_1_9")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(C_LOWERCASE_KEY, start_directory);
        }

        // XXX: mb. 2 args - var, value?
        // `[-e start-directory]` - takes the form ‘VARIABLE=value’ and sets an environment variable
        // for the newly created session; it may be specified multiple times.
        #[cfg(feature = "tmux_3_2")]
        if let Some(environment) = self.environment {
            for variable in environment {
                cmd.push_option(E_LOWERCASE_KEY, format!("{}={}", variable.0, variable.1));
            }
        }

        // XXX: refactor vec?
        // `[-f flags]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(flags) = self.flags {
            cmd.push_option(F_LOWERCASE_KEY, flags.to_string());
        }

        // `[-F format]`
        #[cfg(feature = "tmux_1_8")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-n window-name]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(window_name) = self.window_name {
            cmd.push_option(N_LOWERCASE_KEY, window_name);
        }

        // `[-s session-name]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(session_name) = self.session_name {
            cmd.push_option(S_LOWERCASE_KEY, session_name);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[-t group-name]`
        #[cfg(feature = "tmux_2_4")]
        if let Some(group_name) = self.group_name {
            cmd.push_option(T_LOWERCASE_KEY, group_name);
        }

        // `[-x width]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(width) = self.width {
            cmd.push_option(X_LOWERCASE_KEY, width.to_string());
        }

        // `[-y height]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(height) = self.height {
            cmd.push_option(Y_LOWERCASE_KEY, height.to_string());
        }

        // `[command]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        // `[shell-command]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
