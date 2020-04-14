use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// tmux choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux choose-tree [-suw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux choose-tree [-sw] [-b session-template] [-c window-template] [-S format] [-W format]
/// [-t target-window]
/// ```
#[derive(Default, Debug)]
pub struct ChooseTree<'a, T: Display> {
    #[cfg(feature = "tmux_2_7")]
    /// [-G] - include all sessions in any session groups in the tree rather than only the first
    pub all: Option<bool>,
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    /// [-r] - reverses the sort order
    pub reverse_sort_order: Option<bool>,
    /// [-s] - start with collapsed sessions
    pub collapsed_sessions: Option<bool>,
    /// [-w] - start with collapsed windows
    pub collapsed_windows: Option<bool>,
    #[cfg(feature = "tmux_2_7")]
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-f filter] - filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specifies the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a T>,
    /// [template] - template
    pub template: Option<&'a str>,
}

impl<'a, T: Display + Default> ChooseTree<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ChooseTreeBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_2_7")]
    pub all: Option<bool>,
    pub without_preview: Option<bool>,
    #[cfg(feature = "tmux_X_X")]
    pub reverse_sort_order: Option<bool>,
    pub collapsed_sessions: Option<bool>,
    pub collapsed_windows: Option<bool>,
    #[cfg(feature = "tmux_2_7")]
    pub zoom: Option<bool>,
    pub format: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort_order: Option<&'a str>,
    pub target_pane: Option<&'a T>,
    pub template: Option<&'a str>,
}

impl<'a, T: Display + Default> ChooseTreeBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn all(&mut self) -> &mut Self {
        self.all = Some(true);
        self
    }

    pub fn without_preview(&mut self) -> &mut Self {
        self.without_preview = Some(true);
        self
    }

    #[cfg(feature = "tmux_X_X")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = Some(true);
        self
    }

    pub fn collapsed_sessions(&mut self) -> &mut Self {
        self.collapsed_sessions = Some(true);
        self
    }

    pub fn collapsed_windows(&mut self) -> &mut Self {
        self.collapsed_windows = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn filter(&mut self, filter: &'a str) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    pub fn sort_order(&mut self, sort_order: &'a str) -> &mut Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    pub fn template(&mut self, template: &'a str) -> &mut Self {
        self.template = Some(template);
        self
    }

    pub fn build(&self) -> ChooseTree<'a, T> {
        ChooseTree {
            #[cfg(feature = "tmux_2_7")]
            all: self.all,
            without_preview: self.without_preview,
            #[cfg(feature = "tmux_X_X")]
            reverse_sort_order: self.reverse_sort_order,
            collapsed_sessions: self.collapsed_sessions,
            collapsed_windows: self.collapsed_windows,
            #[cfg(feature = "tmux_2_7")]
            zoom: self.zoom,
            format: self.format,
            filter: self.filter,
            sort_order: self.sort_order,
            target_pane: self.target_pane,
            template: self.template,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const CHOOSE_TREE: &'static str = "choose-tree";

    /// Put a pane into tree mode, where a session, window or pane may be chosen interactively
    /// from a list
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn choose_tree<T: Display>(
        &mut self,
        choose_tree: Option<&ChooseTree<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(choose_tree) = choose_tree {
            #[cfg(feature = "tmux_2_7")]
            {
                if choose_tree.all.unwrap_or(false) {
                    args.push(G_KEY);
                }
            }
            if choose_tree.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            #[cfg(feature = "tmux_X_X")]
            {
                if choose_tree.reverse_sort_order.unwrap_or(false) {
                    args.push(r_KEY);
                }
            }
            if choose_tree.collapsed_sessions.unwrap_or(false) {
                args.push(s_KEY);
            }
            if choose_tree.collapsed_windows.unwrap_or(false) {
                args.push(w_KEY);
            }
            #[cfg(feature = "tmux_2_7")]
            {
                if choose_tree.zoom.unwrap_or(false) {
                    args.push(Z_KEY);
                }
            }
            if let Some(s) = choose_tree.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = choose_tree.filter {
                args.extend_from_slice(&[f_KEY, &s])
            }
            if let Some(s) = choose_tree.sort_order {
                args.extend_from_slice(&[O_KEY, &s])
            }
            if let Some(target_pane) = choose_tree.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = choose_tree.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_TREE, &args)?;
        Ok(output)
    }
}
