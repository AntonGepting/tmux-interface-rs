use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const UNBIND_KEY: &'static str = "unbind-key";

    /// # Manual
    ///
    /// ```text
    /// tmux unbind-key [-an] [-T key-table] key
    /// (alias: unbind)
    /// ```
    pub fn unbind_key(
        &mut self,
        all: Option<bool>,
        root: Option<bool>,
        key_table: Option<&str>,
        key: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if root.unwrap_or(false) {
            args.push(n_KEY);
        }
        if let Some(s) = key_table {
            args.extend_from_slice(&[T_KEY, &s])
        }
        args.push(key);
        let output = self.subcommand(TmuxInterface::UNBIND_KEY, &args)?;
        Ok(output)
    }
}
