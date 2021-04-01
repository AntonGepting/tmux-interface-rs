use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// XXX: better return type
/// List panes on the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// tmux list-panes [-as] [-F format] [-t target]
/// (alias: lsp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux list-panes [-as] [-t target]
/// (alias: lsp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux list-panes [-t target]
/// (alias: lsp)
/// ```
#[derive(Debug, Clone)]
pub struct ListPanes<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ListPanes<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LIST_PANES)),
            ..Default::default()
        })
    }
}

impl<'a> ListPanes<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    pub fn all(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-s]`
    pub fn session(&mut self) -> &mut Self {
        self.0.push_flag(S_LOWERCASE_KEY);
        self
    }

    /// `[-F format]`
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    /// `[-t target]`
    pub fn target<S: Into<Cow<'a, str>>>(&mut self, target: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ListPanes<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LIST_PANES)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ListPanes<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LIST_PANES)),
            ..Default::default()
        })
    }
}
