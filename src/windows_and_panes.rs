use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;



/// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
///
/// # Manual
///
/// ```text
/// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
/// (alias: breakp)
/// ```
#[derive(Default)]
pub struct BreakPane<'a> {
    pub detached: Option<bool>,                 // [-d]
    pub print: Option<bool>,                    // [-P]
    pub format: Option<&'a str>,                // [-F format]
    pub window_name: Option<&'a str>,           // [-n window-name]
    pub src_pane: Option<&'a str>,              // [-s src-pane]
    pub dst_window: Option<&'a str>,            // [-t dst-window]
}


impl<'a> BreakPane<'a> {
    pub fn new() -> BreakPane<'a> {
        Default::default()
    }
}


/// Capture the contents of a pane
///
/// # Manual
///
/// ```text
/// tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line]
/// [-t target-pane]
/// (alias: capturep)
/// ```
#[derive(Default)]
pub struct CapturePane<'a> {
    pub alternate_screen: Option<bool>,         // [-a]
    pub escape_sequences: Option<bool>,         // [-e]
    pub stdout: Option<bool>,                   // [-p]
    pub pane: Option<bool>,                     // [-P]
    pub quite: Option<bool>,                    // [-q]
    pub escape_non_printable: Option<bool>,     // [-C]
    pub join: Option<bool>,                     // [-J]
    pub buffer_name: Option<&'a str>,           // [-b buffen_name]
    pub end_line: Option<&'a str>,              // [-E end_line]
    pub start_line: Option<&'a str>,            // [-S start_line]
}


impl<'a> CapturePane<'a> {
    pub fn new() -> CapturePane<'a> {
        Default::default()
    }
}


/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// ```text
/// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default)]
pub struct ChooseClient<'a> {
    pub without_preview: Option<bool>,          // [-N]
    pub zoom: Option<bool>,                     // [-Z]
    pub format: Option<&'a str>,                // [-F format]
    pub filter: Option<&'a str>,                // [-f filter]
    pub sort_order: Option<&'a str>,            // [-O sort-order]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub template: Option<&'a str>,              // [template]
}


impl<'a> ChooseClient<'a> {
    pub fn new() -> ChooseClient<'a> {
        Default::default()
    }
}


/// Put a pane into tree mode, where a session, window or pane may be chosen interactively
/// from a list
///
/// # Manual
///
/// ```text
/// tmux choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default)]
pub struct ChooseTree<'a> {
    pub all: Option<bool>,                      // [-G]
    pub without_preview: Option<bool>,          // [-N]
    pub collapsed_sessions: Option<bool>,       // [-s]
    pub collapsed_windows: Option<bool>,        // [-w]
    pub zoom: Option<bool>,                     // [-Z]
    pub format: Option<&'a str>,                // [-F format]
    pub filter: Option<&'a str>,                // [-f filter]
    pub sort_order: Option<&'a str>,            // [-O sort-order]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub template: Option<&'a str>,              // [template]
}


impl<'a> ChooseTree<'a> {
    pub fn new() -> ChooseTree<'a> {
        Default::default()
    }
}


/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// ```text
/// tmux find-window [-CNTZ] [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Default)]
pub struct FindWindow<'a> {
    pub only_visible: Option<bool>,             // [-C]
    pub only_name: Option<bool>,                // [-N]
    pub only_title: Option<bool>,               // [-T]
    pub zoom: Option<bool>,                     // [-Z]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub match_string: &'a str,                  // match-string
}


impl<'a> FindWindow<'a> {
    pub fn new() -> FindWindow<'a> {
        Default::default()
    }
}


/// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
/// and move `src-pane` into the space
///
/// # Manual
///
/// ```text
/// tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
#[derive(Default)]
pub struct JoinPane<'a> {
    pub left_above: Option<bool>,               // [-b]
    pub detached: Option<bool>,                 // [-d]
    pub horizontal: Option<bool>,               // [-h]
    pub vertical: Option<bool>,                 // [-v]
    pub size: Option<usize>,                    // [-l size]
    pub percentage: Option<usize>,              // [-p percentage]
    pub src_pane: Option<&'a str>,              // [-s src-pane]
    pub dst_pane: Option<&'a str>,              // [-t dst-pane]
}


impl<'a> JoinPane<'a> {
    pub fn new() -> JoinPane<'a> {
        Default::default()
    }
}


/// Structure for creating new window, using `tmux new-window` command
///
/// # Manual
///
/// ```text
/// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window]
/// [shell-command]
/// (alias: neww)
/// ```
#[derive(Default)]
pub struct NewWindow<'a> {
    pub add: Option<bool>,                      // [-a]
    pub detached: Option<bool>,                 // [-d]
    pub kill: Option<bool>,                     // [-k]
    pub print: Option<bool>,                    // [-P]
    pub cwd: Option<&'a str>,                   // [-c start-directory]
    pub format: Option<&'a str>,                // [-F format]
    pub window_name: Option<&'a str>,           // [-n window-name]
    pub target_window: Option<&'a str>,         // [-t target-window]
    pub shell_command: Option<&'a str>,         // [shell-command]
}


impl<'a> NewWindow<'a> {
    pub fn new() -> NewWindow<'a> {
        Default::default()
    }
}


/// # Manual
///
/// ```text
/// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size | -p percentage]
/// [-t target-pane] [shell-command] [-F format]
/// (alias: splitw)
/// ```
#[derive(Default)]
pub struct SplitWindow<'a> {
    pub before: Option<bool>,                   // [-b]
    pub detached: Option<bool>,                 // [-d]
    pub full: Option<bool>,                     // [-f]
    pub horizontal: Option<bool>,               // [-h]
    pub vertical: Option<bool>,                 // [-v]
    pub print: Option<bool>,                    // [-P]
    pub cwd: Option<&'a str>,                   // [-c start-directory]
    pub size: Option<usize>,                    // [-l size]
    pub percentage: Option<usize>,              // [-p percentage]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
    pub shell_command: Option<&'a str>,         // [shell-command]
    pub format: Option<&'a str>,                // [-F format]
}


impl<'a> SplitWindow<'a> {
    pub fn new() -> SplitWindow<'a> {
        Default::default()
    }
}


///
/// # Manual
///
/// ```text
/// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
#[derive(Default)]
pub struct SelectPane<'a> {
    pub down: Option<bool>,                     // [-D]
    pub disable: Option<bool>,                  // [-d]
    pub enable: Option<bool>,                   // [-e]
    pub show_style: Option<bool>,               // [-g]
    pub left: Option<bool>,                     // [-L]
    pub last: Option<bool>,                     // [-l]
    pub set_marked: Option<bool>,               // [-M]
    pub clear_marked: Option<bool>,             // [-m]
    pub right: Option<bool>,                    // [-R]
    pub up: Option<bool>,                       // [-U]
    pub style: Option<&'a str>,                 // [-P style]
    pub title: Option<&'a str>,                 // [-T title]
    pub target_pane: Option<&'a str>,           // [-t target-pane]
}


impl<'a> SelectPane<'a> {
    pub fn new() -> SelectPane<'a> {
        Default::default()
    }
}


///
/// # Manual
///
/// ```text
/// tmux select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
#[derive(Default)]
pub struct SelectWindow<'a> {
    pub last: Option<bool>,                     // [-l]
    pub next: Option<bool>,                     // [-n]
    pub previous: Option<bool>,                 // [-p]
    pub switch: Option<bool>,                   // [-T]
    pub target_window: Option<&'a str>          // [-t target-window]
}


impl<'a> SelectWindow<'a> {
    pub fn new() -> SelectWindow<'a> {
        Default::default()
    }
}


/// Windows and panes
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES)
impl<'a> TmuxInterface<'a> {

    const COPY_MODE: &'static str = "copy-mode";
    const BREAK_PANE: &'static str = "break-pane";
    const CAPTURE_PANE: &'static str = "capture-pane";
    const CHOOSE_CLIENT: &'static str = "choose-client";
    const CHOOSE_TREE: &'static str = "choose-tree";
    const DISPLAY_PANES: &'static str = "display-panes";
    const FIND_WINDOW: &'static str = "find-window";
    const JOIN_PANE: &'static str = "join-pane";
    const KILL_PANE: &'static str = "kill-pane";
    const KILL_WINDOW: &'static str = "kill-window";
    const LAST_PANE: &'static str = "last-pane";
    const LAST_WINDOW: &'static str = "last-window";
    //const LINK_WINDOW: &'static str = "link-window";
    const LIST_PANES: &'static str = "list-panes";
    const LIST_WINDOWS: &'static str = "list-windows";
    //const MOVE_PANE: &'static str = "move-pane";
    //const MOVE_WINDOW: &'static str = "move-window";
    const NEW_WINDOW: &'static str = "new-window";
    const NEXT_LAYOUT: &'static str = "next-layout";
    const NEXT_WINDOW: &'static str = "next-window";
    //const PIPE_PANE: &'static str = "pipe-pane";
    const PREVIOUS_LAYOUT: &'static str = "previous-layout";
    const PREVIOUS_WINDOW: &'static str = "previous-window";
    const RENAME_WINDOW: &'static str = "rename-window";
    //const RESIZE_PANE: &'static str = "resize-pane";
    //const RESIZE_WINDOW: &'static str = "resize-window";
    //const RESPAWN_WINDOW: &'static str = "respawn-window";
    const ROTATE_WINDOW: &'static str = "rotate-window";
    //const SELECT_LAYOUT: &'static str = "select-layout";
    const SELECT_PANE: &'static str = "select-pane";
    const SELECT_WINDOW: &'static str = "select-window";
    const SPLIT_WINDOW: &'static str = "split-window";
    //const SWAP_PANE: &'static str = "swap-pane";
    const SWAP_WINDOW: &'static str = "swap-window";
    const UNLINK_WINDOW: &'static str = "unlink-window";


    /// Enter copy mode
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux copy-mode [-Meu] [-t target-pane]
    /// ```
    pub fn copy_mode(&self,
                     mouse_drag: Option<bool>,
                     bottom_exit: Option<bool>,
                     page_up: Option<bool>,
                     target_pane: Option<&str>
                     ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if mouse_drag.unwrap_or(false) { args.push(M_KEY); }
        if bottom_exit.unwrap_or(false) { args.push(e_KEY); }
        if page_up.unwrap_or(false) { args.push(u_KEY); }
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::COPY_MODE, &args)?;
        Ok(output.status.success())
    }


    /// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    pub fn break_pane(&self, break_pane: &BreakPane) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if break_pane.detached.unwrap_or(false) { args.push(d_KEY); }
        if break_pane.print.unwrap_or(false) { args.push(P_KEY); }
        break_pane.format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        break_pane.window_name.and_then(|s| Some(args.extend_from_slice(&[n_KEY, &s])));
        break_pane.src_pane.and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        break_pane.dst_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::BREAK_PANE, &args)?;
        Ok(output.status.success())
    }


    /// Capture the contents of a pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux capture-pane [-aepPqCJ] [-b buffer-name] [-E end-line] [-S start-line]
    /// [-t target-pane]
    /// (alias: capturep)
    /// ```
    pub fn capture_pane(&self, capture_pane: &CapturePane) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if capture_pane.alternate_screen.unwrap_or(false) { args.push(a_KEY); }
        if capture_pane.escape_sequences.unwrap_or(false) { args.push(e_KEY); }
        if capture_pane.stdout.unwrap_or(false) { args.push(p_KEY); }
        if capture_pane.pane.unwrap_or(false) { args.push(P_KEY); }
        if capture_pane.quite.unwrap_or(false) { args.push(q_KEY); }
        if capture_pane.escape_non_printable.unwrap_or(false) { args.push(C_KEY); }
        if capture_pane.join.unwrap_or(false) { args.push(J_KEY); }
        capture_pane.buffer_name.and_then(|s| Some(args.extend_from_slice(&[b_KEY, &s])));
        capture_pane.end_line.and_then(|s| Some(args.extend_from_slice(&[E_KEY, &s])));
        capture_pane.start_line.and_then(|s| Some(args.extend_from_slice(&[S_KEY, &s])));
        let output = self.subcommand(TmuxInterface::CAPTURE_PANE, &args)?;
        Ok(output.status.success())
    }


    /// Put a pane into client mode, allowing a client to be selected interactively from a list
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn choose_client(&self, choose_client: &ChooseClient) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if choose_client.without_preview.unwrap_or(false) { args.push(N_KEY); }
        if choose_client.zoom.unwrap_or(false) { args.push(Z_KEY); }
        choose_client.format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        choose_client.filter.and_then(|s| Some(args.extend_from_slice(&[f_KEY, &s])));
        choose_client.sort_order.and_then(|s| Some(args.extend_from_slice(&[O_KEY, &s])));
        choose_client.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        choose_client.template.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::CHOOSE_CLIENT, &args)?;
        Ok(output.status.success())
    }


    /// Put a pane into tree mode, where a session, window or pane may be chosen interactively
    /// from a list
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    pub fn choose_tree(&self, choose_tree: &ChooseTree) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if choose_tree.all.unwrap_or(false) { args.push(G_KEY); }
        if choose_tree.without_preview.unwrap_or(false) { args.push(N_KEY); }
        if choose_tree.collapsed_sessions.unwrap_or(false) { args.push(s_KEY); }
        if choose_tree.collapsed_windows.unwrap_or(false) { args.push(w_KEY); }
        if choose_tree.zoom.unwrap_or(false) { args.push(Z_KEY); }
        choose_tree.format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        choose_tree.filter.and_then(|s| Some(args.extend_from_slice(&[f_KEY, &s])));
        choose_tree.sort_order.and_then(|s| Some(args.extend_from_slice(&[O_KEY, &s])));
        choose_tree.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        choose_tree.template.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::CHOOSE_TREE, &args)?;
        Ok(output.status.success())
    }


    /// Display a visible indicator of each pane shown by `target-client`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux display-panes [-b] [-d duration] [-t target-client] [template] (alias: displayp)
    /// ```
    pub fn display_panes(&self,
                         not_block: Option<bool>,
                         duration: Option<&str>,
                         target_client: Option<&str>,
                         template: Option<&str>
                         ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if not_block.unwrap_or(false) { args.push(b_KEY); }
        duration.and_then(|s| Some(args.extend_from_slice(&[d_KEY, &s])));
        target_client.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        template.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::DISPLAY_PANES, &args)?;
        Ok(output.status.success())
    }


    /// Search for the fnmatch(3) pattern `match-string` in window names,
    /// titles, and visible content (but not history)
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux find-window [-CNTZ] [-t target-pane] match-string
    /// (alias: findw)
    /// ```
    pub fn find_window(&self, find_window: &FindWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if find_window.only_visible.unwrap_or(false) { args.push(C_KEY); }
        if find_window.only_name.unwrap_or(false) { args.push(N_KEY); }
        if find_window.only_title.unwrap_or(false) { args.push(T_KEY); }
        if find_window.zoom.unwrap_or(false) { args.push(Z_KEY); }
        find_window.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(find_window.match_string);
        let output = self.subcommand(TmuxInterface::FIND_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    /// and move `src-pane` into the space
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux join-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    pub fn join_pane(&self, join_pane: &JoinPane) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if join_pane.left_above.unwrap_or(false) { args.push(b_KEY); }
        if join_pane.detached.unwrap_or(false) { args.push(d_KEY); }
        if join_pane.horizontal.unwrap_or(false) { args.push(h_KEY); }
        if join_pane.vertical.unwrap_or(false) { args.push(v_KEY); }
        let s;
        if let Some(size) = join_pane.size {
            s = size.to_string();
            args.extend_from_slice(&[l_KEY, &s]);
        }
        let p;
        if let Some(percentage) = join_pane.percentage {
            p = percentage.to_string();
            args.extend_from_slice(&[p_KEY, &p]);
        }
        join_pane.src_pane.and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        join_pane.dst_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::JOIN_PANE, &args)?;
        Ok(output.status.success())
    }


    /// Destroy the given pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-pane [-a] [-t target-pane]
    /// (alias: killp)
    /// ```
    pub fn kill_pane(&self, all: Option<bool>, target_pane: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) { args.push(a_KEY); }
        target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::KILL_PANE, &args)?;
        Ok(output.status.success())
    }


    /// Kill the current window or the window at target-window, removing it from any sessions
    /// to which it is linked
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    pub fn kill_window(&self, all: Option<bool>, target_window: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) { args.push(a_KEY); }
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::KILL_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Select the last (previously selected) pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux last-pane [-de] [-t target-window]
    /// (alias: lastp)
    /// ```
    pub fn last_pane(&self,
                     disable: Option<bool>,
                     enable: Option<bool>,
                     target_window: Option<&str>
                     ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if disable.unwrap_or(false) { args.push(d_KEY); }
        if enable.unwrap_or(false) { args.push(e_KEY); }
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LAST_PANE, &args)?;
        Ok(output.status.success())
    }


    /// Select the last (previously selected) window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux last-window [-t target-session]
    /// (alias: last)
    /// ```
    pub fn last_window(&self, target_session: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LAST_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Link the window at src-window to the specified dst-window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux link-window [-adk] [-s src-window] [-t dst-window]
    /// (alias: linkw)
    /// ```
    pub fn link_window(&self) {
        unimplemented!();
    }


    /// List panes on the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-panes [-as] [-F format] [-t target]
    /// (alias: lsp)
    /// ```
    pub fn list_panes(&self, all: bool, session: bool, format: Option<&str>, target: Option<&str>)
        -> Result<String, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all { args.push(a_KEY); }
        if session { args.push(s_KEY); }
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        target.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LIST_PANES, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }


    /// List windows on the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-windows [-a] [-F format] [-t target-session]
    /// (alias: lsw)
    /// ```
    pub fn list_windows(&self, all: bool, format: Option<&str>, target_session: Option<&str>)
        -> Result<String, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all { args.push(a_KEY); }
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, s])));
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::LIST_WINDOWS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }


    /// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    pub fn move_pane(&self) {
        unimplemented!();
    }


    /// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux move-window [-ardk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    pub fn move_window(&self) {
        unimplemented!();
    }


    /// Create a new window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name] [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    pub fn new_window(&self, new_window: NewWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if new_window.add.unwrap_or(false) { args.push(a_KEY); }
        if new_window.detached.unwrap_or(false) { args.push(d_KEY); }
        if new_window.kill.unwrap_or(false) { args.push(k_KEY); }
        if new_window.print.unwrap_or(false) { args.push(P_KEY); }
        new_window.cwd.as_ref().and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        new_window.window_name.as_ref().and_then(|s| Some(args.extend_from_slice(&[n_KEY, &s])));
        new_window.format.as_ref().and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        new_window.target_window.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        new_window.shell_command.as_ref().and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::NEW_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Move a window to the next layout and rearrange the panes to fit
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux next-layout [-t target-window]
    /// (alias: nextl)
    /// ```
    pub fn next_layout(&self, target_window: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::NEXT_LAYOUT, &args)?;
        Ok(output.status.success())
    }


    /// Move to the next window in the session
    ///
    /// # Manual
    ///
    /// tmux next-window [-a] [-t target-session]
    /// (alias: next)
    pub fn next_window(&self, alert: Option<bool>, target_session: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) { args.push(a_KEY); }
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::NEXT_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Pipe output sent by the program in target-pane to a shell command or vice versa
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    pub fn pipe_pane(&self) {
        unimplemented!();
    }


    /// Move to the previous layout in the session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux previous-layout [-t target-window]
    /// (alias: prevl)
    /// ```
    pub fn previous_layout(&self, target_window: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::PREVIOUS_LAYOUT, &args)?;
        Ok(output.status.success())
    }


    /// Move to the previous window in the session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux previous-window [-a] [-t target-session]
    /// (alias: prev)
    /// ```
    pub fn previous_window(&self, alert: Option<bool>, target_session: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) { args.push(a_KEY); }
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::PREVIOUS_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Rename the current window, or the window at target-window if specified, to new-name
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rename-window [-t target-window] new-name
    /// (alias: renamew)
    /// ```
    pub fn rename_window(&self, target_window: Option<&str>, new_name: &str) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Resize a pane, up, down, left or right
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    /// (alias: resizep)
    /// ```
    pub fn resize_pane(&self) {
        unimplemented!();
    }


    /// Resize a window, up, down, left or right
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    /// (alias: resizew)
    /// ```
    pub fn resize_window(&self) {
        unimplemented!();
    }


    /// Reactivate a pane in which the command has exited
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
    /// (alias: respawnp)
    /// ```
    pub fn respawn_pane(&self) {
        unimplemented!();
    }


    /// Reactivate a window in which the command has exited
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window] [shell-command]
    /// (alias: respawnw)
    /// ```
    pub fn respawn_window(&self) {
        unimplemented!();
    }


    /// Rotate the positions of the panes within a window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rotate-window [-DU] [-t target-window]
    /// (alias: rotatew)
    /// ```
    pub fn rotate_window(&self, down: Option<bool>, up: Option<bool>, target_window: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if down.unwrap_or(false) { args.push(D_KEY); }
        if up.unwrap_or(false) { args.push(U_KEY); }
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::ROTATE_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Choose a specific layout for a window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    pub fn select_layout(&self) {
        unimplemented!();
    }


    /// Make pane `target-pane` the active pane in window `target-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    pub fn select_pane(&self, select_pane: &SelectPane) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if select_pane.down.unwrap_or(false) { args.push(D_KEY); }
        if select_pane.disable.unwrap_or(false) { args.push(d_KEY); }
        if select_pane.enable.unwrap_or(false) { args.push(e_KEY); }
        if select_pane.show_style.unwrap_or(false) { args.push(g_KEY); }
        if select_pane.left.unwrap_or(false) { args.push(L_KEY); }
        if select_pane.last.unwrap_or(false) { args.push(l_KEY); }
        if select_pane.set_marked.unwrap_or(false) { args.push(M_KEY); }
        if select_pane.clear_marked.unwrap_or(false) { args.push(m_KEY); }
        if select_pane.right.unwrap_or(false) { args.push(R_KEY); }
        if select_pane.up.unwrap_or(false) { args.push(U_KEY); }
        select_pane.style.and_then(|s| Some(args.extend_from_slice(&[P_KEY, s])));
        select_pane.title.and_then(|s| Some(args.extend_from_slice(&[T_KEY, s])));
        select_pane.target_pane.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output.status.success())
    }


    /// Select the window at target-window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-window [-lnpT] [-t target-window]
    /// (alias: selectw)
    /// ```
    pub fn select_window(&self, select_window: &SelectWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if select_window.last.unwrap_or(false) { args.push(l_KEY); }
        if select_window.next.unwrap_or(false) { args.push(n_KEY); }
        if select_window.previous.unwrap_or(false) { args.push(p_KEY); }
        if select_window.switch.unwrap_or(false) { args.push(T_KEY); }
        select_window.target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::SELECT_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Create a new pane by splitting `target-pane`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux split-window [-bdfhvP] [-c start-directory] [-l size | -p percentage]
    /// [-t target-pane] [shell-command] [-F format]
    /// (alias: splitw)
    /// ```
    pub fn split_window(&self, split_window: &SplitWindow) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if split_window.before.unwrap_or(false) { args.push(b_KEY); }
        if split_window.detached.unwrap_or(false) { args.push(d_KEY); }
        if split_window.full.unwrap_or(false) { args.push(f_KEY); }
        if split_window.horizontal.unwrap_or(false) { args.push(h_KEY); }
        if split_window.vertical.unwrap_or(false) { args.push(v_KEY); }
        if split_window.print.unwrap_or(false) { args.push(P_KEY); }
        split_window.cwd.as_ref().and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        let s;
        if let Some(size) = split_window.size {
            s = size.to_string();
            args.extend_from_slice(&[l_KEY, &s]);
        }
        let p;
        if let Some(percentage) = split_window.percentage {
            p = percentage.to_string();
            args.extend_from_slice(&[p_KEY, &p]);
        }
        split_window.target_pane.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        split_window.shell_command.as_ref().and_then(|s| Some(args.push(&s)));
        split_window.format.as_ref().and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SPLIT_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Swap two panes
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    pub fn swap_pane(&self) {
        unimplemented!();
    }


    /// This is similar to link-window, except the source and destination windows are swapped
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux swap-window [-d] [-s src-window] [-t dst-window]
    /// (alias: swapw)
    /// ```
    pub fn swap_window(&self,
                       detached: Option<bool>,
                       src_window: Option<&str>,
                       dst_window: Option<&str>
                       ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if detached.unwrap_or(false) { args.push(d_KEY); }
        src_window.and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        dst_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SWAP_WINDOW, &args)?;
        Ok(output.status.success())
    }


    /// Unlink `target-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux unlink-window [-k] [-t target-window]
    /// (alias: unlinkw)
    /// ```
    pub fn unlink_window(&self,
                         k: Option<bool>,
                         target_window: Option<&str>
                         ) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if k.unwrap_or(false) { args.push(k_KEY); }
        target_window.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::UNLINK_WINDOW, &args)?;
        Ok(output.status.success())
    }


}
