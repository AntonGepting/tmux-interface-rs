use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type UnlinkW<'a> = UnlinkWindow<'a>;

/// Unlink `target-window`
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// unlink-window [-k] [-t target-window]
/// (alias: unlinkw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// unlink-window [-t target-window]
/// (alias: unlinkw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UnlinkWindow<'a> {
    /// `[-k]`
    #[cfg(feature = "tmux_1_0")]
    pub detach_other: bool,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> UnlinkWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-k]`
    #[cfg(feature = "tmux_1_0")]
    pub fn detach_other(mut self) -> Self {
        self.detach_other = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(UNLINK_WINDOW);

        // `[-k]`
        #[cfg(feature = "tmux_1_0")]
        if self.detach_other {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
