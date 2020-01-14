use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const SOURCE_FILE: &'static str = "source-file";

    /// Execute commands from path
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux source-file [-nqv] path
    /// (alias: source)
    /// ```
    pub fn source_file(
        &mut self,
        not_execute: Option<bool>,
        quite: Option<bool>,
        show_parsed: Option<bool>,
        path: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if not_execute.unwrap_or(false) {
            args.push(n_KEY);
        }
        if quite.unwrap_or(false) {
            args.push(q_KEY);
        }
        if show_parsed.unwrap_or(false) {
            args.push(v_KEY);
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SOURCE_FILE, &args)?;
        Ok(output)
    }
}
