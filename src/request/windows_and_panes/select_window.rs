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

#[derive(Default, Debug)]
pub struct SelectWindowBuilder<'a, T: Display> {
    pub last: Option<bool>,
    pub next: Option<bool>,
    pub previous: Option<bool>,
    pub switch: Option<bool>,
    pub target_window: Option<&'a T>,
}

impl<'a, T: Display + Default> SelectWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn switch(&mut self) -> &mut Self {
        self.switch = Some(true);
        self
    }

    pub fn target_window(&mut self, target_window: &'a T) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn build(&self) -> SelectWindow<'a, T> {
        SelectWindow {
            last: self.last,
            next: self.next,
            previous: self.previous,
            switch: self.switch,
            target_window: self.target_window,
        }
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
