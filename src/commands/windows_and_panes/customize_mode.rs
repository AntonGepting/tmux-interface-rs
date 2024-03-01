use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.2:
/// ```text
/// customize-mode [-NZ] [-F format] [-f filter] [-t target-pane] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CustomizeMode<'a> {
    /// `[-N]` - start without the option information
    #[cfg(feature = "tmux_3_2")]
    pub no_info: bool,

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_2")]
    pub zoom: bool,

    /// `[-F format]` - format
    #[cfg(feature = "tmux_3_2")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]` - filter
    #[cfg(feature = "tmux_3_2")]
    pub filter: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_2")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[template]` - template
    #[cfg(feature = "tmux_3_2")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseTree<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-N]` - starts without the option information
    #[cfg(feature = "tmux_3_2")]
    pub fn no_info(mut self) -> Self {
        self.no_info = true;
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_2")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-F format]` - format
    #[cfg(feature = "tmux_3_2")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]` - filter
    #[cfg(feature = "tmux_3_2")]
    pub fn filter<S: Into<Cow<'a, str>>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[template]` - template
    #[cfg(feature = "tmux_3_2")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CUSTOMIZE_MODE);

        // `[-N]` - starts without the option information
        #[cfg(feature = "tmux_3_2")]
        if self.without_preview {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-Z]` - zoom the pane
        #[cfg(feature = "tmux_3_2")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-F format]` - format
        #[cfg(feature = "tmux_3_2")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-f filter]` - filter
        #[cfg(feature = "tmux_3_2")]
        if let Some(filter) = self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[template]` - template
        #[cfg(feature = "tmux_3_2")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
