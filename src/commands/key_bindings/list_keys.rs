use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_KEYS: &'static str = "list-keys";

    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux list-keys [-1aN] [-P prefix-string -T key-table]
    /// (alias: lsk)
    /// ```
    ///
    /// tmux ^2.4:
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux list-keys [-t mode-table] [-T key-table]
    /// (alias: lsk)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux list-keys [-t key-table]
    /// (alias: lsk)
    /// ```
    pub fn list_keys(
        &mut self,
        note: Option<bool>,
        key_table: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if note.unwrap_or(false) {
            args.push(N_KEY);
        }
        if let Some(s) = key_table {
            args.extend_from_slice(&[T_KEY, &s])
        }
        let output = self.command(TmuxInterface::LIST_KEYS, &args)?;
        Ok(output)
    }
}
