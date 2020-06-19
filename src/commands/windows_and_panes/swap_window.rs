use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SWAP_WINDOW: &'static str = "swap-window";

    /// This is similar to link-window, except the source and destination windows are swapped
    ///
    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux swap-window [-d] [-s src-window] [-t dst-window]
    /// (alias: swapw)
    /// ```
    pub fn swap_window(
        &mut self,
        detached: Option<bool>,
        src_window: Option<&'a str>,
        dst_window: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if detached.unwrap_or(false) {
            args.push(d_KEY);
        }
        if let Some(src_window) = src_window {
            args.extend_from_slice(&[s_KEY, &src_window])
        }
        if let Some(dst_window) = dst_window {
            args.extend_from_slice(&[t_KEY, &dst_window])
        }
        let output = self.subcommand(TmuxInterface::SWAP_WINDOW, &args)?;
        Ok(output)
    }
}
