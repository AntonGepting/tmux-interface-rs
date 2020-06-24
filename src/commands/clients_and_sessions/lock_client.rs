use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LOCK_CLIENT: &'static str = "lock-client";

    /// Lock `target-client`
    ///
    /// # Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// tmux lock-client [-t target-client]
    /// (alias: lockc)
    /// ```
    pub fn lock_client(&mut self, target_client: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.command(TmuxInterface::LOCK_CLIENT, &args)?;
        Ok(output)
    }
}
