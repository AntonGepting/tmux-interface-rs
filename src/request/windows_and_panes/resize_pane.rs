use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// ```text
/// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
#[derive(Default, Debug)]
pub struct ResizePane<'a> {
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
    pub target_pane: Option<&'a str>,
    /// [-x width] - absolute size
    pub width: Option<usize>,
    /// [-y height] - absolute size
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    pub adjustment: Option<&'a str>,
}

impl<'a> ResizePane<'a> {
    pub fn new() -> ResizePane<'a> {
        Default::default()
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
    pub fn resize_pane(&mut self, resize_pane: Option<&ResizePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
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
            if let Some(s) = resize_pane.target_pane {
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
