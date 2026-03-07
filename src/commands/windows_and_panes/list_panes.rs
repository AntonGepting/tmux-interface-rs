// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type LsP<'a> = ListPanes<'a>;

/// List panes on the server
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// list-panes [-as] [-F format] [-f filter] [-t target]
/// (alias: lsp)
/// ```
///
/// tmux >=1.6:
/// ```text
/// list-panes [-as] [-F format] [-t target]
/// (alias: lsp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// list-panes [-as] [-t target]
/// (alias: lsp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListPanes<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub all: bool,

    /// `[-s]`
    #[cfg(feature = "tmux_1_5")]
    pub session: bool,

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    pub filter: Option<Cow<'a, str>>,

    /// `[-t target]`
    #[cfg(feature = "tmux_1_5")]
    pub target: Option<Cow<'a, str>>,
}

impl<'a> ListPanes<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_5")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-s]`
    #[cfg(feature = "tmux_1_5")]
    pub fn session(mut self) -> Self {
        self.session = true;
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    pub fn filter<S: Into<Cow<'a, str>>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// `[-t target]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target<S: Into<Cow<'a, str>>>(mut self, target: S) -> Self {
        self.target = Some(target.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_PANES);

        // `[-a]`
        #[cfg(feature = "tmux_1_5")]
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-s]`
        #[cfg(feature = "tmux_1_5")]
        if self.session {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_1_6")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-f filter]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(filter) = self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter);
        }

        // `[-t target]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target) = self.target {
            cmd.push_option(T_LOWERCASE_KEY, target);
        }

        cmd
    }
}
