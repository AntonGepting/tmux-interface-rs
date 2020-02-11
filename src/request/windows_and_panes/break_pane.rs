use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// ```text
/// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
#[derive(Default, Debug)]
pub struct BreakPane<'a, T, U: Display> {
    /// [-d] - the new window does not become the current window
    pub detached: Option<bool>,
    /// [-P] - option prints information about the new window after it has been created
    pub print: Option<bool>,
    /// [-F format] - specify format
    pub format: Option<&'a str>,
    /// [-n] - window-name
    pub window_name: Option<&'a str>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a T>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a U>,
}

impl<'a, T: Display + Default, U: Display + Default> BreakPane<'a, T, U> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct BreakPaneBuilder<'a, T: Display, U: Display> {
    pub detached: Option<bool>,
    pub print: Option<bool>,
    pub format: Option<&'a str>,
    pub window_name: Option<&'a str>,
    pub src_pane: Option<&'a T>,
    pub dst_window: Option<&'a U>,
}

impl<'a, T: Display + Default, U: Display + Default> BreakPaneBuilder<'a, T, U> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn window_name(&mut self, format: &'a str) -> &mut Self {
        self.window_name = Some(format);
        self
    }

    pub fn src_pane(&mut self, src_pane: &'a T) -> &mut Self {
        self.src_pane = Some(src_pane);
        self
    }

    pub fn dst_window(&mut self, dst_window: &'a U) -> &mut Self {
        self.dst_window = Some(dst_window);
        self
    }

    pub fn build(&self) -> BreakPane<'a, T, U> {
        BreakPane {
            detached: self.detached,
            print: self.print,
            format: self.format,
            window_name: self.window_name,
            src_pane: self.src_pane,
            dst_window: self.dst_window,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const BREAK_PANE: &'static str = "break-pane";

    /// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    pub fn break_pane<T: Display, U: Display>(
        &mut self,
        break_pane: Option<&BreakPane<T, U>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let t;
        if let Some(break_pane) = break_pane {
            if break_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if break_pane.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = break_pane.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = break_pane.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(src_pane) = break_pane.src_pane {
                s = src_pane.to_string();
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(dst_window) = break_pane.dst_window {
                t = dst_window.to_string();
                args.extend_from_slice(&[t_KEY, &t]);
            }
        }
        let output = self.subcommand(TmuxInterface::BREAK_PANE, &args)?;
        Ok(output)
    }
}
