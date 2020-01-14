use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const COPY_MODE: &'static str = "copy-mode";

    /// Enter copy mode
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux copy-mode [-Meu] [-t target-pane]
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
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::COPY_MODE, &args)?;
        Ok(output)
    }
}
