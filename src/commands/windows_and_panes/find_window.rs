// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type FindW<'a> = FindWindow<'a>;

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// find-window [-iCNrTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=3.0a:
/// ```text
/// find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=2.9:
/// ```text
/// find-window [-CNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=2.6:
/// ```text
/// find-window [-CNT] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=1.7:
/// ```text
/// find-window [-CNT] [-F format] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux >=0.8:
/// ```text
/// find-window [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FindWindow<'a> {
    /// `[-i]`
    #[cfg(feature = "tmux_3_2")]
    pub ignore_case: bool,

    /// `[-C]`
    #[cfg(feature = "tmux_1_7")]
    pub only_visible: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_1_7")]
    pub only_name: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_3_0a")]
    pub regex: bool,

    /// `[-T]`
    #[cfg(feature = "tmux_1_7")]
    pub only_title: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_2_9")]
    pub zoom: bool,

    /// `[-F format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub format: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[match-string]`
    #[cfg(feature = "tmux_0_8")]
    pub match_string: Option<Cow<'a, str>>,
}

impl<'a> FindWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-i]`
    #[cfg(feature = "tmux_3_2")]
    pub fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_1_7")]
    pub fn only_visible(mut self) -> Self {
        self.only_visible = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_1_7")]
    pub fn only_name(mut self) -> Self {
        self.only_name = true;
        self
    }

    /// `[-r]`
    #[cfg(feature = "tmux_3_0a")]
    pub fn regex(mut self) -> Self {
        self.regex = true;
        self
    }

    /// `[-T]`
    #[cfg(feature = "tmux_1_7")]
    pub fn only_title(mut self) -> Self {
        self.only_title = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_2_9")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-F format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[match-string]`
    #[cfg(feature = "tmux_0_8")]
    pub fn match_string<S: Into<Cow<'a, str>>>(mut self, match_string: S) -> Self {
        self.match_string = Some(match_string.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(FIND_WINDOW);

        // `[-i]`
        #[cfg(feature = "tmux_3_2")]
        if self.ignore_case {
            cmd.push_flag(I_LOWERCASE_KEY);
        }

        // `[-C]`
        #[cfg(feature = "tmux_1_7")]
        if self.only_visible {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_1_7")]
        if self.only_name {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_3_0a")]
        if self.regex {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-T]`
        #[cfg(feature = "tmux_1_7")]
        if self.only_title {
            cmd.push_flag(T_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_2_9")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-F format]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[match-string]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(match_string) = self.match_string {
            cmd.push_param(match_string);
        }

        cmd
    }
}
