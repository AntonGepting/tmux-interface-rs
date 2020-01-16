use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SOURCE_FILE: &'static str = "source-file";

    /// Execute commands from path
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux source-file [-nqv] path
    /// (alias: source)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux source-file [-q] path
    /// (alias: source)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
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

    /// Execute commands from path
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux source-file [-nqv] path
    /// (alias: source)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux source-file [-q] path
    /// (alias: source)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn source_file(&mut self, quite: Option<bool>, path: &str) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if quite.unwrap_or(false) {
            args.push(q_KEY);
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SOURCE_FILE, &args)?;
        Ok(output)
    }
}
