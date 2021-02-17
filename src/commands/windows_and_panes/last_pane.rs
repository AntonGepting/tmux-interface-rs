use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LAST_PANE: &'static str = "last-pane";
    #[cfg(feature = "use_cmd_alias")]
    const LAST_PANE: &'static str = "lastp";

    /// Select the last (previously selected) pane
    ///
    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux last-pane [-deZ] [-t target-window]
    /// (alias: lastp)
    /// ```
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux last-pane [-de] [-t target-window]
    /// (alias: lastp)
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// tmux last-pane [-t target-window]
    /// (alias: lastp)
    /// ```
    // FIXME: versions and function parameters
    pub fn last_pane(
        &mut self,
        disable: Option<bool>,
        enable: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        #[cfg(feature = "tmux_2_0")]
        if disable.unwrap_or(false) {
            args.push(d_KEY);
        }
        #[cfg(feature = "tmux_2_0")]
        if enable.unwrap_or(false) {
            args.push(e_KEY);
        }
        #[cfg(feature = "tmux_3_1")]
        if keep_zoomed.unwrap_or(false) {
            args.push(Z_KEY);
        }
        #[cfg(feature = "tmux_1_4")]
        if let Some(target_window) = target_window {
            args.extend_from_slice(&[t_KEY, &target_window])
        }
        let output = self.command(TmuxInterface::LAST_PANE, &args)?;
        Ok(output)
    }
}
