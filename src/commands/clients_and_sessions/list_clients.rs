use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
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
pub struct ListClients<'a>(TmuxCommand<'a>);

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

    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_KEY, format);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<String>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    pub fn output(&self) -> TmuxOutput {
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
