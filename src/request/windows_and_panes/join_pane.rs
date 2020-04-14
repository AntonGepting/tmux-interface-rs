use crate::error::Error;
use crate::tmux_interface::*;
use crate::PaneSize;
use std::fmt::Display;
use std::process::Output;

/// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
/// and move `src-pane` into the space
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux join-pane [-dhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct JoinPane<'a, T: Display> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window height/width
    pub full_size: Option<bool>,
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

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct JoinPane<'a, T: Display> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-h] - full height
    pub horizontal: Option<bool>,
    /// [-v] - full width
    pub vertical: Option<bool>,
    /// [-l size | -p percentage] - specify the size of the new pane in lines/columns
    pub size: Option<&'a PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a T>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a T>,
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct JoinPaneBuilder<'a, T> {
    pub left_above: Option<bool>,
    pub detached: Option<bool>,
    pub full_size: Option<bool>,
    pub horizontal: Option<bool>,
    pub vertical: Option<bool>,
    pub size: Option<&'a PaneSize>,
    pub src_pane: Option<&'a T>,
    pub dst_pane: Option<&'a T>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> JoinPaneBuilder<'a, T> {
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

    pub fn full_size(&mut self) -> &mut Self {
        self.full_size = Some(true);
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

    pub fn build(&self) -> JoinPane<'a, T> {
        JoinPane {
            left_above: self.left_above,
            detached: self.detached,
            full_size: self.full_size,
            horizontal: self.horizontal,
            vertical: self.vertical,
            size: self.size,
            src_pane: self.src_pane,
            dst_pane: self.dst_pane,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct JoinPaneBuilder<'a, T> {
    pub left_above: Option<bool>,
    pub detached: Option<bool>,
    pub horizontal: Option<bool>,
    pub vertical: Option<bool>,
    pub size: Option<&'a PaneSize>,
    pub src_pane: Option<&'a T>,
    pub dst_pane: Option<&'a T>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> JoinPaneBuilder<'a, T> {
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

    pub fn build(&self) -> JoinPane<'a, T> {
        JoinPane {
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

impl<'a, T: Display + Default> JoinPane<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const JOIN_PANE: &'static str = "join-pane";

    /// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    /// and move `src-pane` into the space
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn join_pane<T: Display>(
        &mut self,
        join_pane: Option<&JoinPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        let l;
        if let Some(join_pane) = join_pane {
            if join_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            if join_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if join_pane.full_size.unwrap_or(false) {
                args.push(f_KEY);
            }
            if join_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if join_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(size) = &join_pane.size {
                match size {
                    PaneSize::Size(size) => l = size.to_string(),
                    PaneSize::Percentage(size) => l = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &l]);
            }
            if let Some(src_pane) = join_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_pane) = join_pane.dst_pane {
                t = dst_pane.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::JOIN_PANE, &args)?;
        Ok(output)
    }

    /// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    /// and move `src-pane` into the space
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn join_pane<T: Display>(
        &mut self,
        join_pane: Option<&JoinPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        let l;
        if let Some(join_pane) = join_pane {
            if join_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            if join_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if join_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if join_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(size) = &join_pane.size {
                match size {
                    PaneSize::Size(size) => {
                        l = size.to_string();
                        args.extend_from_slice(&[l_KEY, &l]);
                    }
                    PaneSize::Percentage(size) => {
                        l = size.to_string();
                        args.extend_from_slice(&[p_KEY, &l]);
                    }
                };
            }
            if let Some(src_pane) = join_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_pane) = join_pane.dst_pane {
                t = dst_pane.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::JOIN_PANE, &args)?;
        Ok(output)
    }
}
