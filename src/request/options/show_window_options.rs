use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_WINDOW_OPTIONS: &'static str = "show-window-options";

    /// # Manual
    ///
    /// ```text
    /// tmux show-window-options [-gv] [-t target-window] [option]
    /// (alias: showw)
    /// ```
    pub fn show_window_options<T: Display>(
        &mut self,
        global: Option<bool>,
        only_value: Option<bool>,
        target_window: Option<&T>,
        option: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if only_value.unwrap_or(false) {
            args.push(v_KEY);
        }
        if let Some(target_window) = target_window {
            s = target_window.to_string();
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = option {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::SHOW_WINDOW_OPTIONS, &args)?;
        Ok(output)
    }
}
