use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_COMMANDS: &'static str = "list-commands";

    /// List the syntax of all commands supported by tmux
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-commands [-F format]
    /// (alias: lscm)
    /// ```
    pub fn list_commands(&mut self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_COMMANDS, &args)?;
        Ok(output)
    }
}
