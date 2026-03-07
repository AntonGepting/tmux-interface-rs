// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a pane into customize mode
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// customize-mode [-NZ] [-F format] [-f filter] [-t target-pane] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CustomizeMode<'a> {
    /// `[-N]`
    #[cfg(feature = "tmux_3_2")]
    pub without_option_info: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_3_2")]
    pub zoom: bool,

    /// `[-F format]`
    #[cfg(feature = "tmux_3_2")]
    pub format: Option<Cow<'a, str>>,

    /// `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    pub filter: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[template]`
    #[cfg(feature = "tmux_3_2")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> CustomizeMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_2")]
    pub fn without_option_info(mut self) -> Self {
        self.without_option_info = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_2")]
    pub fn zoom(mut self) -> Self {
        self.zoom = true;
        self
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_3_2")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    pub fn filter<S: Into<Cow<'a, str>>>(mut self, filter: S) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[template]`
    #[cfg(feature = "tmux_3_2")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CUSTOMIZE_MODE);

        // `[-N]`
        #[cfg(feature = "tmux_3_2")]
        if self.without_option_info {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_3_2")]
        if self.zoom {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-F format]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-f filter]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(filter) = self.filter {
            cmd.push_option(F_LOWERCASE_KEY, filter);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[template]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
