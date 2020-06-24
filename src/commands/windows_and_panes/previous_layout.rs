use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const PREVIOUS_LAYOUT: &'static str = "previous-layout";

    /// Move to the previous layout in the session
    ///
    /// # Manual
    ///
    /// tmux ^1.3:
    /// ```text
    /// tmux previous-layout [-t target-window]
    /// (alias: prevl)
    /// ```
    pub fn previous_layout(&mut self, target_window: Option<&'a str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(target_window) = target_window {
            args.extend_from_slice(&[t_KEY, &target_window])
        }
        let output = self.command(TmuxInterface::PREVIOUS_LAYOUT, &args)?;
        Ok(output)
    }
}
