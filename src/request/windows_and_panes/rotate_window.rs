use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const ROTATE_WINDOW: &'static str = "rotate-window";

    /// Rotate the positions of the panes within a window
    ///
    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux rotate-window [-DUZ] [-t target-window]
    /// (alias: rotatew)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux rotate-window [-DU] [-t target-window]
    /// (alias: rotatew)
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn rotate_window<T: Display>(
        &mut self,
        down: Option<bool>,
        up: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if down.unwrap_or(false) {
            args.push(D_KEY);
        }
        if up.unwrap_or(false) {
            args.push(U_KEY);
        }
        if keep_zoomed.unwrap_or(false) {
            args.push(Z_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::ROTATE_WINDOW, &args)?;
        Ok(output)
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_3_1")))]
    pub fn rotate_window<T: Display>(
        &mut self,
        down: Option<bool>,
        up: Option<bool>,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if down.unwrap_or(false) {
            args.push(D_KEY);
        }
        if up.unwrap_or(false) {
            args.push(U_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::ROTATE_WINDOW, &args)?;
        Ok(output)
    }
}
