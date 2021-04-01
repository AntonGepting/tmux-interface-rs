use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for attaching client to already existing session
///
/// # Manual
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
#[derive(Clone, Debug)]
pub struct AttachSession<'a>(pub TmuxCommand<'a>);

impl<'a> Default for AttachSession<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(ATTACH_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - any other clients attached to the session are detached
    #[cfg(feature = "tmux_0_8")]
    pub fn detach_other(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-E]` - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.0.push_flag(E_UPPERCASE_KEY);
        self
    }

    /// `[-r]` - signifies the client is read-only
    #[cfg(feature = "tmux_1_2")]
    pub fn read_only(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-x]` - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.0.push_flag(X_LOWERCASE_KEY);
        self
    }

    /// `[-c working-directory]` - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub fn working_directory<S: Into<Cow<'a, str>>>(&mut self, working_directory: S) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, working_directory);
        self
    }

    /// `[-t target-session]` - specify target session name
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for AttachSession<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(ATTACH_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for AttachSession<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(ATTACH_SESSION)),
            ..Default::default()
        })
    }
}
