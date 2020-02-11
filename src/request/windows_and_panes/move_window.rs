use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// ```text
/// tmux move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[derive(Default, Debug)]
pub struct MoveWindow<'a, T: Display> {
    /// [-a] - the window is moved to the next index up
    pub add: Option<bool>,
    /// [-r] - all windows in the session are renumbered in sequential order
    pub renumber: Option<bool>,
    /// [-d] - the newly linked window is not selected
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    pub src_window: Option<&'a T>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> MoveWindow<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct MoveWindowBuilder<'a, T: Display> {
    pub add: Option<bool>,
    pub renumber: Option<bool>,
    pub detached: Option<bool>,
    pub kill: Option<bool>,
    pub src_window: Option<&'a T>,
    pub dst_window: Option<&'a T>,
}

impl<'a, T: Display + Default> MoveWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self) -> &mut Self {
        self.add = Some(true);
        self
    }

    pub fn renumber(&mut self) -> &mut Self {
        self.renumber = Some(true);
        self
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    pub fn src_window(&mut self, src_window: &'a T) -> &mut Self {
        self.src_window = Some(src_window);
        self
    }

    pub fn dst_window(&mut self, dst_window: &'a T) -> &mut Self {
        self.dst_window = Some(dst_window);
        self
    }

    pub fn build(&self) -> MoveWindow<'a, T> {
        MoveWindow {
            add: self.add,
            renumber: self.renumber,
            detached: self.detached,
            kill: self.kill,
            src_window: self.src_window,
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
    /// ```text
    /// tmux move-window [-ardk] [-s src-window] [-t dst-window]
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
            if move_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if move_window.renumber.unwrap_or(false) {
                args.push(r_KEY);
            }
            if move_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if move_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(src_window) = move_window.src_window {
                s = src_window.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_window) = move_window.dst_window {
                t = dst_window.to_string();
                args.extend_from_slice(&[t_KEY, &t])
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_WINDOW, &args)?;
        Ok(output)
    }
}
