use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const UNBIND_KEY: &'static str = "unbind-key";
    #[cfg(feature = "use_cmd_alias")]
    const UNBIND_KEY: &'static str = "unbind";

    /// # Manual
    ///
    /// ```text
    /// tmux ^2.4:
    /// tmux unbind-key [-an] [-T key-table] key
    /// (alias: unbind)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux unbind-key [-acn] [-t mode-table] [-T key-table] key
    /// (alias: unbind)
    /// ```
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux unbind-key [-acn] [-t mode-table] key
    /// (alias: unbind)
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// tmux unbind-key [-acn] [-t key-table] key
    /// (alias: unbind)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux unbind-key [-cn] [-t key-table] key
    /// (alias: unbind)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux unbind-key key
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
        let output = self.command(TmuxInterface::UNBIND_KEY, &args)?;
        Ok(output)
    }
}
