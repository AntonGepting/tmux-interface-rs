use crate::error::Error;
use crate::tmux_interface::*;

/// Structure for showing options
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
/// (alias: show)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct ShowOptions<'a> {
    /// [-A] - includes options inherited from a parent set of options
    pub include_inherited: Option<bool>,
    /// [-g] - global session or window options are listed
    pub global_options: Option<bool>,
    /// [-H] - includes hooks (omitted by default)
    pub hooks: Option<bool>,
    /// [-p] - show window options
    pub pane: Option<bool>,
    /// [-q] - no error will be returned if `option` is unset
    pub quiet: Option<bool>,
    /// [-s] - show the server options
    pub server: Option<bool>,
    /// [-v] - shows only the option value
    pub option_value: Option<bool>,
    /// [-w] - show the window options
    pub window: Option<bool>,
    /// [-t target-pane] - target session or window name
    pub target: Option<&'a str>,
    /// [option] - specify option name
    pub option: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct ShowOptions<'a> {
    /// [-g] - global session or window options are listed
    pub global_options: Option<bool>,
    /// [-q] - no error will be returned if `option` is unset
    pub quiet: Option<bool>,
    /// [-s] - show the server options
    pub server: Option<bool>,
    /// [-v] - shows only the option value
    pub option_value: Option<bool>,
    /// [-w] - show the window options
    pub window: Option<bool>,
    /// [-t target-session | target-window] - target session or window name
    pub target: Option<&'a str>,
    /// [option] - specify option name
    pub option: Option<&'a str>,
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Options" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS)
impl<'a> TmuxInterface<'a> {
    const SHOW_OPTIONS: &'static str = "show-options";

    // XXX: better result type?
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
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
    /// tmux X.X:
    /// ```text
    /// tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    /// (alias: show)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux show-options [-gqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn show_options(&mut self, show_options: Option<&ShowOptions>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(show_options) = show_options {
            if show_options.global_options.unwrap_or(false) {
                args.push(g_KEY);
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
}
