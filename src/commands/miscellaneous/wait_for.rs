use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const WAIT_FOR: &'static str = "wait-for";
    #[cfg(feature = "use_cmd_alias")]
    const WAIT_FOR: &'static str = "wait";

    // TODO: enum for arg
    /// # Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// tmux wait-for [-L | -S | -U] channel
    /// (alias: wait)
    /// ```
    ///
    /// tmux ^1.8:
    /// ```text
    /// tmux wait-for -LSU channel
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
        let output = self.command(TmuxInterface::WAIT_FOR, &args)?;
        Ok(output)
    }
}
