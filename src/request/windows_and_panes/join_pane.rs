use crate::error::Error;
use crate::tmux_interface::*;
use crate::PaneSize;
use std::process::Output;

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
#[derive(Default, Debug)]
pub struct JoinPane<'a> {
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
    pub size: Option<PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct JoinPane<'a> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-h] - full height
    pub horizontal: Option<bool>,
    /// [-v] - full width
    pub vertical: Option<bool>,
    /// [-l size | -p percentage] - specify the size of the new pane in lines/columns
    pub size: Option<PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
}

impl<'a> JoinPane<'a> {
    pub fn new() -> JoinPane<'a> {
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
    pub fn join_pane(&mut self, join_pane: Option<&JoinPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
                    PaneSize::Size(size) => s = size.to_string(),
                    PaneSize::Percentage(size) => s = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &s]);
            }
            if let Some(s) = join_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = join_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
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
    pub fn join_pane(&mut self, join_pane: Option<&JoinPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
                        s = size.to_string();
                        args.extend_from_slice(&[l_KEY, &s]);
                    }
                    PaneSize::Percentage(size) => {
                        s = size.to_string();
                        args.extend_from_slice(&[p_KEY, &s]);
                    }
                };
            }
            if let Some(s) = join_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = join_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::JOIN_PANE, &args)?;
        Ok(output)
    }
}
