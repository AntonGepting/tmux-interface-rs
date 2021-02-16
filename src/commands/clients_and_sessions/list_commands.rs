use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const LIST_COMMANDS: &'static str = "list-commands";
    #[cfg(feature = "use_cmd_alias")]
    const LIST_COMMANDS: &'static str = "lscm";

    /// List the syntax of all commands supported by tmux
    ///
    /// # Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// tmux list-commands [-F format]
    /// (alias: lscm)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-commands
    /// (alias: lscm)
    /// ```
    pub fn list_commands(&mut self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        let output = self.command(TmuxInterface::LIST_COMMANDS, &args)?;
        Ok(output)
    }
}
