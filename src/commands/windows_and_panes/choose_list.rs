// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Put a window into list choice mode
///
/// # Manual
///
/// tmux >=1.7 && <=1.9a:
/// ```text
/// choose-list [-l items] [-t target-window] [template]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChooseList<'a> {
    /// `[-l items]`
    #[cfg(feature = "tmux_1_7")]
    pub items: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_7")]
    pub target_window: Option<Cow<'a, str>>,

    /// `[template]`
    #[cfg(feature = "tmux_1_7")]
    pub template: Option<Cow<'a, str>>,
}

impl<'a> ChooseList<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-l items]`
    #[cfg(feature = "tmux_1_7")]
    pub fn items<S: Into<Cow<'a, str>>>(mut self, items: S) -> Self {
        self.items = Some(items.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_7")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[template]`
    #[cfg(feature = "tmux_1_7")]
    pub fn template<S: Into<Cow<'a, str>>>(mut self, template: S) -> Self {
        self.template = Some(template.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CHOOSE_LIST);

        // `[-l items]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(items) = self.items {
            cmd.push_option(L_LOWERCASE_KEY, items);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[template]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(template) = self.template {
            cmd.push_param(template);
        }

        cmd
    }
}
