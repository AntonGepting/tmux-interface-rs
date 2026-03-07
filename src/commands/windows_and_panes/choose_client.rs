// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// choose-client [-NryZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.2:
/// ```text
/// choose-client [-NrZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.1:
/// ```text
/// choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.7:
/// ```text
/// choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.6:
/// ```text
/// choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=1.7:
/// ```text
/// choose-client [-F format] [-t target-window] [template]
/// ```
///
/// tmux >=1.0:
/// ```text
/// choose-client  [-t target-window] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseClient<'a> {
    /// `[-N]`
    #[cfg(feature = "tmux_2_6")]
    pub without_preview: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: bool,

    /// `[-y]`
    #[cfg(feature = "tmux_3_6")]
    pub disable_confirmation: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_2_7")]
    pub zoom: bool,

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
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

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[template]`
    #[cfg(feature = "tmux_1_5")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseClient<'a> {
    pub fn new() -> Self {
        Default::default()
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
    #[cfg(feature = "tmux_1_7")]
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

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
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
    #[cfg(feature = "tmux_1_5")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CHOOSE_CLIENT);

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
        #[cfg(feature = "tmux_1_7")]
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

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[template]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
