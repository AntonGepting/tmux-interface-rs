use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;
use std::process::Output;


/// # Manual
///
/// ```text
/// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
#[derive(Default)]
pub struct SetOption<'a> {
    pub append: Option<bool>,                   // [-a]
    pub format: Option<bool>,                   // [-F]
    pub global: Option<bool>,                   // [-g]
    pub not_overwrite: Option<bool>,            // [-o]
    pub quiet: Option<bool>,                    // [-q]
    pub server: Option<bool>,                   // [-s]
    pub unset: Option<bool>,                    // [-u]
    pub window: Option<bool>,                   // [-w]
    pub target: Option<&'a str>,                // [-t target-session | target-window]
    pub option: &'a str,                        // option
    pub value: &'a str                          // value
}

impl<'a> SetOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// # Manual
///
/// ```text
/// tmux set-window-option [-aFgoqu] [-t target-window] option value
/// (alias: setw)
/// ```
#[derive(Default)]
pub struct SetWindowOption<'a> {
    pub append: Option<bool>,                   // [-a]
    pub format: Option<bool>,                   // [-F]
    pub global: Option<bool>,                   // [-g]
    pub not_overwrite: Option<bool>,            // [-o]
    pub quiet: Option<bool>,                    // [-q]
    pub unset: Option<bool>,                    // [-u]
    pub target_window: Option<&'a str>,         // [-t target-window]
    pub option: &'a str,                        // option
    pub value: &'a str                          // value
}

impl<'a> SetWindowOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Show options structure
///
/// # Manual
///
/// ```text
/// tmux show-options [-gHqsvw] [-t target-session | target-window] [option]
/// (alias: show)
/// ```
#[derive(Default)]
pub struct ShowOptions<'a> {
    /// global session or window options are listed
    pub global_options: Option<bool>,           // [-g]
    /// includes hooks (omitted by default)
    pub hooks: Option<bool>,                    // [-H]
    /// no error will be returned if `option` is unset
    pub quiet: Option<bool>,                    // [-q]
    /// show the server options
    pub server_options: Option<bool>,           // [-s]
    /// shows only the option value
    pub option_value: Option<bool>,             // [-v]
    /// show the window options
    pub window_options: Option<bool>,           // [-w]
    /// target session or window name
    pub target: Option<&'a str>,                // [-t target-session | target-window]
    /// option name
    pub option: Option<&'a str>,                // [option]
}

impl<'a> ShowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Options
impl<'a> TmuxInterface<'a> {


    const SET_OPTION: &'static str = "set-option";
    const SET_WINDOW_OPTION: &'static str = "set-window-option";
    const SHOW_OPTIONS: &'static str = "show-options";
    const SHOW_WINDOW_OPTIONS: &'static str = "show-window-options";


    /// # Manual
    ///
    /// ```text
    /// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    pub fn set_option(&self, set_option: &SetOption) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if set_option.append.unwrap_or(false) { args.push(a_KEY); }
        if set_option.format.unwrap_or(false) { args.push(F_KEY); }
        if set_option.global.unwrap_or(false) { args.push(g_KEY); }
        if set_option.not_overwrite.unwrap_or(false) { args.push(o_KEY); }
        if set_option.quiet.unwrap_or(false) { args.push(q_KEY); }
        if set_option.server.unwrap_or(false) { args.push(s_KEY); }
        if set_option.unset.unwrap_or(false) { args.push(u_KEY); }
        if set_option.window.unwrap_or(false) { args.push(w_KEY); }
        set_option.target.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(set_option.option);
        args.push(set_option.value);
        let output = self.subcommand(TmuxInterface::SET_OPTION, &args)?;
        Ok(output)
    }


    /// # Manual
    ///
    /// ```text
    /// tmux set-window-option [-aFgoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    pub fn set_window_option(&self,
                             set_window_option: &SetWindowOption
                             ) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if set_window_option.append.unwrap_or(false) { args.push(a_KEY); }
        if set_window_option.format.unwrap_or(false) { args.push(F_KEY); }
        if set_window_option.global.unwrap_or(false) { args.push(g_KEY); }
        if set_window_option.not_overwrite.unwrap_or(false) { args.push(o_KEY); }
        if set_window_option.quiet.unwrap_or(false) { args.push(q_KEY); }
        if set_window_option.unset.unwrap_or(false) { args.push(u_KEY); }
        set_window_option.target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(set_window_option.option);
        args.push(set_window_option.value);
        let output = self.subcommand(TmuxInterface::SET_WINDOW_OPTION, &args)?;
        Ok(output)
    }

    // XXX: better result type?
    /// # Manual
    ///
    /// ```text
    /// tmux show-options [-gHqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    pub fn show_options(&self, show_options: &ShowOptions) -> Result<String, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if show_options.global_options.unwrap_or(false) { args.push(g_KEY); }
        if show_options.hooks.unwrap_or(false) { args.push(H_KEY); }
        if show_options.quiet.unwrap_or(false) { args.push(q_KEY); }
        if show_options.server_options.unwrap_or(false) { args.push(s_KEY); }
        if show_options.option_value.unwrap_or(false) { args.push(v_KEY); }
        if show_options.window_options.unwrap_or(false) { args.push(w_KEY); }
        show_options.target.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        show_options.option.and_then(|s| Some(args.push(&s)));
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
    pub fn show_window_options(&self,
                               global: Option<bool>,
                               only_value: Option<bool>,
                               target_window: Option<&str>,
                               option: Option<&str>
                               ) -> Result<Output, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) { args.push(g_KEY); }
        if only_value.unwrap_or(false) { args.push(v_KEY); }
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        option.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::SHOW_WINDOW_OPTIONS, &args)?;
        Ok(output)
    }


}
