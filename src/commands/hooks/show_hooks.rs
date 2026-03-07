// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Shows hooks
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// show-hooks [-gpw] [-t target-session]
/// ```
///
/// tmux >=2.2:
/// ```text
/// show-hooks [-g] [-t target-session]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowHooks<'a> {
    /// `[-g]`
    #[cfg(feature = "tmux_2_2")]
    pub global: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_3_2")]
    pub pane: bool,

    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub window: bool,

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
    pub target_session: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> ShowHooks<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-g]`
    #[cfg(feature = "tmux_2_2")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_3_2")]
    pub fn pane(mut self) -> Self {
        self.pane = true;
        self
    }

    /// `[-w]`
    #[cfg(feature = "tmux_3_2")]
    pub fn window(mut self) -> Self {
        self.window = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_HOOKS);

        // `[-g]`
        #[cfg(feature = "tmux_2_2")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_3_2")]
        if self.pane {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-w]`
        #[cfg(feature = "tmux_3_2")]
        if self.window {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        cmd
    }
}
