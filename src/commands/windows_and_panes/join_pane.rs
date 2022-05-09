use crate::commands::constants::*;
use crate::PaneSize;
use crate::TmuxCommand;
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

#[derive(Debug, Default, Clone)]
pub struct JoinPane<'a> {
    /// `[-b]` - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_2_6")]
    pub left_above: bool,

    /// `[-d]` -
    #[cfg(feature = "tmux_1_2")]
    pub detached: bool,

    /// `[-f]` - creates a new pane spanning the full window height/width
    #[cfg(feature = "tmux_2_6")]
    pub full_size: bool,

    /// `[-h]` - full height
    #[cfg(feature = "tmux_1_2")]
    pub horizontal: bool,

    /// `[-v]` - full width
    #[cfg(feature = "tmux_1_2")]
    pub vertical: bool,

    /// `[-l size]`
    /// `[-l size | -p percentage]` - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_2")]
    pub size: Option<&'a PaneSize>,

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_2")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_2")]
    pub dst_pane: Option<Cow<'a, str>>,
}

impl<'a> JoinPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]` - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_2_6")]
    pub fn left_above(&mut self) -> &mut Self {
        self.left_above = true;
        self
    }

    /// `[-d]` -
    #[cfg(feature = "tmux_1_2")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = true;
        self
    }

    /// `[-f]` - creates a new pane spanning the full window height/width
    #[cfg(feature = "tmux_2_6")]
    pub fn full_size(&mut self) -> &mut Self {
        self.full_size = true;
        self
    }

    /// `[-h]` - full height
    #[cfg(feature = "tmux_1_2")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = true;
        self
    }

    /// `[-v]` - full width
    #[cfg(feature = "tmux_1_2")]
    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = true;
        self
    }

    /// `[-l size]`
    /// `[-l size | -p percentage]` - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_2")]
    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_2")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(&mut self, src_pane: S) -> &mut Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_2")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(&mut self, dst_pane: S) -> &mut Self {
        self.dst_pane = Some(dst_pane.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(JOIN_PANE);

        // `[-b]` - cause src-pane to be joined to left of or above dst-pane
        #[cfg(feature = "tmux_2_6")]
        if self.left_above {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-d]` -
        #[cfg(feature = "tmux_1_2")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-f]` - creates a new pane spanning the full window height/width
        #[cfg(feature = "tmux_2_6")]
        if self.full_size {
            cmd.push_flag(F_LOWERCASE_KEY);
        }

        // `[-h]` - full height
        #[cfg(feature = "tmux_1_2")]
        if self.horizontal {
            cmd.push_flag(H_LOWERCASE_KEY);
        }

        // `[-v]` - full width
        #[cfg(feature = "tmux_1_2")]
        if self.vertical {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-l size]`
        // `[-l size | -p percentage]` - specify the size of the new pane in lines/columns
        #[cfg(feature = "tmux_1_2")]
        if let Some(size) = &self.size {
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_1")))]
            match size {
                PaneSize::Size(size) => cmd.push_option(L_LOWERCASE_KEY, size.to_string()),
                PaneSize::Percentage(size) => {
                    cmd.push_option(L_LOWERCASE_KEY, format!("{}%", size))
                }
            };

            #[cfg(feature = "tmux_3_1")]
            match size {
                PaneSize::Size(size) => cmd.push_option(L_LOWERCASE_KEY, size.to_string()),
                PaneSize::Percentage(size) => cmd.push_option(P_LOWERCASE_KEY, size.to_string()),
            };
        }

        // `[-s src-pane]` - src-pane
        #[cfg(feature = "tmux_1_2")]
        if let Some(src_pane) = &self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane.as_ref());
        }

        // `[-t dst-pane]` - dst-pane
        #[cfg(feature = "tmux_1_2")]
        if let Some(dst_pane) = &self.dst_pane {
            cmd.push_option(T_LOWERCASE_KEY, dst_pane.as_ref());
        }

        cmd
    }
}
