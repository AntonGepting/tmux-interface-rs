use crate::error::Error;
use crate::tmux_interface::*;
use crate::PaneSize;
use std::process::Output;

/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[derive(Default, Debug)]
pub struct MovePane<'a> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub left_above: Option<bool>,
    /// [-d] -
    #[cfg(feature = "tmux_1_7")]
    pub detached: Option<bool>,
    /// [-h] - full height
    #[cfg(feature = "tmux_1_7")]
    pub horizontal: Option<bool>,
    /// [-v] - full width
    #[cfg(feature = "tmux_1_7")]
    pub vertical: Option<bool>,
    /// [-l size] - specify the size of the new pane in lines/columns
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<&'a PaneSize>,
    /// [-s src-pane] - src-pane
    #[cfg(feature = "tmux_1_7")]
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    #[cfg(feature = "tmux_1_7")]
    pub dst_pane: Option<&'a str>,
}

impl<'a> MovePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct MovePaneBuilder<'a> {
    #[cfg(feature = "tmux_1_7")]
    pub left_above: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub horizontal: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub vertical: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub size: Option<&'a PaneSize>,
    #[cfg(feature = "tmux_1_7")]
    pub src_pane: Option<&'a str>,
    #[cfg(feature = "tmux_1_7")]
    pub dst_pane: Option<&'a str>,
}

impl<'a> MovePaneBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn left_above(&mut self) -> &mut Self {
        self.left_above = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn size(&mut self, size: &'a PaneSize) -> &mut Self {
        self.size = Some(size);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn src_pane(&mut self, src_pane: &'a str) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn dst_pane(&mut self, dst_pane: &'a str) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    pub fn build(&self) -> MovePane<'a> {
        MovePane {
            #[cfg(feature = "tmux_1_7")]
            left_above: self.left_above,
            #[cfg(feature = "tmux_1_7")]
            detached: self.detached,
            #[cfg(feature = "tmux_1_7")]
            horizontal: self.horizontal,
            #[cfg(feature = "tmux_1_7")]
            vertical: self.vertical,
            #[cfg(feature = "tmux_1_7")]
            size: self.size,
            #[cfg(feature = "tmux_1_7")]
            src_pane: self.src_pane,
            #[cfg(feature = "tmux_1_7")]
            dst_pane: self.dst_pane,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const MOVE_PANE: &'static str = "move-pane";
    #[cfg(feature = "use_cmd_alias")]
    const MOVE_PANE: &'static str = "movep";

    /// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    ///
    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    pub fn move_pane(&mut self, move_pane: Option<&MovePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let p;
        if let Some(move_pane) = move_pane {
            #[cfg(feature = "tmux_1_7")]
            if move_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if move_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if move_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if move_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
            if let Some(size) = &move_pane.size {
                match size {
                    PaneSize::Size(size) => p = size.to_string(),
                    PaneSize::Percentage(size) => p = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &p]);
            }
            #[cfg(feature = "tmux_2_6")]
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
            #[cfg(feature = "tmux_1_7")]
            if let Some(src_pane) = move_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &src_pane])
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(dst_pane) = move_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &dst_pane])
            }
        }
        let output = self.command(TmuxInterface::MOVE_PANE, &args)?;
        Ok(output)
    }
}
