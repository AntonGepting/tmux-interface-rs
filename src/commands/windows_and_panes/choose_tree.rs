// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// choose-tree [-GnrswyZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.2:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.1:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.7:
/// ```text
/// choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.6:
/// ```text
/// choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=1.8:
/// ```text
/// choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
///
/// tmux >=1.7:
/// ```text
/// choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseTree<'a> {
    /// `[-G]`
    #[cfg(feature = "tmux_2_7")]
    pub all: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_2_6")]
    pub without_preview: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: bool,

    /// `[-s]`
    #[cfg(feature = "tmux_1_7")]
    pub collapsed_sessions: bool,

    /// `[-u]`
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
    pub expanded_sessions: bool,

    /// `[-w]`
    #[cfg(feature = "tmux_1_7")]
    pub collapsed_windows: bool,

    /// `[-y]`
    #[cfg(feature = "tmux_3_6")]
    pub disable_confirmation: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_2_7")]
    pub zoom: bool,

    /// `[-F format]`
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]`
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<Cow<'a, str>>,

    /// `[-K key-format]`
    #[cfg(feature = "tmux_3_2")]
    pub key_format: Option<Cow<'a, str>>,

    /// `[-O sort-order]`
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<Cow<'a, str>>,

    /// `[-b session-template]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub session_template: Option<Cow<'a, str>>,

    /// `[-c window-template]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub window_template: Option<Cow<'a, str>>,

    /// `[-S format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub format: Option<Cow<'a, str>>,

    /// `[-W format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub format: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[template]`
    #[cfg(feature = "tmux_2_6")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseTree<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-G]`
    #[cfg(feature = "tmux_2_7")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_2_6")]
    pub fn without_preview(mut self) -> Self {
        self.without_preview = true;
        self
    }

    /// `[-r]`
    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(mut self) -> Self {
        self.reverse_sort_order = true;
        self
    }

    /// `[-s]`
    #[cfg(feature = "tmux_1_7")]
    pub fn collapsed_sessions(mut self) -> Self {
        self.collapsed_sessions = true;
        self
    }

    /// `[-u]`
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
    pub fn expanded_sessions(mut self) -> Self {
        self.expanded_sessions = true;
        self
    }

    /// `[-w]`
    #[cfg(feature = "tmux_1_7")]
    pub fn collapsed_windows(mut self) -> Self {
        self.collapsed_windows = true;
        self
    }

    /// `[-y]`
    #[cfg(feature = "tmux_3_6")]
    pub fn disable_confirmation(mut self) -> Self {
        self.disable_confirmation = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_2_6")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]`
    #[cfg(feature = "tmux_2_6")]
    pub fn filter<S: Into<Cow<'a, str>>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// `[-K key-format]`
    #[cfg(feature = "tmux_3_2")]
    pub fn key_format<S: Into<Cow<'a, str>>>(mut self, key_format: S) -> Self {
        self.key_format = Some(key_format.into());
        self
    }

    /// `[-O sort-order]`
    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order<S: Into<Cow<'a, str>>>(mut self, sort_order: S) -> Self {
        self.sort_order = Some(sort_order.into());
        self
    }

    /// `[-b session-template]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn session_template<S: Into<Cow<'a, str>>>(mut self, session_template: S) -> Self {
        self.session_template = Some(session_template.into());
        self
    }

    /// `[-c window-template]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn window_template<S: Into<Cow<'a, str>>>(mut self, window_template: S) -> Self {
        self.window_template = Some(window_template.into());
        self
    }

    /// `[-S format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-W format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
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

    /// `[template]`
    #[cfg(feature = "tmux_2_6")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CHOOSE_TREE);

        // `[-G]`
        #[cfg(feature = "tmux_2_7")]
        if self.all {
            cmd.push_flag(G_UPPERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_2_6")]
        if self.without_preview {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_3_1")]
        if self.reverse_sort_order {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-s]`
        #[cfg(feature = "tmux_1_7")]
        if self.collapsed_sessions {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-u]`
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_2_6")))]
        if self.expanded_sessions {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-w]`
        #[cfg(feature = "tmux_1_7")]
        if self.collapsed_windows {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-y]`
        #[cfg(feature = "tmux_3_6")]
        if self.disable_confirmation {
            cmd.push_flag(Y_LOWERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_2_7")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-f filter]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(filter) = self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter);
        }

        // `[-K key-format]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(key_format) = self.key_format {
            cmd.push_option(K_UPPERCASE_KEY, key_format);
        }

        // `[-O sort-order]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(sort_order) = self.sort_order {
            cmd.push_option(O_UPPERCASE_KEY, sort_order);
        }

        // `[-b session-template]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(session_template) = self.session_template {
            cmd.push_option(B_LOWERCASE_KEY, session_template);
        }

        // `[-c window-template]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(window_template) = self.window_template {
            cmd.push_option(C_LOWERCASE_KEY, window_template);
        }

        // `[-S format]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(format) = self.format {
            cmd.push_option(S_UPPERCASE_KEY, format);
        }

        // `[-W format]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(format) = self.format {
            cmd.push_option(W_UPPERCASE_KEY, format);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[template]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
