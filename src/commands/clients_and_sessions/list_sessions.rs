// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
pub type Ls<'a> = ListSessions<'a>;

// XXX: better result return?
/// List all sessions managed by the server
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// list-sessions [-F format] [-f filter]
/// (alias: ls)
/// ```
///
/// tmux >=1.6:
/// ```text
/// list-sessions [-F format]
/// (alias: ls)
/// ```
///
/// tmux >=0.8:
/// ```text
/// list-sessions
/// (alias: ls)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListSessions<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    pub filter: Option<Cow<'a, str>>,
}

impl<'a> ListSessions<'a> {
    pub fn new() -> Self {
        Default::default()
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

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LIST_SESSIONS);

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

        cmd
    }
}
