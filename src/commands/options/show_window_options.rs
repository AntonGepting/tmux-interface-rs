use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_WINDOW_OPTIONS: &'static str = "show-window-options";

    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// (removed)
    /// ```
    ///
    /// tmux ^1.8:
    /// ```text
    /// tmux show-window-options [-gv] [-t target-window] [option]
    /// (alias: showw)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux show-window-options [-g] [-t target-window] [option]
    /// (alias: showw)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux show-window-options [-g] [-t target-window]
    /// (alias: showw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux show-window-options [-t target-window] option value
    /// (alias: showw)
    /// ```
    pub fn show_window_options(
        &mut self,
        global: Option<bool>,
        only_value: Option<bool>,
        target_window: Option<&str>,
        option: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if only_value.unwrap_or(false) {
            args.push(v_KEY);
        }
        if let Some(target_window) = target_window {
            args.extend_from_slice(&[t_KEY, &target_window])
        }
        if let Some(s) = option {
            args.push(&s)
        }
        let output = self.command(TmuxInterface::SHOW_WINDOW_OPTIONS, &args)?;
        Ok(output)
    }
}
