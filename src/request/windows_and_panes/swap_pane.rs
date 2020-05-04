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
#[derive(Default, Debug)]
pub struct SwapPane<'a, T: Display> {
    /// [-d] - instruct tmux not to change the active pane
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-D] - swap with the next pane
    #[cfg(feature = "tmux_0_8")]
    pub previous_pane: Option<bool>,
    /// [-U] - swap with the previous pane
    #[cfg(feature = "tmux_0_8")]
    pub next_pane: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    /// [-s src-pane] - src-pane
    #[cfg(feature = "tmux_1_0")]
    pub src_pane: Option<&'a T>,
    /// [-t dst-pane] - dst-pane
    #[cfg(feature = "tmux_1_0")]
    pub dst_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> SwapPane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SwapPaneBuilder<'a, T> {
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub previous_pane: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub next_pane: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub src_pane: Option<&'a T>,
    #[cfg(feature = "tmux_1_0")]
    pub dst_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> SwapPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn previous_pane(&mut self) -> &mut Self {
        self.previous_pane = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn next_pane(&mut self) -> &mut Self {
        self.next_pane = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn src_pane(&mut self, src_pane: &'a T) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn dst_pane(&mut self, dst_pane: &'a T) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> SwapPane<'a, T> {
        SwapPane {
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_0_8")]
            previous_pane: self.previous_pane,
            #[cfg(feature = "tmux_0_8")]
            next_pane: self.next_pane,
            #[cfg(feature = "tmux_3_1")]
            keep_zoomed: self.keep_zoomed,
            #[cfg(feature = "tmux_1_0")]
            src_pane: self.src_pane,
            #[cfg(feature = "tmux_1_0")]
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
    pub fn swap_pane<T: Display>(
        &mut self,
        swap_pane: Option<&SwapPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let t;
        let s;
        if let Some(swap_pane) = swap_pane {
            #[cfg(feature = "tmux_0_8")]
            {
                if swap_pane.detached.unwrap_or(false) {
                    args.push(d_KEY);
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if swap_pane.previous_pane.unwrap_or(false) {
                    args.push(D_KEY);
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if swap_pane.next_pane.unwrap_or(false) {
                    args.push(U_KEY);
                }
            }
            #[cfg(feature = "tmux_3_1")]
            {
                if swap_pane.keep_zoomed.unwrap_or(false) {
                    args.push(Z_KEY);
                }
            }
            #[cfg(feature = "tmux_1_0")]
            {
                if let Some(src_pane) = swap_pane.src_pane {
                    s = src_pane.to_string();
                    args.extend_from_slice(&[s_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_1_0")]
            {
                if let Some(dst_pane) = swap_pane.dst_pane {
                    t = dst_pane.to_string();
                    args.extend_from_slice(&[t_KEY, &t])
                }
            }
        }
        let output = self.subcommand(TmuxInterface::SWAP_PANE, &args)?;
        Ok(output)
    }
}
