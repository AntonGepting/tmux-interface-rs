use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Buffers" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS)
impl<'a> TmuxInterface<'a> {
    const CLEAR_HISTORY: &'static str = "clear-history";

    /// Remove and free the history for the specified pane.
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux clear-history [-t target-pane]
    /// (alias: clearhist)
    /// ```
    pub fn clear_history(&mut self, target_pane: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::CLEAR_HISTORY, &args)?;
        Ok(output)
    }
}
