// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Popup<'a> = DisplayPopup<'a>;

/// Display a menu on target-client
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// display-popup [-BCEkN] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
/// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
/// [-y position] [shell-command]
/// ```
///
/// tmux >=3.3:
/// ```text
/// display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
/// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
/// [-y position] [shell-command]
/// ```
///
/// tmux >=3.2:
/// ```text
/// display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
/// [-w width] [-x position] [-y position] [shell-command]
/// (alias: popup)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DisplayPopup<'a> {
    /// `[-B]`
    #[cfg(feature = "tmux_3_3")]
    pub no_border: bool,

    /// `[-C]`
    #[cfg(feature = "tmux_3_2")]
    pub close: bool,

    /// `[-E]`
    #[cfg(feature = "tmux_3_2")]
    pub close_on_exit: bool,

    /// `[-EE]`
    #[cfg(feature = "tmux_3_2")]
    pub close_on_success: bool,

    /// `[-k]`
    #[cfg(feature = "tmux_3_6")]
    pub any_key_dismiss: bool,

    /// `[-N]`
    #[cfg(feature = "tmux_3_6")]
    pub disable_previous: bool,

    /// `[-b border-lines]`
    #[cfg(feature = "tmux_3_3")]
    pub border_lines: Option<Cow<'a, str>>,

    /// `[-c target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-d start-directory]`
    #[cfg(feature = "tmux_3_2")]
    pub start_directory: Option<Cow<'a, str>>,

    // XXX: struct instead of tuple
    /// `[-e environment]`
    #[cfg(feature = "tmux_3_3")]
    pub environment: Option<Cow<'a, str>>,

    /// `[-h height]`
    #[cfg(feature = "tmux_3_2")]
    pub height: Option<usize>,

    /// `[-s style]`
    #[cfg(feature = "tmux_3_3")]
    pub style: Option<Cow<'a, str>>,

    // TODO: not imlemented!
    /// `[-S border-style]`
    #[cfg(feature = "tmux_3_3")]
    pub border_style: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-T title]`
    #[cfg(feature = "tmux_3_3")]
    pub title: Option<Cow<'a, str>>,

    /// `[-w width]`
    #[cfg(feature = "tmux_3_2")]
    pub width: Option<usize>,

    /// `[-x position]`
    #[cfg(feature = "tmux_3_2")]
    pub x: Option<usize>,

    /// `[-y position]`
    #[cfg(feature = "tmux_3_2")]
    pub y: Option<usize>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_3_2")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> DisplayPopup<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-B]`
    #[cfg(feature = "tmux_3_3")]
    pub fn no_border(mut self) -> Self {
        self.no_border = true;
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_3_2")]
    pub fn close(mut self) -> Self {
        self.close = true;
        self
    }

    /// `[-E]`
    #[cfg(feature = "tmux_3_2")]
    pub fn close_on_exit(mut self) -> Self {
        self.close_on_exit = true;
        self
    }

    /// `[-EE]`
    #[cfg(feature = "tmux_3_2")]
    pub fn close_on_success(mut self) -> Self {
        self.close_on_success = true;
        self
    }

    /// `[-k]`
    #[cfg(feature = "tmux_3_6")]
    pub fn any_key_dismiss(mut self) -> Self {
        self.any_key_dismiss = true;
        self
    }

    /// `[-N]`
    #[cfg(feature = "tmux_3_6")]
    pub fn disable_previous(mut self) -> Self {
        self.disable_previous = true;
        self
    }

    /// `[-b border-lines]`
    #[cfg(feature = "tmux_3_3")]
    pub fn border_lines<S: Into<Cow<'a, str>>>(mut self, border_lines: S) -> Self {
        self.border_lines = Some(border_lines.into());
        self
    }

    /// `[-c target-client]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-d start-directory]`
    #[cfg(feature = "tmux_3_2")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-e environment]`
    #[cfg(feature = "tmux_3_3")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, environment: S) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// `[-h height]`
    #[cfg(feature = "tmux_3_2")]
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// `[-s style]`
    #[cfg(feature = "tmux_3_3")]
    pub fn style<S: Into<Cow<'a, str>>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    /// `[-S border-style]`
    #[cfg(feature = "tmux_3_3")]
    pub fn border_style<S: Into<Cow<'a, str>>>(mut self, border_style: S) -> Self {
        self.border_style = Some(border_style.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-T title]`
    #[cfg(feature = "tmux_3_3")]
    pub fn title<S: Into<Cow<'a, str>>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// `[-w width]`
    #[cfg(feature = "tmux_3_2")]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// `[-x position]`
    #[cfg(feature = "tmux_3_2")]
    pub fn x(mut self, x: usize) -> Self {
        self.x = Some(x);
        self
    }

    /// `[-y position]`
    #[cfg(feature = "tmux_3_2")]
    pub fn y(mut self, y: usize) -> Self {
        self.y = Some(y);
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_3_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(DISPLAY_POPUP);

        // `[-B]`
        #[cfg(feature = "tmux_3_3")]
        if self.no_border {
            cmd.push_flag(B_UPPERCASE_KEY);
        }

        // `[-C]`
        #[cfg(feature = "tmux_3_2")]
        if self.close {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-E]`
        #[cfg(feature = "tmux_3_2")]
        if self.close_on_exit {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-EE]`
        #[cfg(feature = "tmux_3_2")]
        if self.close_on_success {
            cmd.push_flag(EE_UPPERCASE_KEY);
        }

        // `[-k]`
        #[cfg(feature = "tmux_3_6")]
        if self.any_key_dismiss {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-N]`
        #[cfg(feature = "tmux_3_6")]
        if self.disable_previous {
            cmd.push_flag(N_UPPERCASE_KEY);
        }

        // `[-b border-lines]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(border_lines) = self.border_lines {
            cmd.push_option(B_LOWERCASE_KEY, border_lines);
        }

        // `[-c target-client]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-d start-directory]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(D_LOWERCASE_KEY, start_directory);
        }

        // `[-e environment]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(environment) = self.environment {
            cmd.push_option(E_LOWERCASE_KEY, environment);
        }

        // `[-h height]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(height) = self.height {
            cmd.push_option(H_LOWERCASE_KEY, height.to_string());
        }

        // `[-s style]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(style) = self.style {
            cmd.push_option(S_LOWERCASE_KEY, style);
        }

        // `[-S border-style]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(border_style) = self.border_style {
            cmd.push_option(S_UPPERCASE_KEY, border_style);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-T title]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(title) = self.title {
            cmd.push_option(T_UPPERCASE_KEY, title);
        }

        // `[-w width]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(width) = self.width {
            cmd.push_option(W_LOWERCASE_KEY, width.to_string());
        }

        // `[-x position]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(x) = self.x {
            cmd.push_option(X_LOWERCASE_KEY, x.to_string());
        }

        // `[-y position]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(y) = self.y {
            cmd.push_option(Y_LOWERCASE_KEY, y.to_string());
        }

        // `[shell-command]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
