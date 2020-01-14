use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const PREVIOUS_LAYOUT: &'static str = "previous-layout";

    /// Move to the previous layout in the session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux previous-layout [-t target-window]
    /// (alias: prevl)
    /// ```
    pub fn previous_layout(&mut self, target_window: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::PREVIOUS_LAYOUT, &args)?;
        Ok(output)
    }
}
