// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a window into session choice mode
///
/// # Manual
///
/// tmux =1.7:
/// ```text
/// choose-session [-F format] [-t target-window] [template]
/// ```
///
/// tmux >=1.5:
/// ```text
/// choose-session [-t target-window] [template]
/// ```
///
/// tmux >=0.8:
/// ```text
/// choose-session [-t target-window]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseSession<'a> {
    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,

    /// `[template]`
    #[cfg(feature = "tmux_1_5")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
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

        cmd.name(CHOOSE_SESSION);

        // `[-F format]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(format) = self.format {
            cmd.push_option(F_UPPERCASE_KEY, format);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[template]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
