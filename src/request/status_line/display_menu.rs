use crate::error::Error;
use crate::tmux_interface::*;
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
pub struct DisplayMenu<'a> {
    /// [-c target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
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

impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Status line" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE)
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
            if let Some(s) = display_menu.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = display_menu.target_pane {
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
