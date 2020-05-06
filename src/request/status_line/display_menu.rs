use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
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
pub struct DisplayMenu<'a, T: Display> {
    /// [-c target-client] - target-client
    #[cfg(feature = "tmux_3_0")]
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<&'a T>,
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

impl<'a, T: Display + Default> DisplayMenu<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct DisplayMenuBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_3_0")]
    pub target_client: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub target_pane: Option<&'a T>,
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

impl<'a, T: Display + Default> DisplayMenuBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
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

    pub fn build(&self) -> DisplayMenu<'a, T> {
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
    pub fn display_menu<T: Display>(
        &mut self,
        display_menu: Option<&DisplayMenu<T>>,
        name: &str,
        key: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        let s;
        if let Some(display_menu) = display_menu {
            #[cfg(feature = "tmux_3_0")]
            if let Some(s) = display_menu.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(target_pane) = display_menu.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            #[cfg(feature = "tmux_3_0")]
            if let Some(s) = display_menu.title {
                args.extend_from_slice(&[T_KEY, &s])
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
        let output = self.subcommand(TmuxInterface::DISPLAY_MENU, &args)?;
        Ok(output)
    }
}
