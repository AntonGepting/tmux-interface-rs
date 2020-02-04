use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Select the window at target-window.
///
/// # Manual
///
/// ```text
/// tmux select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
#[derive(Default, Debug)]
pub struct SelectWindow<'a, T: Display> {
    /// [-l] - equivalent to last-window
    pub last: Option<bool>,
    /// [-n] - equivalent to next-window
    pub next: Option<bool>,
    /// [-p] - equivalent to previous-window
    pub previous: Option<bool>,
    /// [-T] - if the selected window is already the current window, behave like last-window
    pub switch: Option<bool>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a T>,
}

impl<'a, T: Display + Default> SelectWindow<'a, T> {
    pub fn new() -> SelectWindow<'a, T> {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const SELECT_WINDOW: &'static str = "select-window";

    /// Select the window at target-window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-window [-lnpT] [-t target-window]
    /// (alias: selectw)
    /// ```
    pub fn select_window<T: Display>(
        &mut self,
        select_window: Option<&SelectWindow<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(select_window) = select_window {
            if select_window.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            if select_window.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if select_window.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if select_window.switch.unwrap_or(false) {
                args.push(T_KEY);
            }
            if let Some(target_window) = select_window.target_window {
                s = target_window.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_WINDOW, &args)?;
        Ok(output)
    }
}
