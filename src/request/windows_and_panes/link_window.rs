use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Link the window at src-window to the specified dst-window
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux link-window [-adk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux link-window [-dk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
#[derive(Default, Debug)]
pub struct LinkWindow<'a, T: Display> {
    /// [-a] - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub add: Option<bool>,
    /// [-d] - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_0_8")]
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<&'a T>,
    /// [-t dst-window] - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> LinkWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct LinkWindowBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_2_1")]
    pub add: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub kill: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<&'a T>,
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> LinkWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn src_window(&mut self, src_window: &'a T) -> &mut Self {
        self.src_window = Some(src_window);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window(&mut self, dst_window: &'a T) -> &mut Self {
        self.dst_window = Some(dst_window);
        self
    }

    pub fn build(&self) -> LinkWindow<'a, T> {
        LinkWindow {
            #[cfg(feature = "tmux_2_1")]
            add: self.add,
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_0_8")]
            kill: self.kill,
            #[cfg(feature = "tmux_0_8")]
            src_window: self.src_window,
            #[cfg(feature = "tmux_0_8")]
            dst_window: self.dst_window,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const LINK_WINDOW: &'static str = "link-window";

    /// Link the window at src-window to the specified dst-window
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux link-window [-adk] [-s src-window] [-t dst-window]
    /// (alias: linkw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux link-window [-dk] [-s src-window] [-t dst-window]
    /// (alias: linkw)
    /// ```
    pub fn link_window<T: Display>(
        &mut self,
        link_window: Option<&LinkWindow<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        if let Some(link_window) = link_window {
            #[cfg(feature = "tmux_2_1")]
            if link_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if link_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if link_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(src_window) = link_window.src_window {
                s = src_window.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(dst_window) = link_window.dst_window {
                t = dst_window.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::LINK_WINDOW, &args)?;
        Ok(output)
    }
}
