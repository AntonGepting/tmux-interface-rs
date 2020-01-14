use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// ```text
/// tmux find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Default, Debug)]
pub struct FindWindow<'a> {
    /// [-r] - regular expression
    pub regex: Option<bool>,
    /// [-C] - match only visible window contents
    pub only_visible: Option<bool>,
    /// [-N] - match only the window name
    pub only_name: Option<bool>,
    /// [-T] - match only the window title
    pub only_title: Option<bool>,
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    // match-string
    //pub match_string: &'a str,
}

impl<'a> FindWindow<'a> {
    pub fn new() -> FindWindow<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const FIND_WINDOW: &'static str = "find-window";

    /// Search for the fnmatch(3) pattern `match-string` in window names,
    /// titles, and visible content (but not history)
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux find-window [-rCNTZ] [-t target-pane] match-string
    /// (alias: findw)
    /// ```
    pub fn find_window(
        &mut self,
        find_window: Option<&FindWindow>,
        match_string: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(find_window) = find_window {
            if find_window.regex.unwrap_or(false) {
                args.push(r_KEY);
            }
            if find_window.only_visible.unwrap_or(false) {
                args.push(C_KEY);
            }
            if find_window.only_name.unwrap_or(false) {
                args.push(N_KEY);
            }
            if find_window.only_title.unwrap_or(false) {
                args.push(T_KEY);
            }
            if find_window.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(s) = find_window.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(match_string);
        let output = self.subcommand(TmuxInterface::FIND_WINDOW, &args)?;
        Ok(output)
    }
}
