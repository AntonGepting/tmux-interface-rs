use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// ```text
/// tmux move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[derive(Default, Debug)]
pub struct MoveWindow<'a> {
    /// [-a] - the window is moved to the next index up
    pub add: Option<bool>,
    /// [-r] - all windows in the session are renumbered in sequential order
    pub renumber: Option<bool>,
    /// [-d] - the newly linked window is not selected
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    pub src_window: Option<&'a str>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a str>,
}

impl<'a> MoveWindow<'a> {
    pub fn new() -> MoveWindow<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const MOVE_WINDOW: &'static str = "move-window";

    /// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux move-window [-ardk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    pub fn move_window(&mut self, move_window: Option<&MoveWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(move_window) = move_window {
            if move_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if move_window.renumber.unwrap_or(false) {
                args.push(r_KEY);
            }
            if move_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if move_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = move_window.src_window {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = move_window.dst_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_WINDOW, &args)?;
        Ok(output)
    }
}
