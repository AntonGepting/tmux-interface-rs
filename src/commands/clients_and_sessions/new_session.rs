use crate::commands::constants::*;
#[cfg(feature = "tmux_3_2")]
use crate::ClientFlags;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for creating a new session
///
/// # Manual
///
/// tmux 3.2:
/// ```text
/// tmux new-session [-AdDEPX] [-c start-directory] [-e environment] [-f flags] [-F format]
/// [-n window-name] [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 3.0:
/// ```text
/// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.4:
/// ```text
/// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.1:
/// ```text
/// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.9:
/// ```text
/// tmux new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.8:
/// ```text
/// tmux new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
/// [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.6:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
/// [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.2:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.1:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
/// (alias: new)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [command]
/// (alias: new)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NewSession<'a> {
    /// [-A] - behave like `attach-session` if `session-name` already exists
    #[cfg(feature = "tmux_1_8")]
    pub attach: bool,
    /// [-d] - new session is not attached to the current terminal
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,
    /// [-D] - any other clients attached to the session are detached
    #[cfg(feature = "tmux_1_8")]
    pub detach_other: bool,
    /// [-E] - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: bool,
    /// [-P] - print information about the new session after it has been created
    #[cfg(feature = "tmux_1_8")]
    pub print: bool,
    /// [-X] - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub parent_sighup: bool,
    /// [-c start-directory] - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub start_directory: Option<Cow<'a, str>>,
    /// [-F format] - specify different format
    #[cfg(feature = "tmux_1_8")]
    pub format: Option<Cow<'a, str>>,
    /// [-n window-name] - window name of the initial window
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<Cow<'a, str>>,
    /// [-s session-name] - specify a session name
    #[cfg(feature = "tmux_0_8")]
    pub session_name: Option<Cow<'a, str>>,
    /// [-t group-name] - specify a session group
    #[cfg(feature = "tmux_2_4")]
    pub group_name: Option<Cow<'a, str>>,
    /// [-x width] - specify a different width
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
    /// [-y height] - specify a different height
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    /// [shell-command] - shell command to execute in the initial window
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> NewSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-A]` - behave like `attach-session` if `session-name` already exists
    #[cfg(feature = "tmux_1_8")]
    pub fn attach(mut self) -> Self {
        self.attach = true;
        self
    }

    /// `[-d]` - new session is not attached to the current terminal
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-D]` - any other clients attached to the session are detached
    #[cfg(feature = "tmux_1_8")]
    pub fn detach_other(mut self) -> Self {
        self.detach_other = true;
        self
    }

    /// `[-E]` - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(mut self) -> Self {
        self.not_update_env = true;
        self
    }

    /// `[-P]` - print information about the new session after it has been created
    #[cfg(feature = "tmux_1_8")]
    pub fn print(mut self) -> Self {
        self.print = true;
        self
    }

    /// `[-X]` - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub fn parent_sighup(mut self) -> Self {
        self.parent_sighup = true;
        self
    }

    /// `[-c start-directory]` - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    // XXX: mb. 2 args - var, value?
    /// `[-e start-directory]` - takes the form ‘VARIABLE=value’ and sets an environment variable
    /// for the newly created session; it may be specified multiple times.
    #[cfg(feature = "tmux_3_2")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, environment: S) -> Self {
        self.environment = Some(environment.into());
        self
    }

    // XXX: refactor vec?
    /// `[-f flags]` - sets a comma-separated list of client flags
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(mut self, flags: ClientFlags) -> Self {
        self.flags = Some(flags.into());
        self
    }

    /// `[-F format]` - specify different format
    #[cfg(feature = "tmux_1_8")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-n window-name]` - window name of the initial window
    #[cfg(feature = "tmux_0_8")]
    pub fn window_name<S: Into<Cow<'a, str>>>(mut self, window_name: S) -> Self {
        self.window_name = Some(window_name.into());
        self
    }

    /// `[-s session-name]` - specify a session name
    #[cfg(feature = "tmux_0_8")]
    pub fn session_name<S: Into<Cow<'a, str>>>(mut self, session_name: S) -> Self {
        self.session_name = Some(session_name.into());
        self
    }

    /// `[-t group-name]` - specify a session group
    #[cfg(feature = "tmux_2_4")]
    pub fn group_name<S: Into<Cow<'a, str>>>(mut self, group_name: S) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    /// `[-x width]` - specify a different width
    #[cfg(feature = "tmux_1_6")]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width.into());
        self
    }

    /// `[-y height]` - specify a different height
    #[cfg(feature = "tmux_1_6")]
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height.into());
        self
    }

    /// `[shell-command]` - shell command to execute in the initial window
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    // NOTE: old: NewSession::new()
    //      new: NewSessionBin::new()? or only TmuxCommand::new_session()
    //      TmuxBinCommand::new_session()
    //
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(NEW_SESSION);

        // `[-A]` - behave like `attach-session` if `session-name` already exists
        #[cfg(feature = "tmux_1_8")]
        if self.attach {
            cmd.push_flag(A_UPPERCASE_KEY);
        }

        // `[-d]` - new session is not attached to the current terminal
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-D]` - any other clients attached to the session are detached
        #[cfg(feature = "tmux_1_8")]
        if self.detach_other {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-E]` - `update-environment` option will not be applied
        #[cfg(feature = "tmux_2_1")]
        if self.not_update_env {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-P]` - print information about the new session after it has been created
        #[cfg(feature = "tmux_1_8")]
        if self.print {
            cmd.push_flag(P_UPPERCASE_KEY);
        }

        // `[-X]` - send SIGHUP to the parent process, detaching the client
        #[cfg(feature = "tmux_3_0")]
        if self.parent_sighup {
            cmd.push_flag(X_UPPERCASE_KEY);
        }

        // `[-c start-directory]` - specify starting directory
        #[cfg(feature = "tmux_1_9")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(C_LOWERCASE_KEY, start_directory);
        }

        // XXX: mb. 2 args - var, value?
        // `[-e start-directory]` - takes the form ‘VARIABLE=value’ and sets an environment variable
        // for the newly created session; it may be specified multiple times.
        #[cfg(feature = "tmux_3_2")]
        if let Some(environment) = self.environment {
            cmd.push_option(E_LOWERCASE_KEY, environment);
        }

        // XXX: refactor vec?
        // `[-f flags]` - sets a comma-separated list of client flags
        #[cfg(feature = "tmux_3_2")]
        if let Some(flags) = self.flags {
            cmd.push_option(F_LOWERCASE_KEY, flags.to_string());
        }

        // `[-F format]` - specify different format
        #[cfg(feature = "tmux_1_8")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-n window-name]` - window name of the initial window
        #[cfg(feature = "tmux_0_8")]
        if let Some(window_name) = self.window_name {
            cmd.push_option(N_LOWERCASE_KEY, window_name);
        }

        // `[-s session-name]` - specify a session name
        #[cfg(feature = "tmux_0_8")]
        if let Some(session_name) = self.session_name {
            cmd.push_option(S_LOWERCASE_KEY, session_name);
        }

        // `[-t group-name]` - specify a session group
        #[cfg(feature = "tmux_2_4")]
        if let Some(group_name) = self.group_name {
            cmd.push_option(T_LOWERCASE_KEY, group_name);
        }

        // `[-x width]` - specify a different width
        #[cfg(feature = "tmux_1_6")]
        if let Some(width) = self.width {
            cmd.push_option(X_LOWERCASE_KEY, width.to_string());
        }

        // `[-y height]` - specify a different height
        #[cfg(feature = "tmux_1_6")]
        if let Some(height) = self.height {
            cmd.push_option(Y_LOWERCASE_KEY, height.to_string());
        }

        // `[shell-command]` - shell command to execute in the initial window
        #[cfg(feature = "tmux_1_2")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }

    //pub fn append_to(self, cmds: &mut TmuxCommands<'a>) {
    //self.0.append_to(cmds);
    //}

    //pub fn into_inner(self) -> TmuxCommand<'a> {
    //self.0
    //}

    //// ?
    //pub fn as_ref(&self) -> &TmuxCommand<'a> {
    //&self.0
    //}

    //// ?
    //pub fn as_mut(&mut self) -> &mut TmuxCommand<'a> {
    //&mut self.0
    //}

    //pub fn into_tmux_bin_command(self) -> TmuxBinCommand<'a> {
    //self.0.into_tmux_bin_command()
    //}

    //pub fn into_tmux_bin_command_ext(self, tmux: TmuxBin<'a>) -> TmuxBinCommand<'a> {
    //self.0.into_tmux_bin_command_ext(tmux)
    //}
}
