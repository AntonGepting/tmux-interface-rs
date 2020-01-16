use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Link the window at src-window to the specified dst-window
///
/// # Manual
///
/// ```text
/// tmux link-window [-adk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
#[derive(Default, Debug)]
pub struct LinkWindow<'a> {
    /// [-a] - the window is moved to the next index up
    pub add: Option<bool>,
    /// [-d] - the newly linked window is not selected
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    pub src_window: Option<&'a str>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a str>,
}

impl<'a> LinkWindow<'a> {
    pub fn new() -> LinkWindow<'a> {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const LINK_WINDOW: &'static str = "link-window";

    /// Link the window at src-window to the specified dst-window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux link-window [-adk] [-s src-window] [-t dst-window]
    /// (alias: linkw)
    /// ```
    pub fn link_window(&mut self, link_window: Option<&LinkWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(link_window) = link_window {
            if link_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if link_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if link_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = link_window.src_window {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = link_window.dst_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::LINK_WINDOW, &args)?;
        Ok(output)
    }
}
