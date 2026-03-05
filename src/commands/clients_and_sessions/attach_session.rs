// auto-generated file
//

use crate::commands::constants::*;
#[cfg(feature = "tmux_3_2")]
use crate::ClientFlags;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Attach<'a> = AttachSession<'a>;

/// Attach client to already existing session
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// attach-session [-dErx] [-c working-directory] [-f flags] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux >=2.1:
/// ```text
/// attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux >=1.9:
/// ```text
/// attach-session [-dr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux >=1.5:
/// ```text
/// attach-session [-dr] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux >=0.8:
/// ```text
/// attach-session [-d] [-t target-session]
/// (alias: attach)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AttachSession<'a> {
    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub detach_other: bool,

    /// `[-E]`
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub read_only: bool,

    /// `[-x]`
    #[cfg(feature = "tmux_3_0a")]
    pub parent_sighup: bool,

    /// `[-c working-directory]`
    #[cfg(feature = "tmux_1_9")]
    pub working_directory: Option<Cow<'a, str>>,

    /// `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    pub flags: Option<ClientFlags>,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
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

    /// `[-r]`
    #[cfg(feature = "tmux_1_5")]
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// `[-x]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn parent_sighup(mut self) -> Self {
        self.parent_sighup = true;
        self
    }

    /// `[-c working-directory]`
    #[cfg(feature = "tmux_1_9")]
    pub fn working_directory<S: Into<Cow<'a, str>>>(mut self, working_directory: S) -> Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    /// `[-f flags]`
    #[cfg(feature = "tmux_3_2")]
    pub fn flags(mut self, flags: ClientFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(ATTACH_SESSION);

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.detach_other {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-E]`
        #[cfg(feature = "tmux_2_1")]
        if self.not_update_env {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_1_5")]
        if self.read_only {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-x]`
        #[cfg(feature = "tmux_3_0a")]
        if self.parent_sighup {
            cmd.push_flag(X_LOWERCASE_KEY);
        }

        // `[-c working-directory]`
        #[cfg(feature = "tmux_1_9")]
        if let Some(working_directory) = self.working_directory {
            cmd.push_option(C_LOWERCASE_KEY, working_directory);
        }

        // `[-f flags]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(flags) = self.flags {
            cmd.push_option(F_LOWERCASE_KEY, flags.to_string());
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
