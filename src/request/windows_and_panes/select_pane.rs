use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux 2.6
/// ```text
/// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SelectPane<'a, T: Display> {
    /// [-D] - pane below
    pub down: Option<bool>,
    /// [-d] - disable input
    pub disable: Option<bool>,
    /// [-e] - enable input
    pub enable: Option<bool>,
    /// [-L] - pane left
    pub left: Option<bool>,
    /// [-l] - equivalent to last-pane command
    pub last: Option<bool>,
    /// [-M] - clear marked pane
    pub set_marked: Option<bool>,
    /// [-m] - set marked pane
    pub clear_marked: Option<bool>,
    /// [-R] - pane right
    pub right: Option<bool>,
    /// [-U] - pane above
    pub up: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    pub keep_zoomed: Option<bool>,
    /// [-T title] - title
    pub title: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SelectPane<'a, T: Display> {
    /// [-D] - pane below
    pub down: Option<bool>,
    /// [-d] - disable input
    pub disable: Option<bool>,
    /// [-e] - enable input
    pub enable: Option<bool>,
    /// [-g] - show the current pane style
    pub show_style: Option<bool>,
    /// [-L] - pane left
    pub left: Option<bool>,
    /// [-l] - equivalent to last-pane command
    pub last: Option<bool>,
    /// [-M] - clear marked pane
    pub set_marked: Option<bool>,
    /// [-m] - set marked pane
    pub clear_marked: Option<bool>,
    /// [-R] - pane right
    pub right: Option<bool>,
    /// [-U] - pane above
    pub up: Option<bool>,
    /// [-P style] - set the style for a single pane
    pub style: Option<&'a str>,
    /// [-T title] - title
    pub title: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> SelectPane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SelectPaneBuilder<'a, T: Display> {
    pub down: Option<bool>,
    pub disable: Option<bool>,
    pub enable: Option<bool>,
    pub left: Option<bool>,
    pub last: Option<bool>,
    pub set_marked: Option<bool>,
    pub clear_marked: Option<bool>,
    pub right: Option<bool>,
    pub up: Option<bool>,
    pub keep_zoomed: Option<bool>,
    pub title: Option<&'a str>,
    pub target_pane: Option<&'a T>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> SelectPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.disable = Some(true);
        self
    }

    pub fn enable(&mut self) -> &mut Self {
        self.enable = Some(true);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn set_marked(&mut self) -> &mut Self {
        self.set_marked = Some(true);
        self
    }

    pub fn clear_marked(&mut self) -> &mut Self {
        self.clear_marked = Some(true);
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = Some(true);
        self
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> SelectPane<'a, T> {
        SelectPane {
            down: self.down,
            disable: self.disable,
            enable: self.enable,
            left: self.left,
            last: self.last,
            set_marked: self.set_marked,
            clear_marked: self.clear_marked,
            right: self.right,
            up: self.up,
            keep_zoomed: self.keep_zoomed,
            title: self.title,
            target_pane: self.target_pane,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SelectPaneBuilder<'a, T: Display> {
    pub down: Option<bool>,
    pub disable: Option<bool>,
    pub enable: Option<bool>,
    pub show_style: Option<bool>,
    pub left: Option<bool>,
    pub last: Option<bool>,
    pub set_marked: Option<bool>,
    pub clear_marked: Option<bool>,
    pub right: Option<bool>,
    pub up: Option<bool>,
    pub style: Option<&'a str>,
    pub title: Option<&'a str>,
    pub target_pane: Option<&'a T>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> SelectPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.disable = Some(true);
        self
    }

    pub fn enable(&mut self) -> &mut Self {
        self.enable = Some(true);
        self
    }

    pub fn show_style(&mut self) -> &mut Self {
        self.show_style = Some(true);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn set_marked(&mut self) -> &mut Self {
        self.set_marked = Some(true);
        self
    }

    pub fn clear_marked(&mut self) -> &mut Self {
        self.clear_marked = Some(true);
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    pub fn style(&mut self, style: &'a str) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> SelectPane<'a, T> {
        SelectPane {
            down: self.down,
            disable: self.disable,
            enable: self.enable,
            show_style: self.show_style,
            left: self.left,
            last: self.last,
            set_marked: self.set_marked,
            clear_marked: self.clear_marked,
            right: self.right,
            up: self.up,
            style: self.style,
            title: self.title,
            target_pane: self.target_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SELECT_PANE: &'static str = "select-pane";

    /// Make pane `target-pane` the active pane in window `target-window`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn select_pane<T: Display>(
        &mut self,
        select_pane: Option<&SelectPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(select_pane) = select_pane {
            if select_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if select_pane.disable.unwrap_or(false) {
                args.push(d_KEY);
            }
            if select_pane.enable.unwrap_or(false) {
                args.push(e_KEY);
            }
            if select_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if select_pane.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            if select_pane.set_marked.unwrap_or(false) {
                args.push(M_KEY);
            }
            if select_pane.clear_marked.unwrap_or(false) {
                args.push(m_KEY);
            }
            if select_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if select_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if select_pane.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(s) = select_pane.title {
                args.extend_from_slice(&[T_KEY, s])
            }
            if let Some(target_pane) = select_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output)
    }

    /// Make pane `target-pane` the active pane in window `target-window`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn select_pane<T: Display>(
        &mut self,
        select_pane: Option<&SelectPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(select_pane) = select_pane {
            if select_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if select_pane.disable.unwrap_or(false) {
                args.push(d_KEY);
            }
            if select_pane.enable.unwrap_or(false) {
                args.push(e_KEY);
            }
            if select_pane.show_style.unwrap_or(false) {
                args.push(g_KEY);
            }
            if select_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if select_pane.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            if select_pane.set_marked.unwrap_or(false) {
                args.push(M_KEY);
            }
            if select_pane.clear_marked.unwrap_or(false) {
                args.push(m_KEY);
            }
            if select_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if select_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(s) = select_pane.style {
                args.extend_from_slice(&[P_KEY, s])
            }
            if let Some(s) = select_pane.title {
                args.extend_from_slice(&[T_KEY, s])
            }
            if let Some(target_pane) = select_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output)
    }
}
