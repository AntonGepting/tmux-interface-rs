use crate::error::Error;
use crate::tmux_interface::*;
use crate::PaneSize;
use std::fmt::Display;
use std::process::Output;

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
pub struct MovePane<'a, T: Display> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-h] - full height
    pub horizontal: Option<bool>,
    /// [-v] - full width
    pub vertical: Option<bool>,
    /// [-l size] - specify the size of the new pane in lines/columns
    pub size: Option<&'a PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a T>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> MovePane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct MovePaneBuilder<'a, T: Display> {
    pub left_above: Option<bool>,
    pub detached: Option<bool>,
    pub horizontal: Option<bool>,
    pub vertical: Option<bool>,
    pub size: Option<&'a PaneSize>,
    pub src_pane: Option<&'a T>,
    pub dst_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> MovePaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn left_above(&mut self) -> &mut Self {
        self.left_above = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = Some(true);
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = Some(true);
        self
    }

    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn src_pane(&mut self, src_pane: &'a T) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    pub fn dst_pane(&mut self, dst_pane: &'a T) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> MovePane<'a, T> {
        MovePane {
            left_above: self.left_above,
            detached: self.detached,
            horizontal: self.horizontal,
            vertical: self.vertical,
            size: self.size,
            src_pane: self.src_pane,
            dst_pane: self.dst_pane,
        }
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
    pub fn move_pane<T: Display>(
        &mut self,
        move_pane: Option<&MovePane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let l;
        let p;
        let s;
        let t;
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
                        PaneSize::Size(size) => l = size.to_string(),
                        PaneSize::Percentage(size) => l = format!("{}%", size),
                    };
                    args.extend_from_slice(&[l_KEY, &l]);
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
            if let Some(src_pane) = move_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_pane) = move_pane.dst_pane {
                t = dst_pane.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_PANE, &args)?;
        Ok(output)
    }
}
