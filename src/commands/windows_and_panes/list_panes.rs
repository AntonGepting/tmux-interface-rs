use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListPanes<'a> {
    /// `[-a]`
    pub all: bool,

    /// `[-s]`
    pub session: bool,

    /// `[-F format]`
    pub format: Option<Cow<'a, str>>,

    /// `[-t target]`
    pub target: Option<Cow<'a, str>>,
}

impl<'a> ListPanes<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-s]`
    pub fn session(mut self) -> Self {
        self.session = true;
        self
    }

    /// `[-F format]`
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-t target]`
    pub fn target<S: Into<Cow<'a, str>>>(mut self, target: S) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_PANES);

        // `[-a]`
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-s]`
        if self.session {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-F format]`
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-t target]`
        if let Some(target) = self.target {
            cmd.push_option(T_LOWERCASE_KEY, target);
        }

        cmd
    }
}
