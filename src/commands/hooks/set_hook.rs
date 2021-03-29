use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct SetHook<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SetHook<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SET_HOOK)),
            ..Default::default()
        })
    }
}

impl<'a> SetHook<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn append(&mut self) -> &mut Self {
        self.0.push_flag(a_KEY);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(g_KEY);
        self
    }

    #[cfg(feature = "tmux_2_8")]
    pub fn run(&mut self) -> &mut Self {
        self.0.push_flag(R_KEY);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn unset(&mut self) -> &mut Self {
        self.0.push_flag(u_KEY);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn name<S: Into<Cow<'a, str>>>(&mut self, name: S) -> &mut Self {
        self.0.push_param(name);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SetHook<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SET_HOOK)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SetHook<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SET_HOOK)),
            ..Default::default()
        })
    }
}
