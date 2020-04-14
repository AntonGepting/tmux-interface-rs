use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Swap two panes
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
/// (alias: swapp)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SwapPane<'a, T: Display> {
    /// [-d] - instruct tmux not to change the active pane
    pub detached: Option<bool>,
    /// [-D] - swap with the next pane
    pub previous: Option<bool>,
    /// [-U] - swap with the previous pane
    pub next: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    pub keep_zoomed: Option<bool>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a T>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a T>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SwapPane<'a, T: Display> {
    /// [-d] - instruct tmux not to change the active pane
    pub detached: Option<bool>,
    /// [-D] - swap with the next pane
    pub previous: Option<bool>,
    /// [-U] - swap with the previous pane
    pub next: Option<bool>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a T>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> SwapPane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SwapPaneBuilder<'a, T> {
    pub detached: Option<bool>,
    pub previous: Option<bool>,
    pub next: Option<bool>,
    pub keep_zoomed: Option<bool>,
    pub src_pane: Option<&'a T>,
    pub dst_pane: Option<&'a T>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> SwapPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = Some(true);
        self
    }

    pub fn src_pane(&mut self, src_pane: &'a T) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    pub fn dst_pane(&mut self, dst_pane: &'a T) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> SwapPane<'a, T> {
        SwapPane {
            detached: self.detached,
            previous: self.previous,
            next: self.next,
            keep_zoomed: self.keep_zoomed,
            src_pane: self.src_pane,
            dst_pane: self.dst_pane,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SwapPaneBuilder<'a, T> {
    pub detached: Option<bool>,
    pub previous: Option<bool>,
    pub next: Option<bool>,
    pub src_pane: Option<&'a T>,
    pub dst_pane: Option<&'a T>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> SwapPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn src_pane(&mut self, src_pane: &'a T) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    pub fn dst_pane(&mut self, dst_pane: &'a T) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> SwapPane<'a, T> {
        SwapPane {
            detached: self.detached,
            previous: self.previous,
            next: self.next,
            src_pane: self.src_pane,
            dst_pane: self.dst_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SWAP_PANE: &'static str = "swap-pane";

    /// Swap two panes
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn swap_pane<T: Display>(
        &mut self,
        swap_pane: Option<&SwapPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let t;
        let s;
        if let Some(swap_pane) = swap_pane {
            if swap_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if swap_pane.detached.unwrap_or(false) {
                args.push(D_KEY);
            }
            if swap_pane.detached.unwrap_or(false) {
                args.push(U_KEY);
            }
            if swap_pane.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(src_pane) = swap_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_pane) = swap_pane.dst_pane {
                t = dst_pane.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::SWAP_PANE, &args)?;
        Ok(output)
    }

    /// Swap two panes
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn swap_pane<T: Display>(
        &mut self,
        swap_pane: Option<&SwapPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        if let Some(swap_pane) = swap_pane {
            if swap_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if swap_pane.detached.unwrap_or(false) {
                args.push(D_KEY);
            }
            if swap_pane.detached.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(src_pane) = swap_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_pane) = swap_pane.dst_pane {
                t = dst_pane.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::SWAP_PANE, &args)?;
        Ok(output)
    }
}
