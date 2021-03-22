use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux display-menu [-c target-client] [-t target-pane] [-T title]
/// [-x position] [-y position] name key command ...
/// ```
#[derive(Default, Debug)]
#[cfg(feature = "tmux_3_0")]
pub struct DisplayMenu<'a> {
    // name - name
//pub name: &'a str,
// key - key
//pub key: &'a str,
// command ... - command
//pub command: &'a str,
}

impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// [-c target-client] - target-client
    #[cfg(feature = "tmux_3_0")]
    pub fn target_client<S: Into<Cow<'a, str>>>(&mut self, target_client: S) -> &mut Self {
        self.0.push_option(c_KEY, target_client);
        self
    }

    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(t_KEY, target_pane);
        self
    }

    /// [-T title] - title
    #[cfg(feature = "tmux_3_0")]
    pub fn title<S: Into<Cow<'a, str>>>(&mut self, title: S) -> &mut Self {
        self.0.push_option(T_KEY, title);
        self
    }

    /// [-x position] - x position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub fn x(&mut self, x: usize) -> &mut Self {
        self.0.push_option(x_KEY, x.to_string());
        self
    }

    /// [-y position] - y position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub fn y(&mut self, y: usize) -> &mut Self {
        self.0.push_option(y_KEY, y.to_string());
        self
    }

    pub fn name<S: Into<Cow<'a, str>>>(&mut self, name: S) -> &mut Self {
        self.0.push_param(name);
        self
    }

    pub fn key<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
        self.0.push_param(name);
        self
    }

    pub fn name<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> TmuxOutput {
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
