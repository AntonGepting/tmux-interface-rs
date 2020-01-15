use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Choose a specific layout for a window
///
/// # Manual
///
/// tmux X.X
/// ```text
/// tmux select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux 2.6
/// ```text
/// tmux select-layout [-nop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SelectLayot<'a> {
    /// [-E] - spread the current pane and any panes next to it out evenly
    pub spread: Option<bool>,
    /// [-n] - next-layout equivalent
    pub next: Option<bool>,
    /// [-o] - apply the last set layout if possible
    pub last: Option<bool>,
    /// [-p] - previous-layout equivalent
    pub previous: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [layout-name] - layout-name
    pub layout_name: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SelectLayot<'a> {
    /// [-n] - next-layout equivalent
    pub next: Option<bool>,
    /// [-o] - apply the last set layout if possible
    pub last: Option<bool>,
    /// [-p] - previous-layout equivalent
    pub previous: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [layout-name] - layout-name
    pub layout_name: Option<&'a str>,
}

impl<'a> SelectLayot<'a> {
    pub fn new() -> SelectLayot<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const SELECT_LAYOUT: &'static str = "select-layout";

    /// Choose a specific layout for a window
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux select-layout [-nop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn select_layout(&mut self, select_layout: Option<&SelectLayot>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_layout) = select_layout {
            if cfg!(not(feature = "tmux_2_6")) {
                if select_layout.spread.unwrap_or(false) {
                    args.push(E_KEY);
                }
            }
            if select_layout.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if select_layout.last.unwrap_or(false) {
                args.push(o_KEY);
            }
            if select_layout.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if let Some(s) = select_layout.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = select_layout.layout_name {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }

    /// Choose a specific layout for a window
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux select-layout [-nop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn select_layout(&mut self, select_layout: Option<&SelectLayot>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_layout) = select_layout {
            if select_layout.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if select_layout.last.unwrap_or(false) {
                args.push(o_KEY);
            }
            if select_layout.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if let Some(s) = select_layout.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = select_layout.layout_name {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }
}
