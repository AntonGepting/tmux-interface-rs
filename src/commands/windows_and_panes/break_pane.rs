use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// tmux ^2.4:
/// ```text
/// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux break-pane [-dP] [-F format] [-t target-pane]
/// (alias: breakp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux break-pane [-d] [-t target-window]
/// (alias: breakp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux break-pane [-d] [-p pane-index] [-t target-window]
/// (alias: breakp)
/// ```
// FIXME: phantom conditionals
#[derive(Default, Debug)]
pub struct BreakPane<'a> {
    /// [-d] - the new window does not become the current window
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-P] - option prints information about the new window after it has been created
    #[cfg(feature = "tmux_1_7")]
    pub print: Option<bool>,
    /// [-F format] - specify format
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    /// [-n] - window-name
    #[cfg(feature = "tmux_2_4")]
    pub window_name: Option<&'a str>,
    /// [-s src-pane] - src-pane
    #[cfg(feature = "tmux_2_1")]
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub dst_pane: Option<&'a str>,
    /// [-t dst-window] - dst-window
    #[cfg(feature = "tmux_2_2")]
    pub dst_window: Option<&'a str>,
    /// [-t target-window] - target-window
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub target_window: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub target_pane: Option<&'a str>,
    /// [-p pane-index] - pane-index
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub pane_index: Option<usize>,
}

impl<'a> BreakPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct BreakPaneBuilder<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub print: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    #[cfg(feature = "tmux_2_4")]
    pub window_name: Option<&'a str>,
    #[cfg(feature = "tmux_2_1")]
    pub src_pane: Option<&'a str>,
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub dst_pane: Option<&'a str>,
    #[cfg(feature = "tmux_2_2")]
    pub dst_window: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub target_window: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub target_pane: Option<&'a str>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub pane_index: Option<usize>,
}

impl<'a> BreakPaneBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn window_name(&mut self, format: &'a str) -> &mut Self {
        self.window_name = Some(format);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn src_pane(&mut self, src_pane: &'a str) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub fn dst_pane(&mut self, dst_pane: &'a str) -> &mut Self {
        self.dst_pane = Some(dst_pane);
        self
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn dst_window(&mut self, dst_window: &'a str) -> &mut Self {
        self.dst_window = Some(dst_window);
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
    pub fn target_window(&mut self, target_window: &'a str) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    pub fn build(&self) -> BreakPane<'a> {
        BreakPane {
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_1_7")]
            print: self.print,
            #[cfg(feature = "tmux_1_7")]
            format: self.format,
            #[cfg(feature = "tmux_2_4")]
            window_name: self.window_name,
            #[cfg(feature = "tmux_2_1")]
            src_pane: self.src_pane,
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
            dst_pane: self.dst_pane,
            #[cfg(feature = "tmux_2_2")]
            dst_window: self.dst_window,
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_7")))]
            target_window: self.target_window,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            target_pane: self.target_pane,
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
            pane_index: self.dst_window,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const BREAK_PANE: &'static str = "break-pane";

    /// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    ///
    /// # Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    ///
    /// tmux ^2.2:
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-s src-pane] [-t dst-pane]
    /// (alias: breakp)
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-t target-pane]
    /// (alias: breakp)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux break-pane [-d] [-t target-window]
    /// (alias: breakp)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux break-pane [-d] [-p pane-index] [-t target-window]
    /// (alias: breakp)
    /// ```
    pub fn break_pane(
        &mut self,
        break_pane: Option<&BreakPane>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(break_pane) = break_pane {
            #[cfg(feature = "tmux_0_8")]
            if break_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if break_pane.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(format) = break_pane.format {
                args.extend_from_slice(&[F_KEY, &format])
            }
            #[cfg(feature = "tmux_2_4")]
            if let Some(window_name) = break_pane.window_name {
                args.extend_from_slice(&[n_KEY, &window_name])
            }
            #[cfg(feature = "tmux_2_1")]
            if let Some(src_pane) = break_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &src_pane])
            }
            #[cfg(feature = "tmux_2_2")]
            if let Some(dst_window) = break_pane.dst_window {
                args.extend_from_slice(&[t_KEY, &dst_window]);
            }
        }
        let output = self.subcommand(TmuxInterface::BREAK_PANE, &args)?;
        Ok(output)
    }
}
