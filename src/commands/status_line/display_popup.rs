use crate::commands::common::Size;
use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
/// [-w width] [-x position] [-y position] [shell-command]
/// (alias: popup)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg(feature = "tmux_3_2")]
pub struct DisplayPopup<'a> {
    /// `[-C]` - closes any popup on the client
    #[cfg(feature = "tmux_3_2")]
    pub close: bool,

    /// `[-E]` - closes the popup automatically when shell-command exits
    #[cfg(feature = "tmux_3_2")]
    pub close_on_exit: bool,

    /// `[-EE]` - closes the popup only if shell-command exited with success
    #[cfg(feature = "tmux_3_2")]
    pub close_on_success: bool,

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_3_2")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-d start-directory]` -
    #[cfg(feature = "tmux_3_2")]
    pub start_directory: Option<Cow<'a, str>>,

    /// `[-h height]` - height of the popup
    #[cfg(feature = "tmux_3_2")]
    pub height: Option<Size>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_2")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-w width]` - width of the popup
    #[cfg(feature = "tmux_3_2")]
    pub width: Option<Size>,

    /// `[-x position]` - x position of the popup
    #[cfg(feature = "tmux_3_2")]
    pub x: Option<Size>,

    /// `[-y position]` - y position of the popup
    #[cfg(feature = "tmux_3_2")]
    pub y: Option<Size>,

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_3_2")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> DisplayPopup<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-C]` - closes any popup on the client
    #[cfg(feature = "tmux_3_2")]
    pub fn close(mut self) -> Self {
        self.close = true;
        self
    }

    /// `[-E]` - closes the popup automatically when shell-command exits
    #[cfg(feature = "tmux_3_2")]
    pub fn close_on_exit(mut self) -> Self {
        self.close_on_exit = true;
        self
    }

    /// `[-EE]` - closes the popup only if shell-command exited with success
    #[cfg(feature = "tmux_3_2")]
    pub fn close_on_success(mut self) -> Self {
        self.close_on_success = true;
        self
    }

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_3_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-d start-directory]` - start-directory
    #[cfg(feature = "tmux_3_2")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-h height]` - height of the popup
    #[cfg(feature = "tmux_3_2")]
    pub fn height(mut self, height: Size) -> Self {
        self.height = Some(height);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-w width]` - width of the pane
    #[cfg(feature = "tmux_3_2")]
    pub fn width(mut self, width: Size) -> Self {
        self.width = Some(width);
        self
    }

    /// `[-x position]` - x position of the pane
    #[cfg(feature = "tmux_3_2")]
    pub fn x(mut self, x: Size) -> Self {
        self.x = Some(x);
        self
    }

    /// `[-y position]` - y position of the pane
    #[cfg(feature = "tmux_3_2")]
    pub fn y(mut self, y: Size) -> Self {
        self.y = Some(y);
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_3_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(DISPLAY_POPUP);

        // `[-C]` - closes any popup on the client
        #[cfg(feature = "tmux_3_2")]
        if self.close {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-E]` - closes the popup automatically when shell-command exits
        #[cfg(feature = "tmux_3_2")]
        if self.close_on_exit {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-EE]` - closes the popup only if shell-command exited with success
        #[cfg(feature = "tmux_3_2")]
        if self.close_on_success {
            cmd.push_flag(EE_UPPERCASE_KEY);
        }

        // `[-c target-client]` - target-client
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-d start-directory]` - start-directory
        #[cfg(feature = "tmux_3_2")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(D_LOWERCASE_KEY, start_directory);
        }

        // `[-h height]` - height of the popup
        #[cfg(feature = "tmux_3_2")]
        if let Some(height) = self.height {
            cmd.push_option(H_LOWERCASE_KEY, height.to_string());
        }

        // `[-t target-pane]` - target-client
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-w width]` - width of the popup
        #[cfg(feature = "tmux_3_2")]
        if let Some(width) = self.width {
            cmd.push_option(W_LOWERCASE_KEY, width.to_string());
        }

        // `[-x position]` - x position of the popup
        #[cfg(feature = "tmux_3_2")]
        if let Some(x) = self.x {
            cmd.push_option(X_LOWERCASE_KEY, x.to_string());
        }

        // `[-y position]` - y position of the popup
        #[cfg(feature = "tmux_3_2")]
        if let Some(y) = self.y {
            cmd.push_option(Y_LOWERCASE_KEY, y.to_string());
        }

        // `[shell-command]` - shell-command
        #[cfg(feature = "tmux_3_2")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
