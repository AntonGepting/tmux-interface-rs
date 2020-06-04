use crate::error::Error;
use crate::tmux_interface::*;
//use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_PANES: &'static str = "list-panes";

    // XXX: better return type
    /// List panes on the server
    ///
    /// # Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux list-panes [-as] [-F format] [-t target]
    /// (alias: lsp)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux list-panes [-as] [-t target]
    /// (alias: lsp)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-panes [-t target]
    /// (alias: lsp)
    /// ```
    pub fn list_panes(
        &mut self,
        all: Option<bool>,
        session: Option<bool>,
        format: Option<&'a str>,
        target: Option<&'a str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if session.unwrap_or(false) {
            args.push(s_KEY);
        }
        if let Some(format) = format {
            args.extend_from_slice(&[F_KEY, &format])
        }
        if let Some(target) = target {
            args.extend_from_slice(&[t_KEY, &target])
        }
        let output = self.subcommand(TmuxInterface::LIST_PANES, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }
}
