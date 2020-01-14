use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Key Bindings" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS)
impl<'a> TmuxInterface<'a> {
    const SEND_PREFIX: &'static str = "send-prefix";

    /// # Manual
    ///
    /// ```text
    /// tmux send-prefix [-2] [-t target-pane]
    /// ```
    pub fn send_prefix(
        &mut self,
        secondary: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if secondary.unwrap_or(false) {
            args.push(_2_KEY);
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SEND_PREFIX, &args)?;
        Ok(output)
    }
}
