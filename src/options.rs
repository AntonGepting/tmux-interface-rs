use super::tmux_interface::*;


/// Options
impl<'a> TmuxInterface<'a> {


    /// ```text
    /// set-option [-aFgoqsuw] [-t target-session | target-window] option value
    /// (alias: set)
    /// ```
    pub fn set_option() {
        unimplemented!();
    }


    /// ```text
    /// set-window-option [-aFgoqu] [-t target-window] option value
    /// (alias: setw)
    /// ```
    pub fn set_window_option() {
        unimplemented!();
    }


    /// ```text
    /// show-options [-gHqsvw] [-t target-session | target-window] [option]
    /// (alias: show)
    /// ```
    pub fn show_options() {
    }


    /// ```text
    /// show-window-options [-gv] [-t target-window] [option]
    /// (alias: showw)
    /// ```
    pub fn show_window_options() {
        unimplemented!();
    }


}
