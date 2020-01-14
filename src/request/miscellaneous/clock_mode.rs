use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Miscellaneous" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS)
impl<'a> TmuxInterface<'a> {
    const CLOCK_MODE: &'static str = "clock-mode";

    /// # Manual
    ///
    /// ```text
    /// tmux clock-mode [-t target-pane]
    /// ```
    pub fn clock_mode(&mut self, target_pane: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::CLOCK_MODE, &args)?;
        Ok(output)
    }
}
