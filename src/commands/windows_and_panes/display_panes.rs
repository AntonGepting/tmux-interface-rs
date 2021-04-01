use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Display a visible indicator of each pane shown by `target-client`
///
/// # Manual
///
/// tmux ^2.9:
/// ```text
/// tmux display-panes [-b] [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux display-panes [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.3:
/// ```text
/// tmux display-panes [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux display-panes [-t target-client]
/// (alias: displayp)
/// ```
#[derive(Debug, Clone)]
pub struct DisplayPanes<'a>(pub TmuxCommand<'a>);

impl<'a> Default for DisplayPanes<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(DISPLAY_PANES)),
            ..Default::default()
        })
    }
}

impl<'a> DisplayPanes<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]`
    #[cfg(feature = "tmux_2_9")]
    pub fn not_block(&mut self) -> &mut Self {
        self.0.push_flag(B_LOWERCASE_KEY);
        self
    }

    /// `[-d duration]`
    #[cfg(feature = "tmux_2_6")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(&mut self, duration: S) -> &mut Self {
        self.0.push_option(D_LOWERCASE_KEY, duration);
        self
    }

    /// `[-d duration]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_client);
        self
    }

    /// `[template]`
    #[cfg(feature = "tmux_2_3")]
    pub fn template<S: Into<Cow<'a, str>>>(&mut self, template: S) -> &mut Self {
        self.0.push_param(template);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for DisplayPanes<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(DISPLAY_PANES)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for DisplayPanes<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(DISPLAY_PANES)),
            ..Default::default()
        })
    }
}
