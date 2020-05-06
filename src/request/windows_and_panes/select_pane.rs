use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux select-pane [-DdeLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-pane [-DLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux select-pane [-DLRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux select-pane [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux select-pane [-p pane-index] [-t target-window]
/// (alias: selectp)
/// ```
#[derive(Default, Debug)]
pub struct SelectPane<'a, T: Display> {
    /// [-D] - pane below
    #[cfg(feature = "tmux_1_3")]
    pub down: Option<bool>,
    /// [-d] - disable input
    #[cfg(feature = "tmux_2_0")]
    pub disable: Option<bool>,
    /// [-e] - enable input
    #[cfg(feature = "tmux_2_0")]
    pub enable: Option<bool>,
    /// [-g] - show the current pane style
    #[cfg(feature = "tmux_2_1")]
    pub show_style: Option<bool>,
    /// [-L] - pane left
    #[cfg(feature = "tmux_1_3")]
    pub left: Option<bool>,
    /// [-l] - equivalent to last-pane command
    #[cfg(feature = "tmux_1_5")]
    pub last: Option<bool>,
    /// [-M] - clear marked pane
    #[cfg(feature = "tmux_2_1")]
    pub set_marked: Option<bool>,
    /// [-m] - set marked pane
    #[cfg(feature = "tmux_2_1")]
    pub clear_marked: Option<bool>,
    /// [-R] - pane right
    #[cfg(feature = "tmux_1_3")]
    pub right: Option<bool>,
    /// [-U] - pane above
    #[cfg(feature = "tmux_1_3")]
    pub up: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    /// [-P style] - set the style for a single pane
    #[cfg(feature = "tmux_2_1")]
    pub style: Option<&'a str>,
    /// [-T title] - title
    #[cfg(feature = "tmux_2_6")]
    pub title: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> SelectPane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SelectPaneBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_1_3")]
    pub down: Option<bool>,
    #[cfg(feature = "tmux_2_0")]
    pub disable: Option<bool>,
    #[cfg(feature = "tmux_2_0")]
    pub enable: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub show_style: Option<bool>,
    #[cfg(feature = "tmux_1_3")]
    pub left: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub last: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub set_marked: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub clear_marked: Option<bool>,
    #[cfg(feature = "tmux_1_3")]
    pub right: Option<bool>,
    #[cfg(feature = "tmux_1_3")]
    pub up: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub style: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub title: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> SelectPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn disable(&mut self) -> &mut Self {
        self.disable = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn enable(&mut self) -> &mut Self {
        self.enable = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn show_style(&mut self) -> &mut Self {
        self.show_style = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn set_marked(&mut self) -> &mut Self {
        self.set_marked = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn clear_marked(&mut self) -> &mut Self {
        self.clear_marked = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn style(&mut self, style: &'a str) -> &mut Self {
        self.style = Some(style);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }
    pub fn build(&self) -> SelectPane<'a, T> {
        SelectPane {
            #[cfg(feature = "tmux_1_3")]
            down: self.down,
            #[cfg(feature = "tmux_2_0")]
            disable: self.disable,
            #[cfg(feature = "tmux_2_0")]
            enable: self.enable,
            #[cfg(feature = "tmux_2_1")]
            show_style: self.show_style,
            #[cfg(feature = "tmux_1_3")]
            left: self.left,
            #[cfg(feature = "tmux_1_5")]
            last: self.last,
            #[cfg(feature = "tmux_2_1")]
            set_marked: self.set_marked,
            #[cfg(feature = "tmux_2_1")]
            clear_marked: self.clear_marked,
            #[cfg(feature = "tmux_1_3")]
            right: self.right,
            #[cfg(feature = "tmux_1_3")]
            up: self.up,
            #[cfg(feature = "tmux_3_1")]
            keep_zoomed: self.keep_zoomed,
            #[cfg(feature = "tmux_2_1")]
            style: self.style,
            #[cfg(feature = "tmux_2_6")]
            title: self.title,
            #[cfg(feature = "tmux_1_0")]
            target_pane: self.target_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SELECT_PANE: &'static str = "select-pane";

    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^2.6:
    /// ```text
    /// tmux select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux select-pane [-DdeLlRU] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux select-pane [-DLlRU] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^1.3:
    /// ```text
    /// tmux select-pane [-DLRU] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux select-pane [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux select-pane [-p pane-index] [-t target-window]
    /// (alias: selectp)
    /// ```
    pub fn select_pane<T: Display>(
        &mut self,
        select_pane: Option<&SelectPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;

        if let Some(select_pane) = select_pane {
            #[cfg(feature = "tmux_1_3")]
            if select_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            #[cfg(feature = "tmux_2_0")]
            if select_pane.disable.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_2_0")]
            if select_pane.enable.unwrap_or(false) {
                args.push(e_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if select_pane.show_style.unwrap_or(false) {
                args.push(g_KEY);
            }
            #[cfg(feature = "tmux_1_3")]
            if select_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if select_pane.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if select_pane.set_marked.unwrap_or(false) {
                args.push(M_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if select_pane.clear_marked.unwrap_or(false) {
                args.push(m_KEY);
            }
            #[cfg(feature = "tmux_1_3")]
            if select_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            #[cfg(feature = "tmux_1_3")]
            if select_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if select_pane.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if let Some(s) = select_pane.style {
                args.extend_from_slice(&[P_KEY, s])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = select_pane.title {
                args.extend_from_slice(&[T_KEY, s])
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(target_pane) = select_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output)
    }
}
