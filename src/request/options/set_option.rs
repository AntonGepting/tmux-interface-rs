use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for setting a pane/window/session/server option
///
/// # Manual
///
/// ```text
/// tmux set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
#[derive(Default, Debug)]
pub struct SetOption<'a> {
    /// [-a] - value is appended to the existing setting, if the option expects a string or a style
    pub append: Option<bool>,
    /// [-F] - expand formats in the option value
    pub format: Option<bool>,
    /// [-g] - the global session or window option is set
    pub global: Option<bool>,
    /// [-o] - prevents setting an option that is already set
    pub not_overwrite: Option<bool>,
    /// [-p] - set a pane option
    pub pane: Option<bool>,
    /// [-q] - suppress errors about unknown or ambiguous options
    pub quiet: Option<bool>,
    /// [-s] - set a server option
    pub server: Option<bool>,
    /// [-u] - unset an option, so a session inherits the option from the global options
    pub unset: Option<bool>,
    /// [-w] - set a window option
    pub window: Option<bool>,
    /// [-t target-pane] - specify the target-pane
    pub target: Option<&'a str>,
    // option
    //pub option: &'a str,
    // value
    //pub value: &'a str,
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Options" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS)
impl<'a> TmuxInterface<'a> {
    const SET_OPTION: &'static str = "set-option";

    /// # Manual
    ///
    /// ```text
    /// tmux set-option [-aFgoqsuw] [-t target-pane] option value
    /// (alias: set)
    /// ```
    pub fn set_option(
        &mut self,
        set_option: Option<&SetOption>,
        option: &str,
        value: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(set_option) = set_option {
            if set_option.append.unwrap_or(false) {
                args.push(a_KEY);
            }
            if set_option.format.unwrap_or(false) {
                args.push(F_KEY);
            }
            if set_option.global.unwrap_or(false) {
                args.push(g_KEY);
            }
            if set_option.not_overwrite.unwrap_or(false) {
                args.push(o_KEY);
            }
            if set_option.pane.unwrap_or(false) {
                args.push(p_KEY);
            }
            if set_option.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if set_option.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            if set_option.unset.unwrap_or(false) {
                args.push(u_KEY);
            }
            if set_option.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            if let Some(s) = set_option.target {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        args.push(option);
        args.push(value);
        let output = self.subcommand(TmuxInterface::SET_OPTION, &args)?;
        Ok(output)
    }
}
