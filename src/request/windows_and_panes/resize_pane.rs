use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux resize-pane [-DLRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux resize-pane [-DLRU] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux resize-pane [-DU] [-p pane-index] [-t target-pane] [adjustment]
/// (alias: resizep)
/// ```
#[derive(Default, Debug)]
pub struct ResizePane<'a, T: Display> {
    /// [-D] - resize down by adjustment
    pub down: Option<bool>,
    /// [-L] - resize left by adjustment
    pub left: Option<bool>,
    /// [-M] - begin mouse resizing
    pub mouse: Option<bool>,
    /// [-R] - resize right by adjustment
    pub right: Option<bool>,
    /// [-U] - resize up by adjustment
    pub up: Option<bool>,
    /// [-Z] - the active pane is toggled between zoomed and unzoomed
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [-x width] - absolute size
    pub width: Option<usize>,
    /// [-y height] - absolute size
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    pub adjustment: Option<&'a str>,
}

impl<'a, T: Display + Default> ResizePane<'a, T> {
    pub fn new() -> ResizePane<'a, T> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ResizePaneBuilder<'a, T: Display> {
    pub down: Option<bool>,
    pub left: Option<bool>,
    pub mouse: Option<bool>,
    pub right: Option<bool>,
    pub up: Option<bool>,
    pub zoom: Option<bool>,
    pub target_pane: Option<&'a T>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub adjustment: Option<&'a str>,
}

impl<'a, T: Display + Default> ResizePaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    pub fn mouse(&mut self) -> &mut Self {
        self.mouse = Some(true);
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

    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
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

    pub fn build(&self) -> ResizePane<'a, T> {
        ResizePane {
            down: self.down,
            left: self.left,
            mouse: self.mouse,
            right: self.right,
            up: self.up,
            zoom: self.zoom,
            target_pane: self.target_pane,
            width: self.width,
            height: self.height,
            adjustment: self.adjustment,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const RESIZE_PANE: &'static str = "resize-pane";

    /// Resize a pane, up, down, left or right
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    /// (alias: resizep)
    /// ```
    pub fn resize_pane<T: Display>(
        &mut self,
        resize_pane: Option<&ResizePane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        let s;
        if let Some(resize_pane) = resize_pane {
            if resize_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if resize_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if resize_pane.mouse.unwrap_or(false) {
                args.push(M_KEY);
            }
            if resize_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if resize_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if resize_pane.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(target_pane) = resize_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(width) = resize_pane.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = resize_pane.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(s) = resize_pane.adjustment {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESIZE_PANE, &args)?;
        Ok(output)
    }
}
