use super::tmux_interface::*;


/// Global and session environment
impl<'a> TmuxInterface<'a> {


    /// ```text
    /// set-environment [-gru] [-t target-session] name [value]
    /// (alias: setenv)
    /// ```
    pub fn set_environment() {
        unimplemented!();
    }


    /// ```text
    /// show-environment [-gs] [-t target-session] [variable]
    /// (alias: showenv)
    /// ```
    pub fn show_environment() {
        unimplemented!();
    }


}
