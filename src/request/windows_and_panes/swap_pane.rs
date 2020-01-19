use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

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
#[derive(Default, Debug)]
pub struct SwapPane<'a> {
    /// [-d] - instruct tmux not to change the active pane
    pub detached: Option<bool>,
    /// [-D] - swap with the next pane
    pub previous: Option<bool>,
    /// [-U] - swap with the previous pane
    pub next: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    pub keep_zoomed: Option<bool>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SwapPane<'a> {
    /// [-d] - instruct tmux not to change the active pane
    pub detached: Option<bool>,
    /// [-D] - swap with the next pane
    pub previous: Option<bool>,
    /// [-U] - swap with the previous pane
    pub next: Option<bool>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
}

impl<'a> SwapPane<'a> {
    pub fn new() -> SwapPane<'a> {
        Default::default()
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
    pub fn swap_pane(&mut self, swap_pane: Option<&SwapPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if let Some(s) = swap_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = swap_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
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
    pub fn swap_pane(&mut self, swap_pane: Option<&SwapPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if let Some(s) = swap_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = swap_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SWAP_PANE, &args)?;
        Ok(output)
    }
}
