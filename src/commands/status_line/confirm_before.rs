use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^1.5:
/// ```text
/// tmux confirm-before [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux confirm-before [-t target-client] command
/// (alias: confirm)
/// ```
#[derive(Debug, Clone)]
pub struct ConfirmBefore<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ConfirmBefore<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(CONFIRM_BEFORE)),
            ..Default::default()
        })
    }
}

impl<'a> ConfirmBefore<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-p prompt]`
    #[cfg(feature = "tmux_1_5")]
    pub fn prompt<S: Into<Cow<'a, str>>>(&mut self, prompt: S) -> &mut Self {
        self.0.push_option(P_LOWERCASE_KEY, prompt);
        self
    }

    /// `[-t target-client]`
    #[cfg(feature = "tmux_0_9")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    /// `command`
    #[cfg(feature = "tmux_0_9")]
    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ConfirmBefore<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(CONFIRM_BEFORE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ConfirmBefore<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(CONFIRM_BEFORE)),
            ..Default::default()
        })
    }
}
