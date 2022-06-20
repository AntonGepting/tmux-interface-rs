use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^3.1:
/// ```text
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux choose-client [-F format] [-t target-window] [template]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux choose-client  [-t target-window] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseClient<'a> {
    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub without_preview: bool,

    /// `[-r]` - reverse the sort order
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: bool,

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_1")]
    pub zoom: bool,

    /// `[-F format]` - format
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]` - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<Cow<'a, str>>,

    /// `[-K key-format]` - format for each shortcut key
    #[cfg(feature = "tmux_3_2")]
    pub key_format: Option<Cow<'a, str>>,

    // XXX: type?
    /// `[-O sort-order]` - specify the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[template]` - template
    #[cfg(feature = "tmux_1_0")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub fn without_preview(mut self) -> Self {
        self.without_preview = true;
        self
    }

    /// `[-r]` - reverse the sort order
    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(mut self) -> Self {
        self.reverse_sort_order = true;
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_1")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-F format]` - format
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]` - specify an initial filter
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

    /// `[-O sort-order]` - specify the initial sort field
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
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[template]` - template
    #[cfg(feature = "tmux_1_0")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CHOOSE_CLIENT);

        // `[-N]` - start without the preview
        #[cfg(feature = "tmux_2_6")]
        if self.without_preview {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-r]` - reverse the sort order
        #[cfg(feature = "tmux_3_1")]
        if self.reverse_sort_order {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-Z]` - zoom the pane
        #[cfg(feature = "tmux_3_1")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-F format]` - format
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-f filter]` - specify an initial filter
        #[cfg(feature = "tmux_2_6")]
        if let Some(filter) = self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter);
        }

        // `[-K key-format]` - format for each shortcut key
        #[cfg(feature = "tmux_3_2")]
        if let Some(key_format) = self.key_format {
            cmd.push_option(K_UPPERCASE_KEY, key_format);
        }

        // `[-O sort-order]` - specify the initial sort field
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
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[template]` - template
        #[cfg(feature = "tmux_1_0")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
