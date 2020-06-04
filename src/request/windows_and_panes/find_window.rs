use crate::error::Error;
use crate::tmux_interface::*;
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
pub struct FindWindow<'a> {
    /// [-r] - regular expression
    #[cfg(feature = "tmux_3_0")]
    pub regex: Option<bool>,
    /// [-C] - match only visible window contents
    #[cfg(feature = "tmux_1_7")]
    pub only_visible: Option<bool>,
    /// [-N] - match only the window name
    #[cfg(feature = "tmux_1_7")]
    pub only_name: Option<bool>,
    /// [-T] - match only the window title
    #[cfg(feature = "tmux_1_7")]
    pub only_title: Option<bool>,
    /// [-Z] - zoom the pane
    #[cfg(feature = "tmux_3_0")]
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    //#[cfg(eature = "tmux_2_6")]
    pub target_pane: Option<&'a str>,
    // match-string
    //pub match_string: &'a str,
}

impl<'a> FindWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct FindWindowBuilder<'a> {
    #[cfg(feature = "tmux_3_0")]
    pub regex: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub only_visible: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub only_name: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub only_title: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub zoom: Option<bool>,
    //#[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<&'a str>,
    //pub match_string: &'a str,
}

impl<'a> FindWindowBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn regex(&mut self) -> &mut Self {
        self.regex = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn only_visible(&mut self) -> &mut Self {
        self.only_visible = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn only_name(&mut self) -> &mut Self {
        self.only_name = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn only_title(&mut self) -> &mut Self {
        self.only_title = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    //#[cfg(feature = "tmux_2_6")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn build(&self) -> FindWindow<'a> {
        FindWindow {
            #[cfg(feature = "tmux_3_0")]
            regex: self.regex,
            #[cfg(feature = "tmux_1_7")]
            only_visible: self.only_visible,
            #[cfg(feature = "tmux_1_7")]
            only_name: self.only_name,
            #[cfg(feature = "tmux_1_7")]
            only_title: self.only_title,
            #[cfg(feature = "tmux_3_0")]
            zoom: self.zoom,
            //#[cfg(eature = "tmux_2_6")]
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
    pub fn find_window(
        &mut self,
        find_window: Option<&FindWindow>,
        match_string: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(find_window) = find_window {
            #[cfg(feature = "tmux_3_0")]
            if find_window.regex.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if find_window.only_visible.unwrap_or(false) {
                args.push(C_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if find_window.only_name.unwrap_or(false) {
                args.push(N_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if find_window.only_title.unwrap_or(false) {
                args.push(T_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if find_window.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            //#[cfg(feature = "tmux_2_6")]
            if let Some(target_pane) = find_window.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
        }
        args.push(match_string);
        let output = self.subcommand(TmuxInterface::FIND_WINDOW, &args)?;
        Ok(output)
    }
}
