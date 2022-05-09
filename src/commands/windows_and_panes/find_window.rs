use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^2.6:
/// ```text
/// tmux find-window [-CNT] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^1.7:
/// ```text
/// tmux find-window [-CNT] [-F format] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^0.8:
/// ```text
/// tmux find-window [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Debug, Default, Clone)]
pub struct FindWindow<'a> {
    /// `[-r]` - regular expression
    #[cfg(feature = "tmux_3_0")]
    pub regex: bool,

    /// `[-C]` - match only visible window contents
    #[cfg(feature = "tmux_1_7")]
    pub only_visible: bool,

    /// `[-N]` - match only the window name
    #[cfg(feature = "tmux_1_7")]
    pub only_name: bool,

    /// `[-T]` - match only the window title
    #[cfg(feature = "tmux_1_7")]
    pub only_title: bool,

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_0")]
    pub zoom: bool,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_8")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> FindWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-r]` - regular expression
    #[cfg(feature = "tmux_3_0")]
    pub fn regex(&mut self) -> &mut Self {
        self.regex = true;
        self
    }

    /// `[-C]` - match only visible window contents
    #[cfg(feature = "tmux_1_7")]
    pub fn only_visible(&mut self) -> &mut Self {
        self.only_visible = true;
        self
    }

    /// `[-N]` - match only the window name
    #[cfg(feature = "tmux_1_7")]
    pub fn only_name(&mut self) -> &mut Self {
        self.only_name = true;
        self
    }

    /// `[-T]` - match only the window title
    #[cfg(feature = "tmux_1_7")]
    pub fn only_title(&mut self) -> &mut Self {
        self.only_title = true;
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_0")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = true;
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(FIND_WINDOW);

        // `[-r]` - regular expression
        #[cfg(feature = "tmux_3_0")]
        if self.regex {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-C]` - match only visible window contents
        #[cfg(feature = "tmux_1_7")]
        if self.only_visible {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-N]` - match only the window name
        #[cfg(feature = "tmux_1_7")]
        if self.only_name {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-T]` - match only the window title
        #[cfg(feature = "tmux_1_7")]
        if self.only_title {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-Z]` - zoom the pane
        #[cfg(feature = "tmux_3_0")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        cmd
    }
}
