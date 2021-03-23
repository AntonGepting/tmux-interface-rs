use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Enter copy mode
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux copy-mode [-Meu] [-t target-pane]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux copy-mode [-u] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux copy-mode [-u] [-t target-window]
/// ```
#[derive(Debug, Clone)]
pub struct CopyMode<'a>(pub TmuxCommand<'a>);

impl<'a> Default for CopyMode<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(COPY_MODE)),
            ..Default::default()
        })
    }
}

impl<'a> CopyMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn mouse_drag(&mut self) -> &mut Self {
        self.0.push_flag(M_KEY);
        self
    }

    pub fn bottom_exit(&mut self) -> &mut Self {
        self.0.push_flag(e_KEY);
        self
    }

    pub fn page_up(&mut self) -> &mut Self {
        self.0.push_flag(u_KEY);
        self
    }

    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(t_KEY, target_pane);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for CopyMode<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(COPY_MODE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for CopyMode<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(COPY_MODE)),
            ..Default::default()
        })
    }
}
