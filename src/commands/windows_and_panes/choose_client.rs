use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
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
#[derive(Debug, Clone)]
pub struct ChooseClient<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ChooseClient<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(CHOOSE_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> ChooseClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub fn without_preview(&mut self) -> &mut Self {
        self.0.push_flag(N_UPPERCASE_KEY);
        self
    }

    /// `[-r]` - reverse the sort order
    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_1")]
    pub fn zoom(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-F format]` - format
    #[cfg(feature = "tmux_1_7")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_UPPERCASE_KEY, format);
        self
    }

    /// `[-f filter]` - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    pub fn filter<S: Into<Cow<'a, str>>>(&mut self, filter: S) -> &mut Self {
        self.0.push_option(F_LOWERCASE_KEY, filter);
        self
    }

    /// `[-O sort-order]` - specify the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order<S: Into<Cow<'a, str>>>(&mut self, sort_order: S) -> &mut Self {
        self.0.push_option(O_UPPERCASE_KEY, sort_order);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_2_6")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `[template]` - template
    #[cfg(feature = "tmux_1_0")]
    pub fn template<S: Into<Cow<'a, str>>>(&mut self, template: S) -> &mut Self {
        self.0.push_param(template);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ChooseClient<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(CHOOSE_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ChooseClient<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(CHOOSE_CLIENT)),
            ..Default::default()
        })
    }
}
