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
    /// [-G] - include all sessions in any session groups in the tree rather than only the first
    #[cfg(feature = "tmux_2_7")]
    pub all: Option<bool>,
    /// [-N] - start without the preview
    #[cfg(feature = "tmux_2_7")]
    pub without_preview: Option<bool>,
    /// [-r] - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: Option<bool>,
    /// [-s] - start with collapsed sessions
    #[cfg(feature = "tmux_1_7")]
    pub collapsed_sessions: Option<bool>,
    /// [-w] - start with collapsed windows
    #[cfg(feature = "tmux_1_8")]
    pub collapsed_windows: Option<bool>,
    /// [-Z] - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    pub zoom: Option<bool>,
    /// [-F format] - format
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<&'a str>,
    /// [-f filter] - filter
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<&'a T>,
    /// [template] - template
    #[cfg(feature = "tmux_2_6")]
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
    #[cfg(feature = "tmux_2_7")]
    pub without_preview: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub collapsed_sessions: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub collapsed_windows: Option<bool>,
    #[cfg(feature = "tmux_2_7")]
    pub zoom: Option<bool>,
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<&'a T>,
    #[cfg(feature = "tmux_2_6")]
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

    #[cfg(feature = "tmux_2_7")]
    pub fn without_preview(&mut self) -> &mut Self {
        self.without_preview = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn collapsed_sessions(&mut self) -> &mut Self {
        self.collapsed_sessions = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_8")]
    pub fn collapsed_windows(&mut self) -> &mut Self {
        self.collapsed_windows = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_7")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn filter(&mut self, filter: &'a str) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order(&mut self, sort_order: &'a str) -> &mut Self {
        self.sort_order = Some(sort_order);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn target_pane(&mut self, target_pane: &'a T) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn template(&mut self, template: &'a str) -> &mut Self {
        self.template = Some(template);
        self
    }

    pub fn build(&self) -> ChooseTree<'a, T> {
        ChooseTree {
            #[cfg(feature = "tmux_2_7")]
            all: self.all,
            #[cfg(feature = "tmux_2_7")]
            without_preview: self.without_preview,
            #[cfg(feature = "tmux_3_1")]
            reverse_sort_order: self.reverse_sort_order,
            #[cfg(feature = "tmux_1_7")]
            collapsed_sessions: self.collapsed_sessions,
            #[cfg(feature = "tmux_1_8")]
            collapsed_windows: self.collapsed_windows,
            #[cfg(feature = "tmux_2_7")]
            zoom: self.zoom,
            #[cfg(feature = "tmux_2_6")]
            format: self.format,
            #[cfg(feature = "tmux_2_6")]
            filter: self.filter,
            #[cfg(feature = "tmux_2_6")]
            sort_order: self.sort_order,
            #[cfg(feature = "tmux_2_6")]
            target_pane: self.target_pane,
            #[cfg(feature = "tmux_2_6")]
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
    pub fn choose_tree<T: Display>(
        &mut self,
        choose_tree: Option<&ChooseTree<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s: String;
        if let Some(choose_tree) = choose_tree {
            #[cfg(feature = "tmux_2_7")]
            if choose_tree.all.unwrap_or(false) {
                args.push(G_KEY);
            }
            #[cfg(feature = "tmux_2_7")]
            if choose_tree.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if choose_tree.reverse_sort_order.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if choose_tree.collapsed_sessions.unwrap_or(false) {
                args.push(s_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if choose_tree.collapsed_windows.unwrap_or(false) {
                args.push(w_KEY);
            }
            #[cfg(feature = "tmux_2_7")]
            if choose_tree.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = choose_tree.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = choose_tree.filter {
                args.extend_from_slice(&[f_KEY, &s])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = choose_tree.sort_order {
                args.extend_from_slice(&[O_KEY, &s])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(target_pane) = choose_tree.target_pane {
                s = target_pane.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(s) = choose_tree.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_TREE, &args)?;
        Ok(output)
    }
}
