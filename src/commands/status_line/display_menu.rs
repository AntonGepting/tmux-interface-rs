use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
use std::fmt;

//C        Both    The centre of the terminal
//R        -x      The right side of the terminal
//P        Both    The bottom left of the pane
//M        Both    The mouse position
//W        Both    The window position on the status line
//S        -y      The line above or below the status line

pub enum PositionX {
    Num(usize),
    Percent(usize),
    TerminalCenter,
    Right,
    PaneBottom,
    MousePosition,
    StatusLineWindowPosition,
    AboveBelowStatusLine,
}

impl fmt::Display for PositionX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::Num(n) => format!("{}", n),
            Self::Percent(p) => format!("{}%", p),
            Self::TerminalCenter => "C".to_string(),
            Self::Right => "R".to_string(),
            Self::PaneBottom => "P".to_string(),
            Self::MousePosition => "M".to_string(),
            Self::StatusLineWindowPosition => "W".to_string(),
            Self::AboveBelowStatusLine => "S".to_string(),
        };
        write!(f, "#{{{}}}", output)
    }
}

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux display-menu [-c target-client] [-t target-pane] [-T title]
/// [-x position] [-y position] name key command ...
/// ```
#[derive(Clone, Debug)]
#[cfg(feature = "tmux_3_0")]
pub struct DisplayMenu<'a>(pub TmuxCommand<'a>);

impl<'a> Default for DisplayMenu<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(DISPLAY_MENU)),
            ..Default::default()
        })
    }
}

impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_3_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, target_client);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-T title]` - title
    #[cfg(feature = "tmux_3_0")]
    pub fn title<S: Into<Cow<'a, str>>>(&mut self, title: S) -> &mut Self {
        self.0.push_option(T_UPPERCASE_KEY, title);
        self
    }

    /// `[-x position]` - x position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub fn x(&mut self, x: usize) -> &mut Self {
        self.0.push_option(X_LOWERCASE_KEY, x.to_string());
        self
    }

    /// `[-y position]` - y position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub fn y(&mut self, y: usize) -> &mut Self {
        self.0.push_option(Y_LOWERCASE_KEY, y.to_string());
        self
    }

    /// `name`
    pub fn name<S: Into<Cow<'a, str>>>(&mut self, name: S) -> &mut Self {
        self.0.push_param(name);
        self
    }

    /// `key`
    pub fn key<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
        self.0.push_param(key);
        self
    }

    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for DisplayMenu<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(DISPLAY_MENU)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for DisplayMenu<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(DISPLAY_MENU)),
            ..Default::default()
        })
    }
}
