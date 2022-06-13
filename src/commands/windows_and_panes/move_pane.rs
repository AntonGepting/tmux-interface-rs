use crate::commands::constants::*;
use crate::PaneSize;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MovePane<'a> {
    /// `[-b]` - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub left_above: bool,

    /// `[-d]` -
    #[cfg(feature = "tmux_1_7")]
    pub detached: bool,

    /// `[-h]` - full height
    #[cfg(feature = "tmux_1_7")]
    pub horizontal: bool,

    /// `[-v]` - full width
    #[cfg(feature = "tmux_1_7")]
    pub vertical: bool,

    /// `[-l size]` - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<&'a PaneSize>,

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_7")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub dst_pane: Option<Cow<'a, str>>,
}

impl<'a> MovePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]` - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub fn left_above(mut self) -> Self {
        self.left_above = true;
        self
    }

    /// `[-d]` -
    #[cfg(feature = "tmux_1_7")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-h]` - full height
    #[cfg(feature = "tmux_1_7")]
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// `[-v]` - full width
    #[cfg(feature = "tmux_1_7")]
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// `[-l size]` - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_7")]
    pub fn size(mut self, size: &'a PaneSize) -> Self {
        self.size = Some(size);
        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_7")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(mut self, src_pane: S) -> Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(mut self, dst_pane: S) -> Self {
        self.dst_pane = Some(dst_pane.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(MOVE_PANE);

        // `[-b]` - cause src-pane to be joined to left of or above dst-pane
        #[cfg(feature = "tmux_1_7")]
        if self.left_above {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-d]` -
        #[cfg(feature = "tmux_1_7")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-h]` - full height
        #[cfg(feature = "tmux_1_7")]
        if self.horizontal {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-v]` - full width
        #[cfg(feature = "tmux_1_7")]
        if self.vertical {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-l size]` - specify the size of the new pane in lines/columns
        #[cfg(feature = "tmux_1_7")]
        if let Some(size) = &self.size {
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            match size {
                PaneSize::Size(size) => cmd.push_option(L_LOWERCASE_KEY, size.to_string()),
                PaneSize::Percentage(size) => {
                    cmd.push_option(L_LOWERCASE_KEY, format!("{}%", size))
                }
            };
            #[cfg(feature = "tmux_2_6")]
            match size {
                PaneSize::Size(size) => cmd.push_option(L_LOWERCASE_KEY, size.to_string()),
                PaneSize::Percentage(size) => cmd.push_option(P_LOWERCASE_KEY, size.to_string()),
            };
        }

        // `[-s src-pane]` - src-pane
        #[cfg(feature = "tmux_1_7")]
        if let Some(src_pane) = self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane);
        }

        // `[-t dst-pane]` - dst-pane
        #[cfg(feature = "tmux_1_7")]
        if let Some(dst_pane) = self.dst_pane {
            cmd.push_option(T_LOWERCASE_KEY, dst_pane);
        }

        cmd
    }
}
