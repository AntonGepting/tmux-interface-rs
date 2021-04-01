use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Stucture for putting a pane into buffer mode
///
/// # Manual
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
#[derive(Debug, Clone)]
pub struct ChooseBuffer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ChooseBuffer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(CHOOSE_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> ChooseBuffer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub fn no_preview(&mut self) -> &mut Self {
        self.0.push_flag(N_UPPERCASE_KEY);
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-r]` - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-F format]` - specify the format for each item in the list
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

    /// `[-O sort-order]` - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order<S: Into<Cow<'a, str>>>(&mut self, sort_order: S) -> &mut Self {
        self.0.push_option(O_UPPERCASE_KEY, sort_order);
        self
    }

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_1_3")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[template]` - specify the template
    #[cfg(feature = "tmux_1_3")]
    pub fn template<S: Into<Cow<'a, str>>>(&mut self, template: S) -> &mut Self {
        self.0.push_param(template);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ChooseBuffer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(CHOOSE_BUFFER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ChooseBuffer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(CHOOSE_BUFFER)),
            ..Default::default()
        })
    }
}
