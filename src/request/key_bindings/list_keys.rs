use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const LIST_KEYS: &'static str = "list-keys";

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux list-keys [-N] [-T key-table]
    /// (alias: lsk)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
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
        let output = self.subcommand(TmuxInterface::LIST_KEYS, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux list-keys [-N] [-T key-table]
    /// (alias: lsk)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux list-keys [-T key-table]
    /// (alias: lsk)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn list_keys(&mut self, key_table: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = key_table {
            args.extend_from_slice(&[T_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_KEYS, &args)?;
        Ok(output)
    }
}
