use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LAST_WINDOW: &'static str = "last-window";
    #[cfg(feature = "use_cmd_alias")]
    const LAST_WINDOW: &'static str = "last";

    /// Select the last (previously selected) window
    ///
    /// # Manual
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux last-window [-t target-session]
    /// (alias: last)
    /// ```
    pub fn last_window(&mut self, target_session: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.command(TmuxInterface::LAST_WINDOW, &args)?;
        Ok(output)
    }
}
