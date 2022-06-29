use crate::commands::common::Size;
use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
#[cfg(feature = "tmux_3_3")]
use std::fmt;

/// Set the type of characters used for drawing popup borders. type may be one of:
/// ‘double’ and ‘heavy’ will fall back to standard ACS line drawing when UTF-8 is not supported.
#[cfg(feature = "tmux_3_3")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PopupBorderLinesType {
    /// single lines using ACS or UTF-8 characters (default)
    Single,
    /// variation of single with rounded corners using UTF-8 characters
    Rounded,
    /// double lines using UTF-8 characters
    Double,
    /// heavy lines using UTF-8 characters
    Heavy,
    /// simple ASCII characters
    Simple,
    /// simple ASCII space character
    Padded,
    /// no border
    NoBorder,
}

#[cfg(feature = "tmux_3_3")]
impl fmt::Display for PopupBorderLinesType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Single => "single",
            Self::Rounded => "rounded",
            Self::Double => "double",
            Self::Heavy => "heavy",
            Self::Simple => "simple",
            Self::Padded => "padded",
            Self::NoBorder => "none",
        };
        write!(f, "{}", s)
    }
}

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.3:
/// ```text
/// display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
/// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
/// [-y position] [shell-command]
/// ```
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
    /// `[-B]` - not surround the popup by a border
    #[cfg(feature = "tmux_3_3")]
    pub no_border: bool,

    /// `[-C]` - closes any popup on the client
    #[cfg(feature = "tmux_3_3")]
    pub close: bool,

    /// `[-E]` - closes the popup automatically when shell-command exits
    #[cfg(feature = "tmux_3_2")]
    pub close_on_exit: bool,

    /// `[-EE]` - closes the popup only if shell-command exited with success
    #[cfg(feature = "tmux_3_2")]
    pub close_on_success: bool,

    /// `[-b border-lines]` - sets the type of border line for the popup.  When -B is specified, the
    /// -b option is ignored.  See popup-border-lines for possible values for border-lines
    #[cfg(feature = "tmux_3_3")]
    pub border_lines: Option<PopupBorderLinesType>,

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_3_2")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-d start-directory]` -
    #[cfg(feature = "tmux_3_2")]
    pub start_directory: Option<Cow<'a, str>>,

    // XXX: struct instead of tuple
    /// `[-e environment]` - takes the form ‘VARIABLE=value’ and sets an environment variable for
    /// the popup; it may be specified multiple times
    #[cfg(feature = "tmux_3_3")]
    pub environment: Option<(Cow<'a, str>, Cow<'a, str>)>,

    /// `[-h height]` - height of the popup
    #[cfg(feature = "tmux_3_2")]
    pub height: Option<Size>,

    // TODO: not imlemented!
    /// `[-s style]` - sets the style for the popup
    #[cfg(feature = "tmux_3_3")]
    pub style: Option<Cow<'a, str>>,

    // TODO: not imlemented!
    /// `[-S border-style]` - sets the style for the popup border
    #[cfg(feature = "tmux_3_3")]
    pub border_style: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_2")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-T title]` - is a format for the popup title
    #[cfg(feature = "tmux_3_3")]
    pub title: Option<Cow<'a, str>>,

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

    /// `[-B]` - not surround the popup by a border
    #[cfg(feature = "tmux_3_3")]
    pub fn no_border(mut self) -> Self {
        self.no_border = true;
        self
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

    /// `[-b border-lines]` - sets the type of border line for the popup.  When -B is specified, the
    /// -b option is ignored.  See popup-border-lines for possible values for border-lines
    #[cfg(feature = "tmux_3_2")]
    pub fn border_lines(mut self, border_lines: PopupBorderLinesType) -> Self {
        self.border_lines = Some(border_lines);
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

    // TODO: vec
    /// `[-e environment]` - takes the form ‘VARIABLE=value’ and sets an environment variable for
    /// the popup; it may be specified multiple times
    #[cfg(feature = "tmux_3_3")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, variable: S, value: S) -> Self {
        self.environment = Some((variable.into(), value.into()));
        self
    }

    /// `[-h height]` - height of the popup
    #[cfg(feature = "tmux_3_2")]
    pub fn height(mut self, height: Size) -> Self {
        self.height = Some(height);
        self
    }

    /// `[-s style]` - sets the style for the popup
    #[cfg(feature = "tmux_3_3")]
    pub fn style<S: Into<Cow<'a, str>>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    /// `[-S border-style]` - sets the style for the popup border
    #[cfg(feature = "tmux_3_3")]
    pub fn border_style<S: Into<Cow<'a, str>>>(mut self, border_style: S) -> Self {
        self.border_style = Some(border_style.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_2")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-T title]` - is a format for the popup title
    #[cfg(feature = "tmux_3_3")]
    pub fn title<S: Into<Cow<'a, str>>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
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

        // `[-B]` - not surround the popup by a border
        #[cfg(feature = "tmux_3_3")]
        if self.no_border {
            cmd.push_flag(B_UPPERCASE_KEY);
        }

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

        // `[-b border-lines]` - sets the type of border line for the popup
        #[cfg(feature = "tmux_3_3")]
        if let Some(border_lines) = self.border_lines {
            cmd.push_option(B_LOWERCASE_KEY, border_lines.to_string());
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

        // `[-e environment]` - takes the form ‘VARIABLE=value’ and sets an environment variable for
        // the popup; it may be specified multiple times
        #[cfg(feature = "tmux_3_3")]
        if let Some(environment) = self.environment {
            cmd.push_option(
                E_LOWERCASE_KEY,
                format!("{}={}", environment.0, environment.1),
            );
        }

        // `[-h height]` - height of the popup
        #[cfg(feature = "tmux_3_2")]
        if let Some(height) = self.height {
            cmd.push_option(H_LOWERCASE_KEY, height.to_string());
        }

        // `[-s style]` - sets the style for the popup
        #[cfg(feature = "tmux_3_3")]
        if let Some(style) = self.style {
            cmd.push_option(S_LOWERCASE_KEY, style);
        }

        // `[-S border-style]` - sets the style for the popup border
        #[cfg(feature = "tmux_3_3")]
        if let Some(border_style) = self.border_style {
            cmd.push_option(S_UPPERCASE_KEY, border_style);
        }

        // `[-t target-pane]` - target-client
        #[cfg(feature = "tmux_3_2")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-T title]` - is a format for the popup title
        #[cfg(feature = "tmux_3_3")]
        if let Some(title) = self.title {
            cmd.push_option(T_UPPERCASE_KEY, title);
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
