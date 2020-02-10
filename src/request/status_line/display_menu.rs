use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// ```text
/// tmux display-menu [-c target-client] [-t target-pane] [-T title]
/// [-x position] [-y position] name key command ...
/// ```
#[derive(Default, Debug)]
pub struct DisplayMenu<'a, T: Display> {
    /// [-c target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [-T title] - title
    pub title: Option<&'a str>,
    /// [-x position] - x position of the menu
    pub x: Option<usize>,
    /// [-y position] - y position of the menu
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
    pub target_client: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub title: Option<&'a str>,
    pub x: Option<usize>,
    pub y: Option<usize>,
    //pub name: &'a str,
    //pub key: &'a str,
    //pub command: &'a str,
}

impl<'a, T: Display + Default> DisplayMenuBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn x(&mut self, x: usize) -> &mut Self {
        self.x = Some(x);
        self
    }

    pub fn y(&mut self, y: usize) -> &mut Self {
        self.y = Some(y);
        self
    }

    pub fn build(&self) -> DisplayMenu<'a, T> {
        DisplayMenu {
            target_client: self.target_client,
            target_pane: self.target_pane,
            title: self.title,
            x: self.x,
            y: self.y,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const DISPLAY_MENU: &'static str = "display-menu";

    /// # Manual
    ///
    /// ```text
    /// tmux display-menu [-c target-client] [-t target-pane] [-T title]
    /// [-x position] [-y position] name key command ...
    /// (alias: menu)
    /// ```
    ///
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
            if let Some(s) = display_menu.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(target_pane) = display_menu.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = display_menu.title {
                args.extend_from_slice(&[T_KEY, &s])
            }
            if let Some(position) = display_menu.x {
                x = position.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
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
