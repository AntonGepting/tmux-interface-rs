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
#[derive(Default, Debug)]
pub struct JoinPane<'a, T: Display> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_2_6")]
    pub left_above: Option<bool>,
    /// [-d] -
    #[cfg(feature = "tmux_1_2")]
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window height/width
    #[cfg(feature = "tmux_2_6")]
    pub full_size: Option<bool>,
    /// [-h] - full height
    #[cfg(feature = "tmux_1_2")]
    pub horizontal: Option<bool>,
    /// [-v] - full width
    #[cfg(feature = "tmux_1_2")]
    pub vertical: Option<bool>,
    /// [-l size] - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_2")]
    pub size: Option<&'a PaneSize>,
    /// [-s src-pane] - src-pane
    #[cfg(feature = "tmux_1_2")]
    pub src_pane: Option<&'a T>,
    /// [-t dst-pane] - dst-pane
    #[cfg(feature = "tmux_1_2")]
    pub dst_pane: Option<&'a T>,
}

#[derive(Default, Debug)]
pub struct JoinPaneBuilder<'a, T> {
    #[cfg(feature = "tmux_2_6")]
    pub left_above: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_2_6")]
    pub full_size: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub horizontal: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub vertical: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub size: Option<&'a PaneSize>,
    #[cfg(feature = "tmux_1_2")]
    pub src_pane: Option<&'a T>,
    #[cfg(feature = "tmux_1_2")]
    pub dst_pane: Option<&'a T>,
}

impl<'a, T: Display + Default> JoinPaneBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn left_above(&mut self) -> &mut Self {
        self.left_above = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn full_size(&mut self) -> &mut Self {
        self.full_size = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn src_pane(&mut self, src_pane: &'a T) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn dst_pane(&mut self, dst_pane: &'a T) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> JoinPane<'a, T> {
        JoinPane {
            #[cfg(feature = "tmux_2_6")]
            left_above: self.left_above,
            #[cfg(feature = "tmux_1_2")]
            detached: self.detached,
            #[cfg(feature = "tmux_2_6")]
            full_size: self.full_size,
            #[cfg(feature = "tmux_1_2")]
            horizontal: self.horizontal,
            #[cfg(feature = "tmux_1_2")]
            vertical: self.vertical,
            #[cfg(feature = "tmux_1_2")]
            size: self.size,
            #[cfg(feature = "tmux_1_2")]
            src_pane: self.src_pane,
            #[cfg(feature = "tmux_1_2")]
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
    pub fn join_pane<T: Display>(
        &mut self,
        join_pane: Option<&JoinPane<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        let l;
        if let Some(join_pane) = join_pane {
            #[cfg(feature = "tmux_2_6")]
            if join_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if join_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_2_6")]
            if join_pane.full_size.unwrap_or(false) {
                args.push(f_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if join_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if join_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_1")))]
            if let Some(size) = &join_pane.size {
                match size {
                    PaneSize::Size(size) => l = size.to_string(),
                    PaneSize::Percentage(size) => l = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &l]);
            }
            #[cfg(feature = "tmux_3_1")]
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
            #[cfg(feature = "tmux_1_2")]
            if let Some(src_pane) = join_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            #[cfg(feature = "tmux_1_2")]
            if let Some(dst_pane) = join_pane.dst_pane {
                t = dst_pane.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::JOIN_PANE, &args)?;
        Ok(output)
    }
}
