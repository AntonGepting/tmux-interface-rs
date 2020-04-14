use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
    pub fn previous_layout<T: Display>(
        &mut self,
        target_window: Option<&T>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::PREVIOUS_LAYOUT, &args)?;
        Ok(output)
    }
}
