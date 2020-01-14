use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// ```text
/// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
#[derive(Default, Debug)]
pub struct BreakPane<'a> {
    /// [-d] - the new window does not become the current window
    pub detached: Option<bool>,
    /// [-P] - option prints information about the new window after it has been created
    pub print: Option<bool>,
    /// [-F format] - specify format
    pub format: Option<&'a str>,
    /// [-n] - window-name
    pub window_name: Option<&'a str>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a str>,
}

impl<'a> BreakPane<'a> {
    pub fn new() -> BreakPane<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const BREAK_PANE: &'static str = "break-pane";

    /// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    pub fn break_pane(&mut self, break_pane: Option<&BreakPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(break_pane) = break_pane {
            if break_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if break_pane.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = break_pane.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = break_pane.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(s) = break_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = break_pane.dst_window {
                args.extend_from_slice(&[t_KEY, &s]);
            }
        }
        let output = self.subcommand(TmuxInterface::BREAK_PANE, &args)?;
        Ok(output)
    }
}
