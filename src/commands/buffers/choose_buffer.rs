use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Stucture for putting a pane into buffer mode
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux choose-buffer [-NZr] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^3.1:
/// ```text
/// tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux choose-buffer [-F format] [-t target-pane] [template]
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux choose-buffer [-t target-pane] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseBuffer<'a> {
    /// [-N] - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub no_preview: bool,

    /// [-Z] - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub zoom: bool,

    /// [-r] - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: bool,

    /// [-F] - specify the format for each item in the list
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    /// [-f filter] - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<Cow<'a, str>>,

    /// [-O sort-order] - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<Cow<'a, str>>,

    /// [-t target-pane] - specify the target pane
    #[cfg(feature = "tmux_1_3")]
    pub target_pane: Option<Cow<'a, str>>,

    /// [template] - specify the template
    #[cfg(feature = "tmux_1_3")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub fn no_preview(&mut self) -> &mut Self {
        self.no_preview = true;
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = true;
        self
    }

    /// `[-r]` - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = true;
        self
    }

    /// `[-F format]` - specify the format for each item in the list
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]` - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    pub fn filter<S: Into<Cow<'a, str>>>(&mut self, filter: S) -> &mut Self {
        self.filter = Some(filter.into());
        self
    }

    /// `[-K key-format]` - format for each shortcut key
    #[cfg(feature = "tmux_3_2")]
    pub fn key_format<S: Into<Cow<'a, str>>>(&mut self, key_format: S) -> &mut Self {
        self.key_format = Some(key_format.into());
        self
    }

    /// `[-O sort-order]` - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order<S: Into<Cow<'a, str>>>(&mut self, sort_order: S) -> &mut Self {
        self.sort_order = Some(sort_order.into());
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_3")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[template]` - specify the template
    #[cfg(feature = "tmux_1_3")]
    pub fn template<S: Into<Cow<'a, str>>>(&mut self, template: S) -> &mut Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(CHOOSE_BUFFER);

        // `[-N]` - start without the preview
        #[cfg(feature = "tmux_2_6")]
        if self.no_preview {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-Z]` - zoom the pane
        #[cfg(feature = "tmux_2_7")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-r]` - reverses the sort order
        #[cfg(feature = "tmux_3_1")]
        if self.reverse_sort_order {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-F format]` - specify the format for each item in the list
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = &self.format {
            cmd.push_option(F_UPPERCASE_KEY, format.as_ref());
        }

        // `[-f filter]` - specify an initial filter
        #[cfg(feature = "tmux_2_6")]
        if let Some(filter) = &self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter.as_ref());
        }

        // `[-K key-format]` - format for each shortcut key
        #[cfg(feature = "tmux_3_2")]
        if let Some(key_format) = self.key_format {
            cmd.push_option(K_UPPERCASE_KEY, key_format);
        }

        // `[-O sort-order]` - specifies the initial sort field
        #[cfg(feature = "tmux_2_6")]
        if let Some(sort_order) = &self.sort_order {
            cmd.push_option(O_UPPERCASE_KEY, sort_order.as_ref());
        }

        // `[-t target-pane]` - specify the target pane
        #[cfg(feature = "tmux_1_3")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        // `[template]` - specify the template
        #[cfg(feature = "tmux_1_3")]
        if let Some(template) = &self.template {
            cmd.push_param(template.as_ref());
        }

        cmd
    }
}
