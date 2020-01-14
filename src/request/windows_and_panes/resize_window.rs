use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Resize a window, up, down, left or right
///
/// # Manual
///
/// ```text
/// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[derive(Default, Debug)]
pub struct ResizeWindow<'a> {
    /// [-a] - set the size of the smallest session containing the window
    pub smallest: Option<bool>, // [-a]
    /// [-A] - set the size of the largest session containing the window
    pub largest: Option<bool>,
    /// [-D] - resize down by adjustment
    pub down: Option<bool>,
    /// [-L] - resize left by adjustment
    pub left: Option<bool>,
    /// [-R] - resize right by adjustment
    pub right: Option<bool>,
    /// [-U] - resize up by adjustment
    pub up: Option<bool>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a str>,
    /// [-x width] - absolute size
    pub width: Option<usize>,
    /// [-y height] - absolute size
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    pub adjustment: Option<&'a str>,
}

impl<'a> ResizeWindow<'a> {
    pub fn new() -> ResizeWindow<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const RESIZE_WINDOW: &'static str = "resize-window";

    /// Resize a window, up, down, left or right
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    /// (alias: resizew)
    /// ```
    pub fn resize_window(&mut self, resize_window: Option<&ResizeWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(resize_window) = resize_window {
            if resize_window.smallest.unwrap_or(false) {
                args.push(a_KEY);
            }
            if resize_window.largest.unwrap_or(false) {
                args.push(A_KEY);
            }
            if resize_window.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if resize_window.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if resize_window.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if resize_window.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(s) = resize_window.target_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(width) = resize_window.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = resize_window.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(s) = resize_window.adjustment {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESIZE_WINDOW, &args)?;
        Ok(output)
    }
}
