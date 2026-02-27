// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type ClearHist<'a> = ClearHistory<'a>;

/// Remove and free the history for the specified pane.
///
/// # Manual
///
/// tmux ^3.4:
/// ```text
/// clear-history [-H] [-t target-pane]
/// (alias: clearhist)
/// ```
///
/// tmux ^1.5:
/// ```text
/// clear-history [-t target-pane]
/// (alias: clearhist)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ClearHistory<'a> {
    /// `[-H]`
    #[cfg(feature = "tmux_3_4")]
    pub no_hyperlinks: bool,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> ClearHistory<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-H]`
    #[cfg(feature = "tmux_3_4")]
    pub fn no_hyperlinks(mut self) -> Self {
        self.no_hyperlinks = true;
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CLEAR_HISTORY);

        // `[-H]`
        #[cfg(feature = "tmux_3_4")]
        if self.no_hyperlinks {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        cmd
    }
}
