use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LAST_PANE: &'static str = "last-pane";

    /// Select the last (previously selected) pane
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux last-pane [-deZ] [-t target-window]
    /// (alias: lastp)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux last-pane [-de] [-t target-window]
    /// (alias: lastp)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn last_pane<T: Display>(
        &mut self,
        disable: Option<bool>,
        enable: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if disable.unwrap_or(false) {
            args.push(d_KEY);
        }
        if enable.unwrap_or(false) {
            args.push(e_KEY);
        }
        if keep_zoomed.unwrap_or(false) {
            args.push(Z_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LAST_PANE, &args)?;
        Ok(output)
    }

    /// Select the last (previously selected) pane
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux last-pane [-deZ] [-t target-window]
    /// (alias: lastp)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux last-pane [-de] [-t target-window]
    /// (alias: lastp)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn last_pane<T: Display>(
        &mut self,
        disable: Option<bool>,
        enable: Option<bool>,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if disable.unwrap_or(false) {
            args.push(d_KEY);
        }
        if enable.unwrap_or(false) {
            args.push(e_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LAST_PANE, &args)?;
        Ok(output)
    }
}
