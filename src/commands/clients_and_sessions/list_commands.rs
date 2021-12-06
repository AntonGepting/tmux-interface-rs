use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// List the syntax of all commands supported by tmux
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux list-commands [-F format] [command]
/// (alias: lscm)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux list-commands [-F format]
/// (alias: lscm)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-commands
/// (alias: lscm)
/// ```
#[derive(Debug, Clone)]
pub struct ListCommands<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ListCommands<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_COMMANDS)),
            ..Default::default()
        })
    }
}

impl<'a> ListCommands<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_2_3")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    /// `[command]`
    #[cfg(feature = "tmux_3_2")]
    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ListCommands<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_COMMANDS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ListCommands<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_COMMANDS)),
            ..Default::default()
        })
    }
}
