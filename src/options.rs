use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;


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


    const SHOW_OPTIONS: &'static str = "show-options";


    /// # Manual
    ///
    /// ```text
    /// tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    pub fn set_option() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// tmux set-window-option [-aFgoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    pub fn set_window_option() {
        unimplemented!();
    }


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
        show_options.target.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        show_options.option.as_ref().and_then(|s| Some(args.push(&s)));
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
    pub fn show_window_options() {
        unimplemented!();
    }


}
