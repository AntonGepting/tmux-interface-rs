use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SWAP_WINDOW: &'static str = "swap-window";

    /// This is similar to link-window, except the source and destination windows are swapped
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux swap-window [-d] [-s src-window] [-t dst-window]
    /// (alias: swapw)
    /// ```
    pub fn swap_window<T: Display>(
        &mut self,
        detached: Option<bool>,
        src_window: Option<&'a T>,
        dst_window: Option<&'a T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        if detached.unwrap_or(false) {
            args.push(d_KEY);
        }
        if let Some(src_window) = src_window {
            s = src_window.to_string();
            args.extend_from_slice(&[s_KEY, &s])
        }
        if let Some(dst_window) = dst_window {
            t = dst_window.to_string();
            args.extend_from_slice(&[t_KEY, &t])
        }
        let output = self.subcommand(TmuxInterface::SWAP_WINDOW, &args)?;
        Ok(output)
    }
}
