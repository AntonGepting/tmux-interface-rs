use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Resize a window, up, down, left or right
///
/// # Manual
///
/// tmux ^2.9:
/// ```text
/// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[derive(Default, Debug)]
pub struct ResizeWindow<'a> {
    /// [-a] - set the size of the smallest session containing the window
    #[cfg(feature = "tmux_2_9")]
    pub smallest: Option<bool>,
    /// [-A] - set the size of the largest session containing the window
    #[cfg(feature = "tmux_2_9")]
    pub largest: Option<bool>,
    /// [-D] - resize down by adjustment
    #[cfg(feature = "tmux_2_9")]
    pub down: Option<bool>,
    /// [-L] - resize left by adjustment
    #[cfg(feature = "tmux_2_9")]
    pub left: Option<bool>,
    /// [-R] - resize right by adjustment
    #[cfg(feature = "tmux_2_9")]
    pub right: Option<bool>,
    /// [-U] - resize up by adjustment
    #[cfg(feature = "tmux_2_9")]
    pub up: Option<bool>,
    /// [-t target-window] - target-window
    #[cfg(feature = "tmux_2_9")]
    pub target_window: Option<&'a str>,
    /// [-x width] - absolute size
    #[cfg(feature = "tmux_2_9")]
    pub width: Option<usize>,
    /// [-y height] - absolute size
    #[cfg(feature = "tmux_2_9")]
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    #[cfg(feature = "tmux_2_9")]
    pub adjustment: Option<&'a str>,
}

impl<'a> ResizeWindow<'a> {
    pub fn new() -> ResizeWindow<'a> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ResizeWindowBuilder<'a> {
    #[cfg(feature = "tmux_2_9")]
    pub smallest: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub largest: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub down: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub left: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub right: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub up: Option<bool>,
    #[cfg(feature = "tmux_2_9")]
    pub target_window: Option<&'a str>,
    #[cfg(feature = "tmux_2_9")]
    pub width: Option<usize>,
    #[cfg(feature = "tmux_2_9")]
    pub height: Option<usize>,
    #[cfg(feature = "tmux_2_9")]
    pub adjustment: Option<&'a str>,
}

impl<'a> ResizeWindowBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn smallest(&mut self) -> &mut Self {
        self.smallest = Some(true);
        self
    }

    pub fn largest(&mut self) -> &mut Self {
        self.largest = Some(true);
        self
    }

    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    pub fn target_window(&mut self, target_window: &'a str) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = Some(height);
        self
    }

    pub fn adjustment(&mut self, adjustment: &'a str) -> &mut Self {
        self.adjustment = Some(adjustment);
        self
    }

    pub fn build(&self) -> ResizeWindow<'a> {
        ResizeWindow {
            smallest: self.smallest,
            largest: self.largest,
            down: self.down,
            left: self.left,
            right: self.right,
            up: self.up,
            target_window: self.target_window,
            width: self.width,
            height: self.height,
            adjustment: self.adjustment,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const RESIZE_WINDOW: &'static str = "resize-window";

    /// Resize a window, up, down, left or right
    ///
    /// # Manual
    ///
    /// tmux ^2.9a:
    /// ```text
    /// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    /// (alias: resizew)
    /// ```
    pub fn resize_window(
        &mut self,
        resize_window: Option<&ResizeWindow<T>>,
    ) -> Result<Output, Error> {
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
            if let Some(target_window) = resize_window.target_window {
                args.extend_from_slice(&[t_KEY, &target_window])
            }
            if let Some(width) = resize_window.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = resize_window.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(adjustment) = resize_window.adjustment {
                args.push(adjustment)
            }
        }
        let output = self.subcommand(TmuxInterface::RESIZE_WINDOW, &args)?;
        Ok(output)
    }
}
