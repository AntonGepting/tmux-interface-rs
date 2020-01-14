use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Miscellaneous" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS)
impl<'a> TmuxInterface<'a> {
    const WAIT_FOR: &'static str = "wait-for";

    /// # Manual
    ///
    /// ```text
    /// tmux wait-for [-L | -S | -U] channel
    /// (alias: wait)
    /// ```
    pub fn wait_for(
        &mut self,
        lock: Option<bool>,
        prevent_exit: Option<bool>,
        unlock: Option<bool>,
        channel: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if lock.unwrap_or(false) {
            args.push(L_KEY);
        }
        if prevent_exit.unwrap_or(false) {
            args.push(S_KEY);
        }
        if unlock.unwrap_or(false) {
            args.push(U_KEY);
        }
        args.push(channel);
        let output = self.subcommand(TmuxInterface::WAIT_FOR, &args)?;
        Ok(output)
    }
}
