use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux move-window [-rdk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux move-window [-dk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux move-window [-d] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[derive(Default, Debug)]
pub struct MoveWindow<'a, T: Display> {
    /// [-a] - the window is moved to the next index up
    #[cfg(feature = "tmux_2_1")]
    pub add: Option<bool>,
    /// [-r] - all windows in the session are renumbered in sequential order
    #[cfg(feature = "tmux_1_7")]
    pub renumber: Option<bool>,
    /// [-d] - the newly linked window is not selected
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    #[cfg(feature = "tmux_1_3")]
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<&'a T>,
    /// [-t dst-window] - dst-window
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> MoveWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct MoveWindowBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_2_1")]
    pub add: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub renumber: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_1_3")]
    pub kill: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<&'a T>,
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> MoveWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn renumber(&mut self) -> &mut Self {
        self.renumber = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn src_window(&mut self, src_window: &'a T) -> &mut Self {
        self.src_window = Some(src_window);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window(&mut self, dst_window: &'a T) -> &mut Self {
        self.dst_window = Some(dst_window);
        self
    }

    pub fn build(&self) -> MoveWindow<'a, T> {
        MoveWindow {
            #[cfg(feature = "tmux_2_1")]
            add: self.add,
            #[cfg(feature = "tmux_1_7")]
            renumber: self.renumber,
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_1_3")]
            kill: self.kill,
            #[cfg(feature = "tmux_0_8")]
            src_window: self.src_window,
            #[cfg(feature = "tmux_0_8")]
            dst_window: self.dst_window,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const MOVE_WINDOW: &'static str = "move-window";

    /// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
    ///
    /// # Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux move-window [-ardk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux move-window [-rdk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    ///
    /// tmux ^1.3:
    /// ```text
    /// tmux move-window [-dk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux move-window [-d] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    pub fn move_window<T: Display>(
        &mut self,
        move_window: Option<&MoveWindow<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        if let Some(move_window) = move_window {
            #[cfg(feature = "tmux_2_1")]
            {
                if move_window.add.unwrap_or(false) {
                    args.push(a_KEY);
                }
            }
            #[cfg(feature = "tmux_1_7")]
            {
                if move_window.renumber.unwrap_or(false) {
                    args.push(r_KEY);
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if move_window.detached.unwrap_or(false) {
                    args.push(d_KEY);
                }
            }
            #[cfg(feature = "tmux_1_3")]
            {
                if move_window.kill.unwrap_or(false) {
                    args.push(k_KEY);
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(src_window) = move_window.src_window {
                    s = src_window.to_string();
                    args.extend_from_slice(&[s_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(dst_window) = move_window.dst_window {
                    t = dst_window.to_string();
                    args.extend_from_slice(&[t_KEY, &t])
                }
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_WINDOW, &args)?;
        Ok(output)
    }
}
