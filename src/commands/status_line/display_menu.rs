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
    /// [-c target-client] - target-client
    #[cfg(feature = "tmux_3_0")]
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<&'a str>,
    /// [-T title] - title
    #[cfg(feature = "tmux_3_0")]
    pub title: Option<&'a str>,
    /// [-x position] - x position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub x: Option<usize>,
    /// [-y position] - y position of the menu
    #[cfg(feature = "tmux_3_0")]
    pub y: Option<usize>,
    // name - name
    //pub name: &'a str,
    // key - key
    //pub key: &'a str,
    // command ... - command
    //pub command: &'a str,
}

#[cfg(feature = "tmux_3_0")]
impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
#[cfg(feature = "tmux_3_0")]
pub struct DisplayMenuBuilder<'a> {
    #[cfg(feature = "tmux_3_0")]
    pub target_client: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub title: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub x: Option<usize>,
    #[cfg(feature = "tmux_3_0")]
    pub y: Option<usize>,
    //pub name: &'a str,
    //pub key: &'a str,
    //pub command: &'a str,
}

#[cfg(feature = "tmux_3_0")]
impl<'a> DisplayMenuBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn x(&mut self, x: usize) -> &mut Self {
        self.x = Some(x);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn y(&mut self, y: usize) -> &mut Self {
        self.y = Some(y);
        self
    }

    pub fn build(&self) -> DisplayMenu<'a> {
        DisplayMenu {
            #[cfg(feature = "tmux_3_0")]
            target_client: self.target_client,
            #[cfg(feature = "tmux_3_0")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_3_0")]
            title: self.title,
            #[cfg(feature = "tmux_3_0")]
            x: self.x,
            #[cfg(feature = "tmux_3_0")]
            y: self.y,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const DISPLAY_MENU: &'static str = "display-menu";

    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux display-menu [-c target-client] [-t target-pane] [-T title]
    /// [-x position] [-y position] name key command ...
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn display_menu(
        &mut self,
        display_menu: Option<&DisplayMenu>,
        name: &str,
        key: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(display_menu) = display_menu {
            #[cfg(feature = "tmux_3_0")]
            if let Some(target_client) = display_menu.target_client {
                args.extend_from_slice(&[c_KEY, &target_client])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(target_pane) = display_menu.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(title) = display_menu.title {
                args.extend_from_slice(&[T_KEY, &title])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(position) = display_menu.x {
                x = position.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(position) = display_menu.y {
                y = position.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
        }
        args.push(&name);
        args.push(&key);
        args.push(&command);
        let output = self.command(TmuxInterface::DISPLAY_MENU, &args)?;
        Ok(output)
    }
}
