use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for setting or unsetting hook `hook-name` to command.
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux set-hook [-agRu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.8:
/// ```text
/// tmux set-hook [-gRu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.4:
/// ```text
/// tmux set-hook [-gu] [-t target-session] hook-name command
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux set-hook [-g] [-t target-session] hook-name command
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SetHook<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_3_0")]
    pub append: bool,

    /// `[-g]`
    #[cfg(feature = "tmux_2_2")]
    pub global: bool,

    /// `[-R]`
    #[cfg(feature = "tmux_2_8")]
    pub run: bool,

    /// `[-u]`
    #[cfg(feature = "tmux_2_4")]
    pub unset: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_2_2")]
    pub target_session: Option<Cow<'a, str>>,

    /// `hook-name`
    #[cfg(feature = "tmux_2_2")]
    pub hook_name: Option<Cow<'a, str>>,

    /// `command`
    #[cfg(feature = "tmux_2_2")]
    pub command: Option<Cow<'a, str>>,
}

impl<'a> SetHook<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_3_0")]
    pub fn append(mut self) -> Self {
        self.append = true;
        self
    }

    /// `[-g]`
    #[cfg(feature = "tmux_2_2")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-R]`
    #[cfg(feature = "tmux_2_8")]
    pub fn run(mut self) -> Self {
        self.run = true;
        self
    }

    /// `[-u]`
    #[cfg(feature = "tmux_2_4")]
    pub fn unset(mut self) -> Self {
        self.unset = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_2_2")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `hook-name`
    #[cfg(feature = "tmux_2_2")]
    pub fn hook_name<S: Into<Cow<'a, str>>>(mut self, hook_name: S) -> Self {
        self.hook_name = Some(hook_name.into());
        self
    }

    // XXX: command?
    /// `command`
    #[cfg(feature = "tmux_2_2")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SET_HOOK);

        // `[-a]`
        #[cfg(feature = "tmux_3_0")]
        if self.append {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-g]`
        #[cfg(feature = "tmux_2_2")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-R]`
        #[cfg(feature = "tmux_2_8")]
        if self.run {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-u]`
        #[cfg(feature = "tmux_2_4")]
        if self.unset {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_2_2")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `hook-name`
        #[cfg(feature = "tmux_2_2")]
        if let Some(hook_name) = self.hook_name {
            cmd.push_param(hook_name);
        }

        // `command`
        #[cfg(feature = "tmux_2_2")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
