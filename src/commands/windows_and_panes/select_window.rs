use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Select the window at target-window.
///
/// # Manual
///
/// tmux ^1.8:
/// ```text
/// tmux select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-window [-lnp] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux select-window [-t target-window]
/// (alias: selectw)
/// ```
#[derive(Default, Debug)]
pub struct SelectWindow<'a> {
    /// [-l] - equivalent to last-window
    #[cfg(feature = "tmux_1_5")]
    pub last: Option<bool>,
    /// [-n] - equivalent to next-window
    #[cfg(feature = "tmux_1_5")]
    pub next: Option<bool>,
    /// [-p] - equivalent to previous-window
    #[cfg(feature = "tmux_1_5")]
    pub previous: Option<bool>,
    /// [-T] - if the selected window is already the current window, behave like last-window
    #[cfg(feature = "tmux_1_8")]
    pub switch: Option<bool>,
    /// [-t target-window] - target-window
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a str>,
}

impl<'a> SelectWindow<'a> {
    pub fn new() -> SelectWindow<'a> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SelectWindowBuilder<'a> {
    #[cfg(feature = "tmux_1_5")]
    pub last: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub next: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub previous: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub switch: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a str>,
}

impl<'a> SelectWindowBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn switch(&mut self) -> &mut Self {
        self.switch = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_window(&mut self, target_window: &'a str) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn build(&self) -> SelectWindow<'a> {
        SelectWindow {
            #[cfg(feature = "tmux_1_5")]
            last: self.last,
            #[cfg(feature = "tmux_1_5")]
            next: self.next,
            #[cfg(feature = "tmux_1_5")]
            previous: self.previous,
            #[cfg(feature = "tmux_1_8")]
            switch: self.switch,
            #[cfg(feature = "tmux_0_8")]
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
    /// tmux ^1.8:
    /// ```text
    /// tmux select-window [-lnpT] [-t target-window]
    /// (alias: selectw)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux select-window [-lnp] [-t target-window]
    /// (alias: selectw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux select-window [-t target-window]
    /// (alias: selectw)
    /// ```
    pub fn select_window(
        &mut self,
        select_window: Option<&SelectWindow>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_window) = select_window {
            #[cfg(feature = "tmux_1_5")]
            if select_window.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if select_window.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if select_window.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if select_window.switch.unwrap_or(false) {
                args.push(T_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(target_window) = select_window.target_window {
                args.extend_from_slice(&[t_KEY, &target_window])
            }
        }
        let output = self.command(TmuxInterface::SELECT_WINDOW, &args)?;
        Ok(output)
    }
}
