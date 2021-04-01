use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for setting or unsetting an environment variable
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// tmux set-environment [-gru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
#[derive(Debug, Clone)]
pub struct SetEnvironment<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SetEnvironment<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SET_ENVIRONMENT)),
            ..Default::default()
        })
    }
}

impl<'a> SetEnvironment<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-g]` - make change in the global environment
    #[cfg(feature = "tmux_1_0")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-r]` - remove the variable from the environment before starting a new process
    #[cfg(feature = "tmux_1_0")]
    pub fn remove(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-u]` - unset a variable
    #[cfg(feature = "tmux_1_0")]
    pub fn unset(&mut self) -> &mut Self {
        self.0.push_flag(U_LOWERCASE_KEY);
        self
    }

    /// `[-t target-session]` - target-session
    #[cfg(feature = "tmux_1_0")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    /// `name`
    #[cfg(feature = "tmux_1_0")]
    pub fn name<S: Into<Cow<'a, str>>>(&mut self, name: S) -> &mut Self {
        self.0.push_param(name);
        self
    }

    /// `[value]` - specify the value
    #[cfg(feature = "tmux_1_0")]
    pub fn value<S: Into<Cow<'a, str>>>(&mut self, value: S) -> &mut Self {
        self.0.push_param(value);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SetEnvironment<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SET_ENVIRONMENT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SetEnvironment<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SET_ENVIRONMENT)),
            ..Default::default()
        })
    }
}
