use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

#[derive(Debug)]
pub enum PaneSize {
    Size(usize),
    Percentage(usize),
}

/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[derive(Default, Debug)]
pub struct MovePane<'a> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-h] - full height
    pub horizontal: Option<bool>,
    /// [-v] - full width
    pub vertical: Option<bool>,
    /// [-l size] - specify the size of the new pane in lines/columns
    pub size: Option<PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
}

impl<'a> MovePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const MOVE_PANE: &'static str = "move-pane";

    /// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    pub fn move_pane(&mut self, move_pane: Option<&MovePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let p;
        if let Some(move_pane) = move_pane {
            if move_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            if move_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if move_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if move_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if cfg!(not(feature = "tmux_2_6")) {
                if let Some(size) = &move_pane.size {
                    match size {
                        PaneSize::Size(size) => s = size.to_string(),
                        PaneSize::Percentage(size) => s = format!("{}%", size),
                    };
                    args.extend_from_slice(&[l_KEY, &s]);
                }
            }
            if cfg!(feature = "tmux_2_6") {
                if let Some(size) = &move_pane.size {
                    match size {
                        PaneSize::Size(size) => {
                            p = size.to_string();
                            args.extend_from_slice(&[l_KEY, &p]);
                        }
                        PaneSize::Percentage(size) => {
                            p = size.to_string();
                            args.extend_from_slice(&[p_KEY, &p]);
                        }
                    };
                }
            }
            if let Some(s) = move_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = move_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_PANE, &args)?;
        Ok(output)
    }
}
