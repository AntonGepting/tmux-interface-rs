use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Choose a specific layout for a window
///
/// # Manual
///
/// tmux ^2.7:
/// ```text
/// tmux select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux select-layout [-nop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-layout [-np] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux select-layout [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux select-layout [-t target-pane] layout-name
/// (alias: selectl)
/// ```
#[derive(Default, Debug)]
pub struct SelectLayot<'a, T> {
    /// [-E] - spread the current pane and any panes next to it out evenly
    #[cfg(feature = "tmux_2_7")]
    pub spread: Option<bool>,
    /// [-n] - next-layout equivalent
    #[cfg(feature = "tmux_1_5")]
    pub next: Option<bool>,
    /// [-o] - apply the last set layout if possible
    #[cfg(feature = "tmux_2_1")]
    pub last: Option<bool>,
    /// [-p] - previous-layout equivalent
    #[cfg(feature = "tmux_1_5")]
    pub previous: Option<bool>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<&'a T>,
    /// [layout-name] - layout-name
    #[cfg(feature = "tmux_1_0")]
    pub layout_name: Option<&'a str>,
}

impl<'a, T: Display + Default> SelectLayot<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SelectLayotBuilder<'a, T> {
    #[cfg(feature = "tmux_2_7")]
    pub spread: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub next: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub last: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub previous: Option<bool>,
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<&'a T>,
    #[cfg(feature = "tmux_1_0")]
    pub layout_name: Option<&'a str>,
}

impl<'a, T: Display + Default> SelectLayotBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
    #[cfg(feature = "tmux_2_7")]
    pub fn spread(&mut self) -> &mut Self {
        self.spread = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn layout_name(&mut self, layout_name: &'a str) -> &mut Self {
        self.layout_name = Some(layout_name);
        self
    }

    pub fn build(&self) -> SelectLayot<'a, T> {
        SelectLayot {
            #[cfg(feature = "tmux_2_7")]
            spread: self.spread,
            #[cfg(feature = "tmux_1_5")]
            next: self.next,
            #[cfg(feature = "tmux_2_1")]
            last: self.last,
            #[cfg(feature = "tmux_1_5")]
            previous: self.previous,
            #[cfg(feature = "tmux_0_9")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_1_0")]
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
    /// tmux ^2.7:
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux select-layout [-nop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux select-layout [-np] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux select-layout [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    ///
    /// tmux ^0.9:
    /// ```text
    /// tmux select-layout [-t target-pane] layout-name
    /// (alias: selectl)
    /// ```
    pub fn select_layout<T: Display>(
        &mut self,
        select_layout: Option<&SelectLayot<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(select_layout) = select_layout {
            #[cfg(feature = "tmux_2_7")]
            {
                if select_layout.spread.unwrap_or(false) {
                    args.push(E_KEY);
                }
            }
            #[cfg(feature = "tmux_1_5")]
            {
                if select_layout.next.unwrap_or(false) {
                    args.push(n_KEY);
                }
            }
            #[cfg(feature = "tmux_2_1")]
            {
                if select_layout.last.unwrap_or(false) {
                    args.push(o_KEY);
                }
            }
            #[cfg(feature = "tmux_1_5")]
            {
                if select_layout.previous.unwrap_or(false) {
                    args.push(p_KEY);
                }
            }
            #[cfg(feature = "tmux_0_9")]
            {
                if let Some(target_pane) = select_layout.target_pane {
                    s = target_pane.to_string();
                    args.extend_from_slice(&[t_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_1_0")]
            {
                if let Some(s) = select_layout.layout_name {
                    args.push(&s)
                }
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }
}
