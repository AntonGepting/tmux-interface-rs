// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SelectL<'a> = SelectLayout<'a>;

/// Choose a specific layout for a window
///
/// # Manual
///
/// tmux >=2.7:
/// ```text
/// select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux >=2.1:
/// ```text
/// select-layout [-nop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux >=1.5:
/// ```text
/// select-layout [-np] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SelectLayout<'a> {
    /// `[-E]`
    #[cfg(feature = "tmux_2_7")]
    pub spread: bool,

    /// `[-n]`
    #[cfg(feature = "tmux_1_5")]
    pub next_layout: bool,

    /// `[-o]`
    #[cfg(feature = "tmux_2_1")]
    pub last_layout: bool,

    /// `[-p]`
    #[cfg(feature = "tmux_1_5")]
    pub previous_layout: bool,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_7")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[layout-name]`
    #[cfg(feature = "tmux_1_5")]
    pub layout_name: Option<Cow<'a, str>>,
}

impl<'a> SelectLayout<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-E]`
    #[cfg(feature = "tmux_2_7")]
    pub fn spread(mut self) -> Self {
        self.spread = true;
        self
    }

    /// `[-n]`
    #[cfg(feature = "tmux_1_5")]
    pub fn next_layout(mut self) -> Self {
        self.next_layout = true;
        self
    }

    /// `[-o]`
    #[cfg(feature = "tmux_2_1")]
    pub fn last_layout(mut self) -> Self {
        self.last_layout = true;
        self
    }

    /// `[-p]`
    #[cfg(feature = "tmux_1_5")]
    pub fn previous_layout(mut self) -> Self {
        self.previous_layout = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_7")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[layout-name]`
    #[cfg(feature = "tmux_1_5")]
    pub fn layout_name<S: Into<Cow<'a, str>>>(mut self, layout_name: S) -> Self {
        self.layout_name = Some(layout_name.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SELECT_LAYOUT);

        // `[-E]`
        #[cfg(feature = "tmux_2_7")]
        if self.spread {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-n]`
        #[cfg(feature = "tmux_1_5")]
        if self.next_layout {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-o]`
        #[cfg(feature = "tmux_2_1")]
        if self.last_layout {
            cmd.push_flag(O_LOWERCASE_KEY);
        }

        // `[-p]`
        #[cfg(feature = "tmux_1_5")]
        if self.previous_layout {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_2_7")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[layout-name]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(layout_name) = self.layout_name {
            cmd.push_param(layout_name);
        }

        cmd
    }
}
