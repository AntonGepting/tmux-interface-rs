use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^2.6:
/// ```text
/// tmux find-window [-CNT] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^1.7:
/// ```text
/// tmux find-window [-CNT] [-F format] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^0.8:
/// ```text
/// tmux find-window [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Default, Debug)]
pub struct FindWindow<'a, T: Display> {
    #[cfg(feature = "tmux_3_0")]
    /// [-r] - regular expression
    pub regex: Option<bool>,
    /// [-C] - match only visible window contents
    pub only_visible: Option<bool>,
    /// [-N] - match only the window name
    pub only_name: Option<bool>,
    /// [-T] - match only the window title
    pub only_title: Option<bool>,
    #[cfg(eature = "tmux_2_9")]
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    // match-string
    //pub match_string: &'a str,
}

impl<'a, T: Display + Default> FindWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct FindWindowBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_3_0")]
    pub regex: Option<bool>,
    pub only_visible: Option<bool>,
    pub only_name: Option<bool>,
    pub only_title: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub zoom: Option<bool>,
    pub target_pane: Option<&'a T>,
    //pub match_string: &'a str,
}

impl<'a, T: Display + Default> FindWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn regex(&mut self) -> &mut Self {
        self.regex = Some(true);
        self
    }

    pub fn only_visible(&mut self) -> &mut Self {
        self.only_visible = Some(true);
        self
    }

    pub fn only_name(&mut self) -> &mut Self {
        self.only_name = Some(true);
        self
    }

    pub fn only_title(&mut self) -> &mut Self {
        self.only_title = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> FindWindow<'a, T> {
        FindWindow {
            #[cfg(feature = "tmux_3_0")]
            regex: self.regex,
            only_visible: self.only_visible,
            only_name: self.only_name,
            only_title: self.only_title,
            #[cfg(feature = "tmux_2_9")]
            zoom: self.zoom,
            target_pane: self.target_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const FIND_WINDOW: &'static str = "find-window";

    /// Search for the fnmatch(3) pattern `match-string` in window names,
    /// titles, and visible content (but not history)
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux find-window [-rCNTZ] [-t target-pane] match-string
    /// (alias: findw)
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux find-window [-CNT] [-t target-pane] match-string
    /// (alias: findw)
    /// ```
    pub fn find_window<T: Display>(
        &mut self,
        find_window: Option<&FindWindow<T>>,
        match_string: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(find_window) = find_window {
            #[cfg(feature = "tmux_3_0")]
            {
                if find_window.regex.unwrap_or(false) {
                    args.push(r_KEY);
                }
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
            #[cfg(feature = "tmux_2_9")]
            {
                if find_window.zoom.unwrap_or(false) {
                    args.push(Z_KEY);
                }
            }
            if let Some(target_pane) = find_window.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(match_string);
        let output = self.subcommand(TmuxInterface::FIND_WINDOW, &args)?;
        Ok(output)
    }
}
