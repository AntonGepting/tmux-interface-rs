use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const SAVE_BUFFER: &'static str = "save-buffer";
    #[cfg(feature = "use_cmd_alias")]
    const SAVE_BUFFER: &'static str = "saveb";

    /// Save the contents of the specified paste buffer to path.
    ///
    /// # Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-name] path
    /// (alias: saveb)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-index] path
    /// (alias: saveb)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux save-buffer [-a] [-b buffer-index] [-t target-session] path
    /// (alias: saveb)
    /// ```
    pub fn save_buffer(
        &mut self,
        append: Option<bool>,
        buffer_name: Option<&str>,
        path: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if append.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = buffer_name {
            args.extend_from_slice(&[b_KEY, &s])
        }
        args.push(path);
        let output = self.command(TmuxInterface::SAVE_BUFFER, &args)?;
        Ok(output)
    }
}
