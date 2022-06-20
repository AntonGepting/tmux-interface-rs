use crate::commands::constants::*;
#[cfg(feature = "tmux_3_2")]
use crate::ClientFlags;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for attaching client to already existing session
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux attach-session [-dErx] [-c working-directory] [-f flags] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux attach-session [-dr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux attach-session [-dr] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux attach-session [-d] [-t target-session]
/// (alias: attach)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AttachSession<'a> {
    /// `[-d]` - any other clients attached to the session are detached
    #[cfg(feature = "tmux_0_8")]
    pub detach_other: bool,

    /// `[-E]` - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: bool,

    /// `[-r]` - signifies the client is read-only
    #[cfg(feature = "tmux_1_2")]
    pub read_only: bool,

    /// `[-x]` - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub parent_sighup: bool,

    /// `[-c working-directory]` - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub working_directory: Option<Cow<'a, str>>,

    /// `[-f flags]` - sets a comma-separated list of client flags
    #[cfg(feature = "tmux_3_2")]
    pub flags: Option<ClientFlags>,

    /// `[-t target-session]` - specify target session name
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - any other clients attached to the session are detached
    #[cfg(feature = "tmux_0_8")]
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

    /// `[-r]` - signifies the client is read-only
    #[cfg(feature = "tmux_1_2")]
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// `[-x]` - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub fn parent_sighup(mut self) -> Self {
        self.parent_sighup = true;
        self
    }

    /// `[-c working-directory]` - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub fn working_directory<S: Into<Cow<'a, str>>>(mut self, working_directory: S) -> Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    // XXX: refactor vec?
    /// `[-f flags]` - sets a comma-separated list of client flags
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(mut self, flags: ClientFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// `[-t target-session]` - specify target session name
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(ATTACH_SESSION);

        // `[-d]` - any other clients attached to the session are detached
        #[cfg(feature = "tmux_0_8")]
        if self.detach_other {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-E]` - `update-environment` option will not be applied
        #[cfg(feature = "tmux_2_1")]
        if self.not_update_env {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-r]` - signifies the client is read-only
        #[cfg(feature = "tmux_1_2")]
        if self.read_only {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-x]` - send SIGHUP to the parent process, detaching the client
        #[cfg(feature = "tmux_3_0")]
        if self.parent_sighup {
            cmd.push_flag(X_LOWERCASE_KEY);
        }

        // `[-c working-directory]` - specify starting directory
        #[cfg(feature = "tmux_1_9")]
        if let Some(working_directory) = self.working_directory {
            cmd.push_option(C_LOWERCASE_KEY, working_directory);
        }

        // `[-f flags]` - sets a comma-separated list of client flags
        #[cfg(feature = "tmux_3_2")]
        if let Some(flags) = self.flags {
            cmd.push_option(F_LOWERCASE_KEY, flags.to_string());
        }

        // `[-t target-session]` - specify target session name
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
