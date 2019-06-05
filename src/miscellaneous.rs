use super::tmux_interface::*;


/// Miscellaneous
impl<'a> TmuxInterface<'a> {


    /// # Manual
    ///
    /// ```text
    /// clock-mode [-t target-pane]
    /// ```
    pub fn clock_mode() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// if-shell [-bF] [-t target-pane] shell-command command [command]
    /// (alias: if)
    /// ```
    pub fn if_shell() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// lock-server
    /// (alias: lock)
    /// ```
    pub fn lock_server() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// run-shell [-b] [-t target-pane] shell-command
    /// (alias: run)
    /// ```
    pub fn run_shell() {
        unimplemented!();
    }


    /// # Manual
    ///
    /// ```text
    /// wait-for [-L | -S | -U] channel
    /// (alias: wait)
    /// ```
    pub fn wait_for() {
        unimplemented!();
    }


}
