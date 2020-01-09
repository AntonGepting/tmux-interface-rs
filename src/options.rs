use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// # Manual
///
/// ```text
/// tmux set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
#[derive(Default, Debug)]
pub struct SetOption<'a> {
    pub append: Option<bool>,        // [-a]
    pub format: Option<bool>,        // [-F]
    pub global: Option<bool>,        // [-g]
    pub not_overwrite: Option<bool>, // [-o]
    pub pane: Option<bool>,          // [-p]
    pub quiet: Option<bool>,         // [-q]
    pub server: Option<bool>,        // [-s]
    pub unset: Option<bool>,         // [-u]
    pub window: Option<bool>,        // [-w]
    pub target: Option<&'a str>,     // [-t target-pane]
                                     //pub option: &'a str,             // option
                                     //pub value: &'a str,              // value
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Show options structure
///
/// # Manual
///
/// ```text
/// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
#[derive(Default, Debug)]
pub struct ShowOptions<'a> {
    /// includes options inherited from a parent set of options
    pub include_inherited: Option<bool>, // [-A]
    /// global session or window options are listed
    pub global_options: Option<bool>, // [-g]
    /// includes hooks (omitted by default)
    pub hooks: Option<bool>, // [-H]
    /// show window options
    pub pane: Option<bool>, // [-p]
    /// no error will be returned if `option` is unset
    pub quiet: Option<bool>, // [-q]
    /// show the server options
    pub server: Option<bool>, // [-s]
    /// shows only the option value
    pub option_value: Option<bool>, // [-v]
    /// show the window options
    pub window: Option<bool>, // [-w]
    /// target session or window name
    pub target: Option<&'a str>, // [-t target-pane]
    /// option name
    pub option: Option<&'a str>, // [option]
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Options
impl<'a> TmuxInterface<'a> {
    const SET_OPTION: &'static str = "set-option";
    const SHOW_OPTIONS: &'static str = "show-options";
    const SHOW_WINDOW_OPTIONS: &'static str = "show-window-options";

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

    // XXX: better result type?
    /// # Manual
    ///
    /// ```text
    /// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    /// (alias: show)
    /// ```
    pub fn show_options(&mut self, show_options: Option<&ShowOptions>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(show_options) = show_options {
            if show_options.include_inherited.unwrap_or(false) {
                args.push(A_KEY);
            }
            if show_options.global_options.unwrap_or(false) {
                args.push(g_KEY);
            }
            if show_options.hooks.unwrap_or(false) {
                args.push(H_KEY);
            }
            if show_options.window.unwrap_or(false) {
                args.push(p_KEY);
            }
            if show_options.quiet.unwrap_or(false) {
                args.push(q_KEY);
            }
            if show_options.server.unwrap_or(false) {
                args.push(s_KEY);
            }
            if show_options.option_value.unwrap_or(false) {
                args.push(v_KEY);
            }
            if show_options.window.unwrap_or(false) {
                args.push(w_KEY);
            }
            if let Some(s) = show_options.target {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = show_options.option {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SHOW_OPTIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }

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
