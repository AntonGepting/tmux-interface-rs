use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const ROTATE_WINDOW: &'static str = "rotate-window";

    /// Rotate the positions of the panes within a window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rotate-window [-DUZ] [-t target-window]
    /// (alias: rotatew)
    /// ```
    pub fn rotate_window(
        &mut self,
        down: Option<bool>,
        up: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if down.unwrap_or(false) {
            args.push(D_KEY);
        }
        if up.unwrap_or(false) {
            args.push(U_KEY);
        }
        if keep_zoomed.unwrap_or(false) {
            args.push(Z_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::ROTATE_WINDOW, &args)?;
        Ok(output)
    }
}
