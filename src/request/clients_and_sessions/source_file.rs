use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SOURCE_FILE: &'static str = "source-file";

    /// Execute commands from path
    ///
    /// # Manual
    ///
    /// tmux ^3.0a:
    /// ```text
    /// tmux source-file [-nqv] path
    /// (alias: source)
    /// ```
    ///
    /// tmux ^2.3:
    /// ```text
    /// tmux source-file path
    /// (alias: source)
    ///
    /// ```
    /// tmux ^0.8:
    /// ```text
    /// tmux source-file path
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
        #[cfg(feature = "tmux_3_0a")]
        {
            if not_execute.unwrap_or(false) {
                args.push(n_KEY);
            }
        }
        if quite.unwrap_or(false) {
            args.push(q_KEY);
        }
        #[cfg(feature = "tmux_3_0a")]
        {
            if show_parsed.unwrap_or(false) {
                args.push(v_KEY);
            }
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SOURCE_FILE, &args)?;
        Ok(output)
    }
}
