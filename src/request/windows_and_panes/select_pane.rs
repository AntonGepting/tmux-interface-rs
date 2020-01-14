use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// ```text
/// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
#[derive(Default, Debug)]
pub struct SelectPane<'a> {
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
    pub target_pane: Option<&'a str>,
}

impl<'a> SelectPane<'a> {
    pub fn new() -> SelectPane<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const SELECT_PANE: &'static str = "select-pane";

    /// Make pane `target-pane` the active pane in window `target-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    pub fn select_pane(&mut self, select_pane: Option<&SelectPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
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
                args.push(U_KEY);
            }
            if let Some(s) = select_pane.title {
                args.extend_from_slice(&[T_KEY, s])
            }
            if let Some(s) = select_pane.target_pane {
                args.extend_from_slice(&[t_KEY, s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output)
    }
}
