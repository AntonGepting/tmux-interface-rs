use crate::commands::constants::*;
use crate::PaneSize;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
/// and move `src-pane` into the space
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```

#[derive(Debug, Clone)]
pub struct JoinPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for JoinPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(JOIN_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> JoinPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]` - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_2_6")]
    pub fn left_above(&mut self) -> &mut Self {
        self.0.push_flag(B_LOWERCASE_KEY);
        self
    }

    /// `[-d]` -
    #[cfg(feature = "tmux_1_2")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-f]` - creates a new pane spanning the full window height/width
    #[cfg(feature = "tmux_2_6")]
    pub fn full_size(&mut self) -> &mut Self {
        self.0.push_flag(F_LOWERCASE_KEY);
        self
    }

    /// `[-h]` - full height
    #[cfg(feature = "tmux_1_2")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.0.push_flag(H_LOWERCASE_KEY);
        self
    }

    /// `[-v]` - full width
    #[cfg(feature = "tmux_1_2")]
    pub fn vertical(&mut self) -> &mut Self {
        self.0.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `[-l size]` - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_2")]
    pub fn size(&mut self, size: &PaneSize) -> &mut Self {
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_1")))]
        match size {
            PaneSize::Size(size) => self.0.push_option(L_LOWERCASE_KEY, size.to_string()),
            PaneSize::Percentage(size) => self.0.push_option(L_LOWERCASE_KEY, format!("{}%", size)),
        };

        #[cfg(feature = "tmux_3_1")]
        match size {
            PaneSize::Size(size) => self.0.push_option(L_LOWERCASE_KEY, size.to_string()),
            PaneSize::Percentage(size) => self.0.push_option(P_LOWERCASE_KEY, size.to_string()),
        };

        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_2")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(&mut self, src_pane: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, src_pane);
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_2")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(&mut self, dst_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, dst_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for JoinPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(JOIN_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for JoinPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(JOIN_PANE)),
            ..Default::default()
        })
    }
}
