use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Select the last (previously selected) pane
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux last-pane [-deZ] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux last-pane [-de] [-t target-window]
/// (alias: lastp)
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux last-pane [-t target-window]
/// (alias: lastp)
/// ```
// FIXME: versions and function parameters
#[derive(Debug, Clone)]
pub struct LastPane<'a>(TmuxCommand<'a>);

impl<'a> Default for LastPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LastPane::LAST_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> LastPane<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LAST_PANE: &'static str = "last-pane";
    #[cfg(feature = "use_cmd_alias")]
    const LAST_PANE: &'static str = "lastp";

    pub fn new() -> Self {
        Default::default()
    }

    /// [-d]
    #[cfg(feature = "tmux_2_0")]
    pub fn disable(&mut self) -> &mut Self {
        self.0.push_flag(d_KEY);
        self
    }

    /// [-e]
    #[cfg(feature = "tmux_2_0")]
    pub fn enable(&mut self) -> &mut Self {
        self.0.push_flag(e_KEY);
        self
    }

    /// [-Z]
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.0.push_flag(Z_KEY);
        self
    }

    /// [-t target-window]
    #[cfg(feature = "tmux_1_4")]
    pub fn target_window<S: Into<String>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(t_KEY, target_window);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}
