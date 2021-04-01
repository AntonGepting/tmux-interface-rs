use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// List all clients attached to the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// tmux list-clients [-F format] [-t target-session]
/// (alias: lsc)
///
/// ```
/// tmux ^1.5:
/// ```text
/// tmux list-clients [-t target-session]
/// (alias: lsc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-clients
/// (alias: lsc)
/// ```
#[derive(Debug, Clone)]
pub struct ListClients<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ListClients<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_CLIENTS)),
            ..Default::default()
        })
    }
}

impl<'a> ListClients<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ListClients<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LIST_CLIENTS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ListClients<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LIST_CLIENTS)),
            ..Default::default()
        })
    }
}
