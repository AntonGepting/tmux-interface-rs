use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
    pub fn next_layout<T: Display>(&mut self, target_window: Option<&T>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::NEXT_LAYOUT, &args)?;
        Ok(output)
    }
}
