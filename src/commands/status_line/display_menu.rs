use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux display-menu [-O] [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
///
/// tmux ^3.0:
/// ```text
/// tmux display-menu [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg(feature = "tmux_3_0")]
pub struct DisplayMenu<'a> {
    /// `[-O]` - the menu does not close when the mouse button is released without an item selected
    #[cfg(feature = "tmux_3_2")]
    pub not_close: bool,

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_3_0")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-T title]` - title
    #[cfg(feature = "tmux_3_0")]
    pub title: Option<Cow<'a, str>>,

    /// `[-x position]` - x position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub x: Option<usize>,

    /// `[-y position]` - y position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub y: Option<usize>,

    /// `name`
    pub name: Option<Cow<'a, str>>,

    /// `key`
    pub key: Option<Cow<'a, str>>,

    /// `command`
    pub command: Option<Cow<'a, str>>,
}

impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-O]` - the menu does not close when the mouse button is released without an item selected
    #[cfg(feature = "tmux_3_2")]
    pub fn not_close(mut self) -> Self {
        self.not_close = true;
        self
    }

    /// `[-c target-client]` - target-client
    #[cfg(feature = "tmux_3_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(mut self, target_client: S) -> Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-T title]` - title
    #[cfg(feature = "tmux_3_0")]
    pub fn title<S: Into<Cow<'a, str>>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// `[-x position]` - x position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub fn x(mut self, x: usize) -> Self {
        self.x = Some(x);
        self
    }

    /// `[-y position]` - y position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub fn y(mut self, y: usize) -> Self {
        self.y = Some(y);
        self
    }

    /// `name`
    pub fn name<S: Into<Cow<'a, str>>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// `key`
    pub fn key<S: Into<Cow<'a, str>>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(DISPLAY_MENU);

        // `[-O]` - the menu does not close when the mouse button is released without an item selected
        #[cfg(feature = "tmux_3_2")]
        if self.not_close {
            cmd.push_flag(O_UPPERCASE_KEY);
        }

        // `[-c target-client]` - target-client
        #[cfg(feature = "tmux_3_0")]
        if let Some(target_client) = self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_3_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-T title]` - title
        #[cfg(feature = "tmux_3_0")]
        if let Some(title) = self.title {
            cmd.push_option(T_UPPERCASE_KEY, title);
        }

        // `[-x position]` - x position of the menu
        #[cfg(feature = "tmux_3_0")]
        if let Some(x) = self.x {
            cmd.push_option(X_LOWERCASE_KEY, x.to_string());
        }

        // `[-y position]` - y position of the menu
        #[cfg(feature = "tmux_3_0")]
        if let Some(y) = self.y {
            cmd.push_option(Y_LOWERCASE_KEY, y.to_string());
        }

        // `name`
        if let Some(name) = self.name {
            cmd.push_param(name);
        }

        // `key`
        if let Some(key) = self.key {
            cmd.push_param(key);
        }

        // `command`
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
