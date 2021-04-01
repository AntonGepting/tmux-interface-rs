use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux show-environment [-gs] [-t target-session] [variable]
/// (alias: showenv)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux show-environment [-g] [-t target-session] [variable]
/// (alias: showenv)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux show-environment [-g] [-t target-session]
/// (alias: showenv)
/// ```
#[derive(Debug, Clone)]
pub struct ShowEnvironment<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ShowEnvironment<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SHOW_ENVIRONMENT)),
            ..Default::default()
        })
    }
}

impl<'a> ShowEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-g]`
    #[cfg(feature = "tmux_1_0")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-s]`
    #[cfg(feature = "tmux_2_1")]
    pub fn as_shell_commands(&mut self) -> &mut Self {
        self.0.push_flag(S_LOWERCASE_KEY);
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    /// `[variable]`
    #[cfg(feature = "tmux_1_7")]
    pub fn variable<S: Into<Cow<'a, str>>>(&mut self, variable: S) -> &mut Self {
        self.0.push_param(variable);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ShowEnvironment<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SHOW_ENVIRONMENT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ShowEnvironment<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SHOW_ENVIRONMENT)),
            ..Default::default()
        })
    }
}
