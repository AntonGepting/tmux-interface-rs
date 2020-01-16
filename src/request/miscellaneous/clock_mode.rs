use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

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
