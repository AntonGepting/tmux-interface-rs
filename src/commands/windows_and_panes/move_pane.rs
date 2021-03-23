use crate::commands::constants::*;
use crate::PaneSize;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[derive(Debug, Clone)]
pub struct MovePane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for MovePane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(MOVE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> MovePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub fn left_above(&mut self) -> &mut Self {
        self.0.push_flag(b_KEY);
        self
    }

    /// [-d] -
    #[cfg(feature = "tmux_1_7")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(d_KEY);
        self
    }

    /// [-h] - full height
    #[cfg(feature = "tmux_1_7")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.0.push_flag(h_KEY);
        self
    }

    /// [-v] - full width
    #[cfg(feature = "tmux_1_7")]
    pub fn vertical(&mut self) -> &mut Self {
        self.0.push_flag(v_KEY);
        self
    }

    /// [-l size] - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_7")]
    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        match size {
            PaneSize::Size(size) => self.0.push_option(l_KEY, size.to_string()),
            PaneSize::Percentage(size) => self.0.push_option(l_KEY, format!("{}%", size)),
        };
        #[cfg(feature = "tmux_2_6")]
        match size {
            PaneSize::Size(size) => self.0.push_option(l_KEY, size.to_string()),
            PaneSize::Percentage(size) => self.0.push_option(p_KEY, size.to_string()),
        };
        self
    }

    /// [-s src-pane] - src-pane
    #[cfg(feature = "tmux_1_7")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(&mut self, src_pane: S) -> &mut Self {
        self.0.push_option(s_KEY, src_pane);
        self
    }

    /// [-t dst-pane] - dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(&mut self, dst_pane: S) -> &mut Self {
        self.0.push_option(t_KEY, dst_pane);
        self
    }

    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for MovePane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(MOVE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for MovePane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(MOVE_PANE)),
            ..Default::default()
        })
    }
}
