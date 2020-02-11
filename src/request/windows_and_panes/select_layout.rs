use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Choose a specific layout for a window
///
/// # Manual
///
/// tmux X.X
/// ```text
/// tmux select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux 2.6
/// ```text
/// tmux select-layout [-nop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SelectLayot<'a, T> {
    /// [-E] - spread the current pane and any panes next to it out evenly
    pub spread: Option<bool>,
    /// [-n] - next-layout equivalent
    pub next: Option<bool>,
    /// [-o] - apply the last set layout if possible
    pub last: Option<bool>,
    /// [-p] - previous-layout equivalent
    pub previous: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [layout-name] - layout-name
    pub layout_name: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SelectLayot<'a, T> {
    /// [-n] - next-layout equivalent
    pub next: Option<bool>,
    /// [-o] - apply the last set layout if possible
    pub last: Option<bool>,
    /// [-p] - previous-layout equivalent
    pub previous: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [layout-name] - layout-name
    pub layout_name: Option<&'a str>,
}

impl<'a, T: Display + Default> SelectLayot<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SelectLayotBuilder<'a, T> {
    pub spread: Option<bool>,
    pub next: Option<bool>,
    pub last: Option<bool>,
    pub previous: Option<bool>,
    pub target_pane: Option<&'a T>,
    pub layout_name: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a, T: Display + Default> SelectLayotBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn spread(&mut self) -> &mut Self {
        self.spread = Some(true);
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn layout_name(&mut self, layout_name: &'a str) -> &mut Self {
        self.layout_name = Some(layout_name);
        self
    }

    pub fn build(&self) -> SelectLayot<'a, T> {
        SelectLayot {
            spread: self.spread,
            next: self.next,
            last: self.last,
            previous: self.previous,
            target_pane: self.target_pane,
            layout_name: self.layout_name,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SelectLayotBuilder<'a, T> {
    pub next: Option<bool>,
    pub last: Option<bool>,
    pub previous: Option<bool>,
    pub target_pane: Option<&'a T>,
    pub layout_name: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a, T: Display + Default> SelectLayotBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn layout_name(&mut self, layout_name: &'a str) -> &mut Self {
        self.layout_name = Some(layout_name);
        self
    }

    pub fn build(&self) -> SelectLayot<'a, T> {
        SelectLayot {
            next: self.next,
            last: self.last,
            previous: self.previous,
            target_pane: self.target_pane,
            layout_name: self.layout_name,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SELECT_LAYOUT: &'static str = "select-layout";

    /// Choose a specific layout for a window
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux select-layout [-nop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn select_layout<T: Display>(
        &mut self,
        select_layout: Option<&SelectLayot<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(select_layout) = select_layout {
            if cfg!(not(feature = "tmux_2_6")) {
                if select_layout.spread.unwrap_or(false) {
                    args.push(E_KEY);
                }
            }
            if select_layout.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if select_layout.last.unwrap_or(false) {
                args.push(o_KEY);
            }
            if select_layout.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if let Some(target_pane) = select_layout.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = select_layout.layout_name {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }

    /// Choose a specific layout for a window
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux select-layout [-nop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn select_layout<T: Display>(
        &mut self,
        select_layout: Option<&SelectLayot<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(select_layout) = select_layout {
            if select_layout.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if select_layout.last.unwrap_or(false) {
                args.push(o_KEY);
            }
            if select_layout.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if let Some(target_pane) = select_layout.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = select_layout.layout_name {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }
}
