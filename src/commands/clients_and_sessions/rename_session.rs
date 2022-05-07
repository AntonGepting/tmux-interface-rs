use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Rename the session to `new-name`
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux rename-session [-t target-session] new-name
/// (alias: rename)
/// ```
#[derive(Debug, Default, Clone)]
pub struct RenameSession<'a> {
    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,

    /// `new-name`
    #[cfg(feature = "tmux_0_8")]
    pub new_name: Option<Cow<'a, str>>,
}

impl<'a> RenameSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `new-name`
    #[cfg(feature = "tmux_0_8")]
    pub fn new_name<S: Into<Cow<'a, str>>>(&mut self, new_name: S) -> &mut Self {
        self.new_name = Some(new_name.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(RENAME_SESSION);

        // `[-t target-session]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        // `new-name`
        #[cfg(feature = "tmux_0_8")]
        if let Some(new_name) = &self.new_name {
            cmd.push_param(new_name.as_ref());
        }

        cmd
    }
}
