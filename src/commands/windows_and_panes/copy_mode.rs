use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const COPY_MODE: &'static str = "copy-mode";

    /// Enter copy mode
    ///
    /// # Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux copy-mode [-Meu] [-t target-pane]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux copy-mode [-u] [-t target-pane]
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux copy-mode [-u] [-t target-window]
    /// ```
    pub fn copy_mode(
        &mut self,
        mouse_drag: Option<bool>,
        bottom_exit: Option<bool>,
        page_up: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if mouse_drag.unwrap_or(false) {
            args.push(M_KEY);
        }
        if bottom_exit.unwrap_or(false) {
            args.push(e_KEY);
        }
        if page_up.unwrap_or(false) {
            args.push(u_KEY);
        }
        if let Some(target_pane) = target_pane {
            args.extend_from_slice(&[t_KEY, &target_pane])
        }
        let output = self.command(TmuxInterface::COPY_MODE, &args)?;
        Ok(output)
    }
}
