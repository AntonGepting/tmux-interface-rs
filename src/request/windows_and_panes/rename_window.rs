use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const RENAME_WINDOW: &'static str = "rename-window";

    /// Rename the current window, or the window at target-window if specified, to new-name
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rename-window [-t target-window] new-name
    /// (alias: renamew)
    /// ```
    pub fn rename_window(
        &mut self,
        target_window: Option<&str>,
        new_name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_WINDOW, &args)?;
        Ok(output)
    }
}
