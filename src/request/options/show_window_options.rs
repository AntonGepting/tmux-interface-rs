use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Options" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS)
impl<'a> TmuxInterface<'a> {
    const SHOW_WINDOW_OPTIONS: &'static str = "show-window-options";

    /// # Manual
    ///
    /// ```text
    /// tmux show-window-options [-gv] [-t target-window] [option]
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
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = option {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::SHOW_WINDOW_OPTIONS, &args)?;
        Ok(output)
    }
}
