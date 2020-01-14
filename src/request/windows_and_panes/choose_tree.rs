use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// tmux X.X
/// ```text
/// tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux 2.6
/// ```text
/// tmux choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseTree<'a> {
    #[cfg(not(feature = "tmux_2_6"))]
    /// [-G] - include all sessions in any session groups in the tree rather than only the first
    pub all: Option<bool>,
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    #[cfg(not(feature = "tmux_2_6"))]
    /// [-r] - reverses the sort order
    pub reverse_sort_order: Option<bool>,
    /// [-s] - start with collapsed sessions
    pub collapsed_sessions: Option<bool>,
    /// [-w] - start with collapsed windows
    pub collapsed_windows: Option<bool>,
    #[cfg(not(feature = "tmux_2_6"))]
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-f filter] - filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specifies the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [template] - template
    pub template: Option<&'a str>,
}

impl<'a> ChooseTree<'a> {
    pub fn new() -> ChooseTree<'a> {
        Default::default()
    }
}

/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {
    const CHOOSE_TREE: &'static str = "choose-tree";

    /// Put a pane into tree mode, where a session, window or pane may be chosen interactively
    /// from a list
    ///
    /// # Manual
    ///
    /// tmux X.X
    /// ```text
    /// tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux 2.6
    /// ```text
    /// tmux choose-tree [-Nsw] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn choose_tree(&mut self, choose_tree: Option<&ChooseTree>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_tree) = choose_tree {
            if cfg!(not(feature = "tmux_2_6")) {
                if choose_tree.all.unwrap_or(false) {
                    args.push(G_KEY);
                }
            }
            if choose_tree.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if cfg!(not(feature = "tmux_2_6")) {
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
            if cfg!(not(feature = "tmux_2_6")) {
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
            if let Some(s) = choose_tree.target_pane {
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
