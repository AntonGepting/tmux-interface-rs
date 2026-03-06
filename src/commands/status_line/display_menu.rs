// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Menu<'a> = DisplayMenu<'a>;

/// Display a menu on target-client
///
/// # Manual
///
/// tmux >=3.5:
/// ```text
/// display-menu [-OM] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
///
/// tmux >=3.4:
/// ```text
/// display-menu [-O] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command …
/// alias: menu
/// ```
///
/// tmux >=3.2:
/// ```text
/// display-menu [-O] [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
///
/// tmux >=3.0:
/// ```text
/// display-menu [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DisplayMenu<'a> {
    /// `[-O]`
    #[cfg(feature = "tmux_3_2")]
    pub not_close: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_3_5")]
    pub mouse_in_menu: bool,

    /// `[-c target-client]`
    #[cfg(feature = "tmux_3_0")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-T title]`
    #[cfg(feature = "tmux_3_0")]
    pub title: Option<Cow<'a, str>>,

    /// `[-x position]`
    #[cfg(feature = "tmux_3_0")]
    pub x: Option<usize>,

    /// `[-y position]`
    #[cfg(feature = "tmux_3_0")]
    pub y: Option<usize>,

    /// `[-H selected-style]`
    #[cfg(feature = "tmux_3_4")]
    pub selected_style: Option<Cow<'a, str>>,

    /// `[-s style]`
    #[cfg(feature = "tmux_3_4")]
    pub style: Option<Cow<'a, str>>,

    /// `[-S border-style]`
    #[cfg(feature = "tmux_3_4")]
    pub border_style: Option<Cow<'a, str>>,

    /// `[name]`
    #[cfg(feature = "tmux_3_0")]
    pub name: Option<Cow<'a, str>>,

    /// `[key]`
    #[cfg(feature = "tmux_3_0")]
    pub key: Option<Cow<'a, str>>,

    /// `[command]`
    #[cfg(feature = "tmux_3_0")]
    pub command: Option<Cow<'a, str>>,
}

impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-O]`
    #[cfg(feature = "tmux_3_2")]
    pub fn not_close(mut self) -> Self {
        self.not_close = true;
        self
    }

    /// `[-M]`
    #[cfg(feature = "tmux_3_5")]
    pub fn mouse_in_menu(mut self) -> Self {
        self.mouse_in_menu = true;
        self
    }

    /// `[-c target-client]`
    #[cfg(feature = "tmux_3_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-T title]`
    #[cfg(feature = "tmux_3_0")]
    pub fn title<S: Into<Cow<'a, str>>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// `[-x position]`
    #[cfg(feature = "tmux_3_0")]
    pub fn x(mut self, x: usize) -> Self {
        self.x = Some(x);
        self
    }

    /// `[-y position]`
    #[cfg(feature = "tmux_3_0")]
    pub fn y(mut self, y: usize) -> Self {
        self.y = Some(y);
        self
    }

    /// `[-H selected-style]`
    #[cfg(feature = "tmux_3_4")]
    pub fn selected_style<S: Into<Cow<'a, str>>>(mut self, selected_style: S) -> Self {
        self.selected_style = Some(selected_style.into());
        self
    }

    /// `[-s style]`
    #[cfg(feature = "tmux_3_4")]
    pub fn style<S: Into<Cow<'a, str>>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    /// `[-S border-style]`
    #[cfg(feature = "tmux_3_4")]
    pub fn border_style<S: Into<Cow<'a, str>>>(mut self, border_style: S) -> Self {
        self.border_style = Some(border_style.into());
        self
    }

    /// `[name]`
    #[cfg(feature = "tmux_3_0")]
    pub fn name<S: Into<Cow<'a, str>>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// `[key]`
    #[cfg(feature = "tmux_3_0")]
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    /// `[command]`
    #[cfg(feature = "tmux_3_0")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(DISPLAY_MENU);

        // `[-O]`
        #[cfg(feature = "tmux_3_2")]
        if self.not_close {
            cmd.push_flag(O_UPPERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_3_5")]
        if self.mouse_in_menu {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-c target-client]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-T title]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(title) = self.title {
            cmd.push_option(T_UPPERCASE_KEY, title);
        }

        // `[-x position]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(x) = self.x {
            cmd.push_option(X_LOWERCASE_KEY, x.to_string());
        }

        // `[-y position]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(y) = self.y {
            cmd.push_option(Y_LOWERCASE_KEY, y.to_string());
        }

        // `[-H selected-style]`
        #[cfg(feature = "tmux_3_4")]
        if let Some(selected_style) = self.selected_style {
            cmd.push_option(H_UPPERCASE_KEY, selected_style);
        }

        // `[-s style]`
        #[cfg(feature = "tmux_3_4")]
        if let Some(style) = self.style {
            cmd.push_option(S_LOWERCASE_KEY, style);
        }

        // `[-S border-style]`
        #[cfg(feature = "tmux_3_4")]
        if let Some(border_style) = self.border_style {
            cmd.push_option(S_UPPERCASE_KEY, border_style);
        }

        // `[name]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(name) = self.name {
            cmd.push_param(name);
        }

        // `[key]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        // `[command]`
        #[cfg(feature = "tmux_3_0")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
