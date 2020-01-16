use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// tmux X.X:
/// ```
/// (removed)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux set-window-option [-aFgoqu] [-t target-window] option value
/// (alias: setw)
/// ```
#[derive(Default, Debug)]
pub struct SetWindowOption<'a> {
    /// [-a] -
    pub append: Option<bool>,
    /// [-F] -
    pub format: Option<bool>,
    /// [-g] -
    pub global: Option<bool>,
    /// [-o] -
    pub not_overwrite: Option<bool>,
    /// [-q] -
    pub quiet: Option<bool>,
    /// [-u] -
    pub unset: Option<bool>,
    /// [-t target-window] -
    pub target_window: Option<&'a str>,
    //pub option: &'a str,                // option
    //pub value: &'a str,                 // value
}

impl<'a> SetWindowOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const SET_WINDOW_OPTION: &'static str = "set-window-option";

    /// # Manual
    ///
    /// tmux X.X:
    /// ```
    /// (removed)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux set-window-option [-aFgoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    pub fn set_window_option(
        &mut self,
        set_window_option: Option<&SetWindowOption>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_window_option) = set_window_option {
            if set_window_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            if set_window_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            if set_window_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_window_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            if set_window_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if set_window_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if let Some(s) = set_window_option.target_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_WINDOW_OPTION, &args)?;
        Ok(output)
    }
}
