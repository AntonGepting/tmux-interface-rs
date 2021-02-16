use crate::error::Error;
use crate::tmux_interface::*;
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
pub struct SwapPane<'a> {
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
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    #[cfg(feature = "tmux_1_0")]
    pub dst_pane: Option<&'a str>,
}

impl<'a> SwapPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SwapPaneBuilder<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub previous_pane: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub next_pane: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub src_pane: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub dst_pane: Option<&'a str>,
}

impl<'a> SwapPaneBuilder<'a> {
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
    pub fn src_pane(&mut self, src_pane: &'a str) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn dst_pane(&mut self, dst_pane: &'a str) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> SwapPane<'a> {
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
    #[cfg(not(feature = "use_cmd_alias"))]
    const SWAP_PANE: &'static str = "swap-pane";
    #[cfg(feature = "use_cmd_alias")]
    const SWAP_PANE: &'static str = "swapp";

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
    pub fn swap_pane(&mut self, swap_pane: Option<&SwapPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(swap_pane) = swap_pane {
            #[cfg(feature = "tmux_0_8")]
            if swap_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if swap_pane.previous_pane.unwrap_or(false) {
                args.push(D_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if swap_pane.next_pane.unwrap_or(false) {
                args.push(U_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if swap_pane.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(src_pane) = swap_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &src_pane])
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(dst_pane) = swap_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &dst_pane])
            }
        }
        let output = self.command(TmuxInterface::SWAP_PANE, &args)?;
        Ok(output)
    }
}
