use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^3.1:
/// ```text
/// choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.8:
/// ```text
/// choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
///
/// tmux ^1.7:
/// ```text
/// choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseTree<'a> {
    /// `[-G]` - include all sessions in any session groups in the tree rather than only the first
    #[cfg(feature = "tmux_2_7")]
    pub all: bool,

    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_7")]
    pub without_preview: bool,

    /// `[-r]` - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: bool,

    /// `[-s]` - start with collapsed sessions
    #[cfg(feature = "tmux_1_7")]
    pub collapsed_sessions: bool,

    /// `[-w]` - start with collapsed windows
    #[cfg(feature = "tmux_1_8")]
    pub collapsed_windows: bool,

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub zoom: bool,

    /// `[-F format]` - format
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]` - filter
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<Cow<'a, str>>,

    /// `[-K key-format]` - format for each shortcut key
    #[cfg(feature = "tmux_3_2")]
    pub key_format: Option<Cow<'a, str>>,

    /// `[-O sort-order]` - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[template]` - template
    #[cfg(feature = "tmux_2_6")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseTree<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-G]` - include all sessions in any session groups in the tree rather than only the first
    #[cfg(feature = "tmux_2_7")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_7")]
    pub fn without_preview(mut self) -> Self {
        self.without_preview = true;
        self
    }

    /// `[-r]` - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(mut self) -> Self {
        self.reverse_sort_order = true;
        self
    }

    /// `[-s]` - start with collapsed sessions
    #[cfg(feature = "tmux_1_7")]
    pub fn collapsed_sessions(mut self) -> Self {
        self.collapsed_sessions = true;
        self
    }

    /// `[-w]` - start with collapsed windows
    #[cfg(feature = "tmux_1_8")]
    pub fn collapsed_windows(mut self) -> Self {
        self.collapsed_windows = true;
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-F format]` - format
    #[cfg(feature = "tmux_2_6")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]` - filter
    #[cfg(feature = "tmux_2_6")]
    pub fn filter<S: Into<Cow<'a, str>>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// `[-K key-format]` - format for each shortcut key
    #[cfg(feature = "tmux_3_2")]
    pub fn key_format<S: Into<Cow<'a, str>>>(mut self, key_format: S) -> Self {
        self.key_format = Some(key_format.into());
        self
    }

    /// `[-O sort-order]` - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order<S: Into<Cow<'a, str>>>(mut self, sort_order: S) -> Self {
        self.sort_order = Some(sort_order.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_2_6")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[template]` - template
    #[cfg(feature = "tmux_2_6")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CHOOSE_TREE);

        // `[-G]` - include all sessions in any session groups in the tree rather than only the first
        #[cfg(feature = "tmux_2_7")]
        if self.all {
            cmd.push_flag(G_UPPERCASE_KEY);
        }

        // `[-N]` - start without the preview
        #[cfg(feature = "tmux_2_7")]
        if self.without_preview {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-r]` - reverses the sort order
        #[cfg(feature = "tmux_3_1")]
        if self.reverse_sort_order {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-s]` - start with collapsed sessions
        #[cfg(feature = "tmux_1_7")]
        if self.collapsed_sessions {
            cmd.push_flag(S_LOWERCASE_KEY);
        }

        // `[-w]` - start with collapsed windows
        #[cfg(feature = "tmux_1_8")]
        if self.collapsed_windows {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[-Z]` - zoom the pane
        #[cfg(feature = "tmux_2_7")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-F format]` - format
        #[cfg(feature = "tmux_2_6")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-f filter]` - filter
        #[cfg(feature = "tmux_2_6")]
        if let Some(filter) = self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter);
        }

        // `[-K key-format]` - format for each shortcut key
        #[cfg(feature = "tmux_3_2")]
        if let Some(key_format) = self.key_format {
            cmd.push_option(K_UPPERCASE_KEY, key_format);
        }

        // `[-O sort-order]` - specifies the initial sort field
        #[cfg(feature = "tmux_2_6")]
        if let Some(sort_order) = self.sort_order {
            cmd.push_option(O_UPPERCASE_KEY, sort_order);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_2_6")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-t target-window]` - target-window
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[template]` - template
        #[cfg(feature = "tmux_2_6")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
