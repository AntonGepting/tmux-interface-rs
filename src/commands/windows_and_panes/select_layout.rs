use crate::error::Error;
use crate::tmux_interface::*;
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
pub struct SelectLayot<'a> {
    /// [-E] - spread the current pane and any panes next to it out evenly
    #[cfg(feature = "tmux_2_7")]
    pub spread: Option<bool>,
    /// [-n] - next-layout equivalent
    #[cfg(feature = "tmux_1_5")]
    pub next_layout: Option<bool>,
    /// [-o] - apply the last set layout if possible
    #[cfg(feature = "tmux_2_1")]
    pub last_layout: Option<bool>,
    /// [-p] - previous-layout equivalent
    #[cfg(feature = "tmux_1_5")]
    pub previous_layout: Option<bool>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<&'a str>,
    /// [layout-name] - layout-name
    #[cfg(feature = "tmux_1_0")]
    pub layout_name: Option<&'a str>,
}

impl<'a> SelectLayot<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct SelectLayotBuilder<'a> {
    #[cfg(feature = "tmux_2_7")]
    pub spread: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub next_layout: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub last_layout: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub previous_layout: Option<bool>,
    #[cfg(feature = "tmux_0_9")]
    pub target_pane: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub layout_name: Option<&'a str>,
}

impl<'a> SelectLayotBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    #[cfg(feature = "tmux_2_7")]
    pub fn spread(&mut self) -> &mut Self {
        self.spread = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn next_layout(&mut self) -> &mut Self {
        self.next_layout = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn last_layout(&mut self) -> &mut Self {
        self.last_layout = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn previous_layout(&mut self) -> &mut Self {
        self.previous_layout = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn layout_name(&mut self, layout_name: &'a str) -> &mut Self {
        self.layout_name = Some(layout_name);
        self
    }

    pub fn build(&self) -> SelectLayot<'a> {
        SelectLayot {
            #[cfg(feature = "tmux_2_7")]
            spread: self.spread,
            #[cfg(feature = "tmux_1_5")]
            next_layout: self.next_layout,
            #[cfg(feature = "tmux_2_1")]
            last_layout: self.last_layout,
            #[cfg(feature = "tmux_1_5")]
            previous_layout: self.previous_layout,
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
    pub fn select_layout(
        &mut self,
        select_layout: Option<&SelectLayot>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_layout) = select_layout {
            #[cfg(feature = "tmux_2_7")]
            if select_layout.spread.unwrap_or(false) {
                args.push(E_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if select_layout.next_layout.unwrap_or(false) {
                args.push(n_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if select_layout.last_layout.unwrap_or(false) {
                args.push(o_KEY);
            }
            #[cfg(feature = "tmux_1_5")]
            if select_layout.previous_layout.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_0_9")]
            if let Some(target_pane) = select_layout.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(layout_name) = select_layout.layout_name {
                args.push(&layout_name)
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }
}
