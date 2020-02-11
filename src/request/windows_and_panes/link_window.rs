use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
pub struct LinkWindow<'a, T: Display> {
    /// [-a] - the window is moved to the next index up
    pub add: Option<bool>,
    /// [-d] - the newly linked window is not selected
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    pub src_window: Option<&'a T>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> LinkWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct LinkWindowBuilder<'a, T: Display> {
    pub add: Option<bool>,
    pub detached: Option<bool>,
    pub kill: Option<bool>,
    pub src_window: Option<&'a T>,
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> LinkWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    pub fn src_window(&mut self, src_window: &'a T) -> &mut Self {
        self.src_window = Some(src_window);
        self
    }

    pub fn dst_window(&mut self, dst_window: &'a T) -> &mut Self {
        self.dst_window = Some(dst_window);
        self
    }

    pub fn build(&self) -> LinkWindow<'a, T> {
        LinkWindow {
            add: self.add,
            detached: self.detached,
            kill: self.kill,
            src_window: self.src_window,
            dst_window: self.dst_window,
        }
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
    pub fn link_window<T: Display>(
        &mut self,
        link_window: Option<&LinkWindow<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
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
            if let Some(src_window) = link_window.src_window {
                s = src_window.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_window) = link_window.dst_window {
                t = dst_window.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::LINK_WINDOW, &args)?;
        Ok(output)
    }
}
