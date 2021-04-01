use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Choose a specific layout for a window
///
/// # Manual
///
/// tmux ^2.7:
/// ```text
/// tmux select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux select-layout [-nop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-layout [-np] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux select-layout [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux select-layout [-t target-pane] layout-name
/// (alias: selectl)
/// ```
#[derive(Debug, Clone)]
pub struct SelectLayot<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SelectLayot<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SELECT_LAYOUT)),
            ..Default::default()
        })
    }
}

impl<'a> SelectLayot<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-E]` - spread the current pane and any panes next to it out evenly
    #[cfg(feature = "tmux_2_7")]
    pub fn spread(&mut self) -> &mut Self {
        self.0.push_flag(E_UPPERCASE_KEY);
        self
    }

    /// `[-n]` - next-layout equivalent
    #[cfg(feature = "tmux_1_5")]
    pub fn next_layout(&mut self) -> &mut Self {
        self.0.push_flag(N_LOWERCASE_KEY);
        self
    }

    /// `[-o]` - apply the last set layout if possible
    #[cfg(feature = "tmux_2_1")]
    pub fn last_layout(&mut self) -> &mut Self {
        self.0.push_flag(O_LOWERCASE_KEY);
        self
    }

    /// `[-p]` - previous-layout equivalent
    #[cfg(feature = "tmux_1_5")]
    pub fn previous_layout(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[layout-name]` - layout-name
    #[cfg(feature = "tmux_1_0")]
    pub fn layout_name<S: Into<Cow<'a, str>>>(&mut self, layout_name: S) -> &mut Self {
        self.0.push_param(layout_name);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SelectLayot<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SELECT_LAYOUT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SelectLayot<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SELECT_LAYOUT)),
            ..Default::default()
        })
    }
}
