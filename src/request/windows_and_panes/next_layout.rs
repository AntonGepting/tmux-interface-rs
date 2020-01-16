use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const NEXT_LAYOUT: &'static str = "next-layout";

    /// Move a window to the next layout and rearrange the panes to fit
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux next-layout [-t target-window]
    /// (alias: nextl)
    /// ```
    pub fn next_layout(&mut self, target_window: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::NEXT_LAYOUT, &args)?;
        Ok(output)
    }
}
