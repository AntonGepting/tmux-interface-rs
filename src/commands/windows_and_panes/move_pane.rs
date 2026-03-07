// auto-generated file
//

use crate::commands::constants::*;
use crate::PaneSize;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type MoveP<'a> = MovePane<'a>;

/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// move-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux >=3.1:
/// ```text
/// move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux >=1.7:
/// ```text
/// move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MovePane<'a> {
    /// `[-b]`
    #[cfg(feature = "tmux_1_7")]
    pub left_above: bool,

    /// `[-d]`
    #[cfg(feature = "tmux_1_7")]
    pub detached: bool,

    /// `[-f]`
    #[cfg(feature = "tmux_3_2")]
    pub full_size: bool,

    /// `[-h]`
    #[cfg(feature = "tmux_1_7")]
    pub horizontal: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_1_7")]
    pub vertical: bool,

    /// `[-l size]`
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<&'a PaneSize>,

    /// `[-p percentage]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
    pub percentage: Option<Cow<'a, str>>,

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_1_7")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t dst-pane]`
    #[cfg(feature = "tmux_1_7")]
    pub dst_pane: Option<Cow<'a, str>>,
}

impl<'a> MovePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]`
    #[cfg(feature = "tmux_1_7")]
    pub fn left_above(mut self) -> Self {
        self.left_above = true;
        self
    }

    /// `[-d]`
    #[cfg(feature = "tmux_1_7")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-f]`
    #[cfg(feature = "tmux_3_2")]
    pub fn full_size(mut self) -> Self {
        self.full_size = true;
        self
    }

    /// `[-h]`
    #[cfg(feature = "tmux_1_7")]
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_1_7")]
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// `[-l size]`
    #[cfg(feature = "tmux_1_7")]
    pub fn size(mut self, size: &'a PaneSize) -> Self {
        self.size = Some(size);
        self
    }

    /// `[-p percentage]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
    pub fn percentage<S: Into<Cow<'a, str>>>(mut self, percentage: S) -> Self {
        self.percentage = Some(percentage.into());
        self
    }

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_1_7")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(mut self, src_pane: S) -> Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t dst-pane]`
    #[cfg(feature = "tmux_1_7")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(mut self, dst_pane: S) -> Self {
        self.dst_pane = Some(dst_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(MOVE_PANE);

        // `[-b]`
        #[cfg(feature = "tmux_1_7")]
        if self.left_above {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-d]`
        #[cfg(feature = "tmux_1_7")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-f]`
        #[cfg(feature = "tmux_3_2")]
        if self.full_size {
            cmd.push_flag(F_LOWERCASE_KEY);
        }

        // `[-h]`
        #[cfg(feature = "tmux_1_7")]
        if self.horizontal {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_1_7")]
        if self.vertical {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-l size]`
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

        // `[-p percentage]`
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_3_1")))]
        if let Some(percentage) = self.percentage {
            cmd.push_option(P_LOWERCASE_KEY, percentage);
        }

        // `[-s src-pane]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(src_pane) = self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane);
        }

        // `[-t dst-pane]`
        #[cfg(feature = "tmux_1_7")]
        if let Some(dst_pane) = self.dst_pane {
            cmd.push_option(T_LOWERCASE_KEY, dst_pane);
        }

        cmd
    }
}
