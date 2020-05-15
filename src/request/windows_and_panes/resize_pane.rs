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
    #[cfg(feature = "tmux_0_9")]
    pub down: Option<bool>,
    /// [-L] - resize left by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub left: Option<bool>,
    /// [-M] - begin mouse resizing
    #[cfg(feature = "tmux_2_1")]
    pub mouse: Option<bool>,
    /// [-R] - resize right by adjustment
    #[cfg(feature = "tmux_1_8")]
    pub right: Option<bool>,
    /// [-U] - resize up by adjustment
    #[cfg(feature = "tmux_0_9")]
    pub up: Option<bool>,
    /// [-Z] - the active pane is toggled between zoomed and unzoomed
    #[cfg(feature = "tmux_1_8")]
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<&'a T>,
    /// [-x width] - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub width: Option<usize>,
    /// [-y height] - absolute size
    #[cfg(feature = "tmux_1_8")]
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    #[cfg(feature = "tmux_0_9")]
    pub adjustment: Option<&'a str>,
}

impl<'a, T: Display + Default> ResizePane<'a, T> {
    pub fn new() -> ResizePane<'a, T> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ResizePaneBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_0_9")]
    pub down: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub left: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub mouse: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub right: Option<bool>,
    #[cfg(feature = "tmux_0_9")]
    pub up: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub zoom: Option<bool>,
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<&'a T>,
    #[cfg(feature = "tmux_1_8")]
    pub width: Option<usize>,
    #[cfg(feature = "tmux_1_8")]
    pub height: Option<usize>,
    #[cfg(feature = "tmux_0_9")]
    pub adjustment: Option<&'a str>,
}

impl<'a, T: Display + Default> ResizePaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn down(&mut self) -> &mut Self {
        self.down = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn left(&mut self) -> &mut Self {
        self.left = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(&mut self) -> &mut Self {
        self.mouse = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn right(&mut self) -> &mut Self {
        self.right = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn up(&mut self) -> &mut Self {
        self.up = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = Some(height);
        self
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn adjustment(&mut self, adjustment: &'a str) -> &mut Self {
        self.adjustment = Some(adjustment);
        self
    }

    pub fn build(&self) -> ResizePane<'a, T> {
        ResizePane {
            #[cfg(feature = "tmux_0_9")]
            down: self.down,
            #[cfg(feature = "tmux_1_8")]
            left: self.left,
            #[cfg(feature = "tmux_2_1")]
            mouse: self.mouse,
            #[cfg(feature = "tmux_1_8")]
            right: self.right,
            #[cfg(feature = "tmux_0_9")]
            up: self.up,
            #[cfg(feature = "tmux_1_8")]
            zoom: self.zoom,
            #[cfg(feature = "tmux_0_9")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_1_8")]
            width: self.width,
            #[cfg(feature = "tmux_1_8")]
            height: self.height,
            #[cfg(feature = "tmux_0_9")]
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
    pub fn resize_pane<T: Display>(
        &mut self,
        resize_pane: Option<&ResizePane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        #[cfg(feature = "tmux_1_8")]
        let x: String;
        #[cfg(feature = "tmux_1_8")]
        let y: String;
        let s;
        if let Some(resize_pane) = resize_pane {
            #[cfg(feature = "tmux_0_9")]
            if resize_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if resize_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if resize_pane.mouse.unwrap_or(false) {
                args.push(M_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if resize_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            #[cfg(feature = "tmux_0_9")]
            if resize_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if resize_pane.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_0_9")]
            if let Some(target_pane) = resize_pane.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            #[cfg(feature = "tmux_1_8")]
            if let Some(width) = resize_pane.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            #[cfg(feature = "tmux_1_8")]
            if let Some(height) = resize_pane.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            #[cfg(feature = "tmux_0_9")]
            if let Some(s) = resize_pane.adjustment {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESIZE_PANE, &args)?;
        Ok(output)
    }
}
