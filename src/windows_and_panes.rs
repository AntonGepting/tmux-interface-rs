use super::error::Error;
use super::tmux_interface::*;
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
pub struct BreakPane<'a> {
    pub detached: Option<bool>,       // [-d]
    pub print: Option<bool>,          // [-P]
    pub format: Option<&'a str>,      // [-F format]
    pub window_name: Option<&'a str>, // [-n window-name]
    pub src_pane: Option<&'a str>,    // [-s src-pane]
    pub dst_window: Option<&'a str>,  // [-t dst-window]
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
#[derive(Default, Debug)]
pub struct CapturePane<'a> {
    pub alternate_screen: Option<bool>,     // [-a]
    pub escape_sequences: Option<bool>,     // [-e]
    pub stdout: Option<bool>,               // [-p]
    pub pane: Option<bool>,                 // [-P]
    pub quite: Option<bool>,                // [-q]
    pub escape_non_printable: Option<bool>, // [-C]
    pub join: Option<bool>,                 // [-J]
    pub buffer_name: Option<&'a str>,       // [-b buffen_name]
    pub end_line: Option<&'a str>,          // [-E end_line]
    pub start_line: Option<&'a str>,        // [-S start_line]
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
#[derive(Default, Debug)]
pub struct ChooseClient<'a> {
    pub without_preview: Option<bool>, // [-N]
    pub zoom: Option<bool>,            // [-Z]
    pub format: Option<&'a str>,       // [-F format]
    pub filter: Option<&'a str>,       // [-f filter]
    pub sort_order: Option<&'a str>,   // [-O sort-order]
    pub target_pane: Option<&'a str>,  // [-t target-pane]
    pub template: Option<&'a str>,     // [template]
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
#[derive(Default, Debug)]
pub struct ChooseTree<'a> {
    pub all: Option<bool>,                // [-G]
    pub without_preview: Option<bool>,    // [-N]
    pub collapsed_sessions: Option<bool>, // [-s]
    pub collapsed_windows: Option<bool>,  // [-w]
    pub zoom: Option<bool>,               // [-Z]
    pub format: Option<&'a str>,          // [-F format]
    pub filter: Option<&'a str>,          // [-f filter]
    pub sort_order: Option<&'a str>,      // [-O sort-order]
    pub target_pane: Option<&'a str>,     // [-t target-pane]
    pub template: Option<&'a str>,        // [template]
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
#[derive(Default, Debug)]
pub struct FindWindow<'a> {
    pub only_visible: Option<bool>,   // [-C]
    pub only_name: Option<bool>,      // [-N]
    pub only_title: Option<bool>,     // [-T]
    pub zoom: Option<bool>,           // [-Z]
    pub target_pane: Option<&'a str>, // [-t target-pane]
    pub match_string: &'a str,        // match-string
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
#[derive(Default, Debug)]
pub struct JoinPane<'a> {
    pub left_above: Option<bool>,  // [-b]
    pub detached: Option<bool>,    // [-d]
    pub horizontal: Option<bool>,  // [-h]
    pub vertical: Option<bool>,    // [-v]
    pub size: Option<usize>,       // [-l size]
    pub percentage: Option<usize>, // [-p percentage]
    pub src_pane: Option<&'a str>, // [-s src-pane]
    pub dst_pane: Option<&'a str>, // [-t dst-pane]
}

impl<'a> JoinPane<'a> {
    pub fn new() -> JoinPane<'a> {
        Default::default()
    }
}

/// Link the window at src-window to the specified dst-window
///
/// # Manual
///
/// ```text
/// tmux link-window [-adk] [-s src-window] [-t dst-window]
/// (alias: linkw)
/// ```
#[derive(Default, Debug)]
pub struct LinkWindow<'a> {
    pub add: Option<bool>,           // [-a]
    pub detached: Option<bool>,      // [-d]
    pub kill: Option<bool>,          // [-k]
    pub src_window: Option<&'a str>, // [-t target-window]
    pub dst_window: Option<&'a str>, // [shell-command]
}

impl<'a> LinkWindow<'a> {
    pub fn new() -> LinkWindow<'a> {
        Default::default()
    }
}

/// Like join-pane, but `src-pane` and `dst-pane` may belong to the same window
///
/// # Manual
///
/// ```text
/// tmux move-pane [-bdhv] [-l size | -p percentage] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[derive(Default, Debug)]
pub struct MovePane<'a> {
    pub left_above: Option<bool>,  // [-b]
    pub detached: Option<bool>,    // [-d]
    pub horizontal: Option<bool>,  // [-h]
    pub vertical: Option<bool>,    // [-v]
    pub size: Option<usize>,       // [-l size]
    pub percentage: Option<usize>, // [-p percentage]
    pub src_pane: Option<&'a str>, // [-s src-pane]
    pub dst_pane: Option<&'a str>, // [-t dst-pane]
}

impl<'a> MovePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
///
/// # Manual
///
/// ```text
/// tmux move-window [-ardk] [-s src-window] [-t dst-window]
/// (alias: movew)
/// ```
#[derive(Default, Debug)]
pub struct MoveWindow<'a> {
    pub add: Option<bool>,           // [-a]
    pub renumber: Option<bool>,      // [-r]
    pub detached: Option<bool>,      // [-d]
    pub kill: Option<bool>,          // [-k]
    pub src_window: Option<&'a str>, // [-s src-window]
    pub dst_window: Option<&'a str>, // [-t dst-window]
}

impl<'a> MoveWindow<'a> {
    pub fn new() -> MoveWindow<'a> {
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
#[derive(Default, Debug)]
pub struct NewWindow<'a> {
    pub add: Option<bool>,              // [-a]
    pub detached: Option<bool>,         // [-d]
    pub kill: Option<bool>,             // [-k]
    pub print: Option<bool>,            // [-P]
    pub cwd: Option<&'a str>,           // [-c start-directory]
    pub format: Option<&'a str>,        // [-F format]
    pub window_name: Option<&'a str>,   // [-n window-name]
    pub target_window: Option<&'a str>, // [-t target-window]
    pub shell_command: Option<&'a str>, // [shell-command]
}

impl<'a> NewWindow<'a> {
    pub fn new() -> NewWindow<'a> {
        Default::default()
    }
}

/// Resize a pane, up, down, left or right
///
/// # Manual
///
/// ```text
/// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
/// (alias: resizep)
/// ```
#[derive(Default, Debug)]
pub struct ResizePane<'a> {
    pub down: Option<bool>,           // [-D]
    pub left: Option<bool>,           // [-L]
    pub mouse: Option<bool>,          // [-M]
    pub right: Option<bool>,          // [-R]
    pub up: Option<bool>,             // [-U]
    pub zoom: Option<bool>,           // [-Z]
    pub target_pane: Option<&'a str>, // [-t target-pane]
    pub width: Option<usize>,         // [-x width]
    pub height: Option<usize>,        // [-y height]
    pub adjustment: Option<&'a str>,  // [adjustment]
}

impl<'a> ResizePane<'a> {
    pub fn new() -> ResizePane<'a> {
        Default::default()
    }
}

/// Resize a window, up, down, left or right
///
/// # Manual
///
/// ```text
/// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
/// (alias: resizew)
/// ```
#[derive(Default, Debug)]
pub struct ResizeWindow<'a> {
    pub smallest: Option<bool>,         // [-a]
    pub largest: Option<bool>,          // [-A]
    pub down: Option<bool>,             // [-D]
    pub left: Option<bool>,             // [-L]
    pub right: Option<bool>,            // [-R]
    pub up: Option<bool>,               // [-U]
    pub target_window: Option<&'a str>, // [-t target-window]
    pub width: Option<usize>,           // [-x width]
    pub height: Option<usize>,          // [-y height]
    pub adjustment: Option<&'a str>,    // [adjustment]
}

impl<'a> ResizeWindow<'a> {
    pub fn new() -> ResizeWindow<'a> {
        Default::default()
    }
}

/// Reactivate a pane in which the command has exited
///
/// # Manual
///
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
#[derive(Default, Debug)]
pub struct RespawnPane<'a> {
    pub kill: Option<bool>,               // [-k]
    pub start_directory: Option<&'a str>, // [-c start-directory]
    pub environment: Option<&'a str>,     // [-e environment]
    pub target_pane: Option<&'a str>,     // [-t target-pane]
    pub shell_command: Option<&'a str>,   // [shell-command]
}

impl<'a> RespawnPane<'a> {
    pub fn new() -> RespawnPane<'a> {
        Default::default()
    }
}

/// Reactivate a window in which the command has exited
///
/// # Manual
///
/// ```text
/// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
/// [shell-command]
/// (alias: respawnw)
/// ```
#[derive(Default, Debug)]
pub struct RespawnWindow<'a> {
    pub kill: Option<bool>,               // [-k]
    pub start_directory: Option<&'a str>, // [-c start-directory]
    pub environment: Option<&'a str>,     // [-e environment]
    pub target_window: Option<&'a str>,   // [-t target-pane]
    pub shell_command: Option<&'a str>,   // [shell-command]
}

impl<'a> RespawnWindow<'a> {
    pub fn new() -> RespawnWindow<'a> {
        Default::default()
    }
}

/// Pipe output sent by the program in target-pane to a shell command or vice versa
///
/// # Manual
///
/// ```text
/// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
#[derive(Default, Debug)]
pub struct PipePane<'a> {
    pub stdout: Option<bool>,           // [-I]
    pub stdin: Option<bool>,            // [-O]
    pub open: Option<bool>,             // [-o]
    pub target_pane: Option<&'a str>,   // [-t target-pane]
    pub shell_command: Option<&'a str>, // [shell-command]
}

impl<'a> PipePane<'a> {
    pub fn new() -> PipePane<'a> {
        Default::default()
    }
}

/// Choose a specific layout for a window
///
/// # Manual
///
/// ```text
/// tmux select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
#[derive(Default, Debug)]
pub struct SelectLayot<'a> {
    pub spread: Option<bool>,         // [-E]
    pub next: Option<bool>,           // [-n]
    pub last: Option<bool>,           // [-o]
    pub previous: Option<bool>,       // [-p]
    pub target_pane: Option<&'a str>, // [-t target-pane]
    pub layout_name: Option<&'a str>, // [layout-name]
}

impl<'a> SelectLayot<'a> {
    pub fn new() -> SelectLayot<'a> {
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
#[derive(Default, Debug)]
pub struct SelectPane<'a> {
    pub down: Option<bool>,           // [-D]
    pub disable: Option<bool>,        // [-d]
    pub enable: Option<bool>,         // [-e]
    pub show_style: Option<bool>,     // [-g]
    pub left: Option<bool>,           // [-L]
    pub last: Option<bool>,           // [-l]
    pub set_marked: Option<bool>,     // [-M]
    pub clear_marked: Option<bool>,   // [-m]
    pub right: Option<bool>,          // [-R]
    pub up: Option<bool>,             // [-U]
    pub style: Option<&'a str>,       // [-P style]
    pub title: Option<&'a str>,       // [-T title]
    pub target_pane: Option<&'a str>, // [-t target-pane]
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
#[derive(Default, Debug)]
pub struct SelectWindow<'a> {
    pub last: Option<bool>,             // [-l]
    pub next: Option<bool>,             // [-n]
    pub previous: Option<bool>,         // [-p]
    pub switch: Option<bool>,           // [-T]
    pub target_window: Option<&'a str>, // [-t target-window]
}

impl<'a> SelectWindow<'a> {
    pub fn new() -> SelectWindow<'a> {
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
#[derive(Default, Debug)]
pub struct SplitWindow<'a> {
    pub before: Option<bool>,           // [-b]
    pub detached: Option<bool>,         // [-d]
    pub full: Option<bool>,             // [-f]
    pub horizontal: Option<bool>,       // [-h]
    pub vertical: Option<bool>,         // [-v]
    pub print: Option<bool>,            // [-P]
    pub cwd: Option<&'a str>,           // [-c start-directory]
    pub size: Option<usize>,            // [-l size]
    pub percentage: Option<usize>,      // [-p percentage]
    pub target_pane: Option<&'a str>,   // [-t target-pane]
    pub shell_command: Option<&'a str>, // [shell-command]
    pub format: Option<&'a str>,        // [-F format]
}

impl<'a> SplitWindow<'a> {
    pub fn new() -> SplitWindow<'a> {
        Default::default()
    }
}

/// Swap two panes
///
/// # Manual
///
/// ```text
/// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
#[derive(Default, Debug)]
pub struct SwapPane<'a> {
    pub detached: Option<bool>,    // [-d]
    pub previous: Option<&'a str>, // [-D]
    pub next: Option<&'a str>,     // [-U]
    pub src_pane: Option<&'a str>, // [-s src-pane]
    pub dst_pane: Option<&'a str>, // [-t dst-pane]
}

impl<'a> SwapPane<'a> {
    pub fn new() -> SwapPane<'a> {
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
    const LINK_WINDOW: &'static str = "link-window";
    const LIST_PANES: &'static str = "list-panes";
    const LIST_WINDOWS: &'static str = "list-windows";
    const MOVE_PANE: &'static str = "move-pane";
    const MOVE_WINDOW: &'static str = "move-window";
    const NEW_WINDOW: &'static str = "new-window";
    const NEXT_LAYOUT: &'static str = "next-layout";
    const NEXT_WINDOW: &'static str = "next-window";
    const PIPE_PANE: &'static str = "pipe-pane";
    const PREVIOUS_LAYOUT: &'static str = "previous-layout";
    const PREVIOUS_WINDOW: &'static str = "previous-window";
    const RENAME_WINDOW: &'static str = "rename-window";
    const RESIZE_PANE: &'static str = "resize-pane";
    const RESIZE_WINDOW: &'static str = "resize-window";
    const RESPAWN_PANE: &'static str = "respawn-pane";
    const RESPAWN_WINDOW: &'static str = "respawn-window";
    const ROTATE_WINDOW: &'static str = "rotate-window";
    const SELECT_LAYOUT: &'static str = "select-layout";
    const SELECT_PANE: &'static str = "select-pane";
    const SELECT_WINDOW: &'static str = "select-window";
    const SPLIT_WINDOW: &'static str = "split-window";
    const SWAP_PANE: &'static str = "swap-pane";
    const SWAP_WINDOW: &'static str = "swap-window";
    const UNLINK_WINDOW: &'static str = "unlink-window";

    /// Enter copy mode
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux copy-mode [-Meu] [-t target-pane]
    /// ```
    pub fn copy_mode(
        &self,
        mouse_drag: Option<bool>,
        bottom_exit: Option<bool>,
        page_up: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if mouse_drag.unwrap_or(false) {
            args.push(M_KEY);
        }
        if bottom_exit.unwrap_or(false) {
            args.push(e_KEY);
        }
        if page_up.unwrap_or(false) {
            args.push(u_KEY);
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::COPY_MODE, &args)?;
        Ok(output)
    }

    /// Break `src-pane` off from its containing window to make it the only pane in `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux break-pane [-dP] [-F format] [-n window-name] [-s src-pane] [-t dst-window]
    /// (alias: breakp)
    /// ```
    pub fn break_pane(&self, break_pane: Option<&BreakPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if let Some(s) = break_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = break_pane.dst_window {
                args.extend_from_slice(&[t_KEY, &s]);
            }
        }
        let output = self.subcommand(TmuxInterface::BREAK_PANE, &args)?;
        Ok(output)
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
    pub fn capture_pane(&self, capture_pane: Option<&CapturePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(capture_pane) = capture_pane {
            if capture_pane.alternate_screen.unwrap_or(false) {
                args.push(a_KEY);
            }
            if capture_pane.escape_sequences.unwrap_or(false) {
                args.push(e_KEY);
            }
            if capture_pane.stdout.unwrap_or(false) {
                args.push(p_KEY);
            }
            if capture_pane.pane.unwrap_or(false) {
                args.push(P_KEY);
            }
            if capture_pane.quite.unwrap_or(false) {
                args.push(q_KEY);
            }
            if capture_pane.escape_non_printable.unwrap_or(false) {
                args.push(C_KEY);
            }
            if capture_pane.join.unwrap_or(false) {
                args.push(J_KEY);
            }
            if let Some(s) = capture_pane.buffer_name {
                args.extend_from_slice(&[b_KEY, &s])
            }
            if let Some(s) = capture_pane.end_line {
                args.extend_from_slice(&[E_KEY, &s])
            }
            if let Some(s) = capture_pane.start_line {
                args.extend_from_slice(&[S_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::CAPTURE_PANE, &args)?;
        Ok(output)
    }

    /// Put a pane into client mode, allowing a client to be selected interactively from a list
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    pub fn choose_client(&self, choose_client: Option<&ChooseClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_client) = choose_client {
            if choose_client.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if choose_client.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(s) = choose_client.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = choose_client.filter {
                args.extend_from_slice(&[f_KEY, &s])
            }
            if let Some(s) = choose_client.sort_order {
                args.extend_from_slice(&[O_KEY, &s])
            }
            if let Some(s) = choose_client.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = choose_client.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::CHOOSE_CLIENT, &args)?;
        Ok(output)
    }

    /// Put a pane into tree mode, where a session, window or pane may be chosen interactively
    /// from a list
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux choose-tree [-GNswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    pub fn choose_tree(&self, choose_tree: Option<&ChooseTree>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_tree) = choose_tree {
            if choose_tree.all.unwrap_or(false) {
                args.push(G_KEY);
            }
            if choose_tree.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if choose_tree.collapsed_sessions.unwrap_or(false) {
                args.push(s_KEY);
            }
            if choose_tree.collapsed_windows.unwrap_or(false) {
                args.push(w_KEY);
            }
            if choose_tree.zoom.unwrap_or(false) {
                args.push(Z_KEY);
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

    /// Display a visible indicator of each pane shown by `target-client`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux display-panes [-b] [-d duration] [-t target-client] [template] (alias: displayp)
    /// ```
    pub fn display_panes(
        &self,
        not_block: Option<bool>,
        duration: Option<&str>,
        target_client: Option<&str>,
        template: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if not_block.unwrap_or(false) {
            args.push(b_KEY);
        }
        if let Some(s) = duration {
            args.extend_from_slice(&[d_KEY, &s])
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        if let Some(s) = template {
            args.push(&s)
        }
        let output = self.subcommand(TmuxInterface::DISPLAY_PANES, &args)?;
        Ok(output)
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
    pub fn find_window(&self, find_window: &FindWindow) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if find_window.only_visible.unwrap_or(false) {
            args.push(C_KEY);
        }
        if find_window.only_name.unwrap_or(false) {
            args.push(N_KEY);
        }
        if find_window.only_title.unwrap_or(false) {
            args.push(T_KEY);
        }
        if find_window.zoom.unwrap_or(false) {
            args.push(Z_KEY);
        }
        if let Some(s) = find_window.target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(find_window.match_string);
        let output = self.subcommand(TmuxInterface::FIND_WINDOW, &args)?;
        Ok(output)
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
    pub fn join_pane(&self, join_pane: Option<&JoinPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let p;
        if let Some(join_pane) = join_pane {
            if join_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            if join_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if join_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if join_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(size) = join_pane.size {
                s = size.to_string();
                args.extend_from_slice(&[l_KEY, &s]);
            }
            if let Some(percentage) = join_pane.percentage {
                p = percentage.to_string();
                args.extend_from_slice(&[p_KEY, &p]);
            }
            if let Some(s) = join_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = join_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::JOIN_PANE, &args)?;
        Ok(output)
    }

    /// Destroy the given pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-pane [-a] [-t target-pane]
    /// (alias: killp)
    /// ```
    pub fn kill_pane(&self, all: Option<bool>, target_pane: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_pane {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_PANE, &args)?;
        Ok(output)
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
    pub fn kill_window(
        &self,
        all: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_WINDOW, &args)?;
        Ok(output)
    }

    /// Select the last (previously selected) pane
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux last-pane [-de] [-t target-window]
    /// (alias: lastp)
    /// ```
    pub fn last_pane(
        &self,
        disable: Option<bool>,
        enable: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if disable.unwrap_or(false) {
            args.push(d_KEY);
        }
        if enable.unwrap_or(false) {
            args.push(e_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LAST_PANE, &args)?;
        Ok(output)
    }

    /// Select the last (previously selected) window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux last-window [-t target-session]
    /// (alias: last)
    /// ```
    pub fn last_window(&self, target_session: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LAST_WINDOW, &args)?;
        Ok(output)
    }

    /// Link the window at src-window to the specified dst-window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux link-window [-adk] [-s src-window] [-t dst-window]
    /// (alias: linkw)
    /// ```
    pub fn link_window(&self, link_window: Option<&LinkWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(link_window) = link_window {
            if link_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if link_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if link_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = link_window.src_window {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = link_window.dst_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::LINK_WINDOW, &args)?;
        Ok(output)
    }

    // XXX: better return type
    /// List panes on the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-panes [-as] [-F format] [-t target]
    /// (alias: lsp)
    /// ```
    pub fn list_panes(
        &self,
        all: bool,
        session: bool,
        format: Option<&str>,
        target: Option<&str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all {
            args.push(a_KEY);
        }
        if session {
            args.push(s_KEY);
        }
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        if let Some(s) = target {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_PANES, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }

    // XXX: better return type
    /// List windows on the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-windows [-a] [-F format] [-t target-session]
    /// (alias: lsw)
    /// ```
    pub fn list_windows(
        &self,
        all: bool,
        format: Option<&str>,
        target_session: Option<&str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all {
            args.push(a_KEY);
        }
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, s])
        }
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
    pub fn move_pane(&self, move_pane: Option<&MovePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let p;
        if let Some(move_pane) = move_pane {
            if move_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            if move_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if move_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if move_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(size) = move_pane.size {
                s = size.to_string();
                args.extend_from_slice(&[l_KEY, &s]);
            }
            if let Some(percentage) = move_pane.percentage {
                p = percentage.to_string();
                args.extend_from_slice(&[p_KEY, &p]);
            }
            if let Some(s) = move_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = move_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_PANE, &args)?;
        Ok(output)
    }

    /// This is similar to link-window, except the window at `src-window` is moved to `dst-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux move-window [-ardk] [-s src-window] [-t dst-window]
    /// (alias: movew)
    /// ```
    pub fn move_window(&self, move_window: Option<&MoveWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if let Some(s) = move_window.src_window {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = move_window.dst_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::MOVE_WINDOW, &args)?;
        Ok(output)
    }

    /// Create a new window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux new-window [-adkP] [-c start-directory] [-F format] [-n window-name]
    /// [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    // TODO: target_window exitst error
    pub fn new_window(&self, new_window: Option<&NewWindow>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(new_window) = new_window {
            if new_window.add.unwrap_or(false) {
                args.push(a_KEY);
            }
            if new_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if new_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if new_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = new_window.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = new_window.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(s) = new_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_window.target_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = new_window.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::NEW_WINDOW, &args)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
            Ok(stdout.to_string())
        } else {
            let stdout = String::from_utf8_lossy(&output.stderr.as_slice());
            Err(Error::new(&stdout))
        }
    }

    /// Move a window to the next layout and rearrange the panes to fit
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux next-layout [-t target-window]
    /// (alias: nextl)
    /// ```
    pub fn next_layout(&self, target_window: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::NEXT_LAYOUT, &args)?;
        Ok(output)
    }

    /// Move to the next window in the session
    ///
    /// # Manual
    ///
    /// tmux next-window [-a] [-t target-session]
    /// (alias: next)
    pub fn next_window(
        &self,
        alert: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::NEXT_WINDOW, &args)?;
        Ok(output)
    }

    /// Pipe output sent by the program in target-pane to a shell command or vice versa
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
    /// (alias: pipep)
    /// ```
    pub fn pipe_pane(&self, pipe_pane: Option<&PipePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(pipe_pane) = pipe_pane {
            if pipe_pane.stdout.unwrap_or(false) {
                args.push(I_KEY);
            }
            if pipe_pane.stdin.unwrap_or(false) {
                args.push(O_KEY);
            }
            if pipe_pane.open.unwrap_or(false) {
                args.push(o_KEY);
            }
            if let Some(s) = pipe_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = pipe_pane.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::PIPE_PANE, &args)?;
        Ok(output)
    }

    /// Move to the previous layout in the session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux previous-layout [-t target-window]
    /// (alias: prevl)
    /// ```
    pub fn previous_layout(&self, target_window: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::PREVIOUS_LAYOUT, &args)?;
        Ok(output)
    }

    /// Move to the previous window in the session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux previous-window [-a] [-t target-session]
    /// (alias: prev)
    /// ```
    pub fn previous_window(
        &self,
        alert: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if alert.unwrap_or(false) {
            args.push(a_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::PREVIOUS_WINDOW, &args)?;
        Ok(output)
    }

    /// Rename the current window, or the window at target-window if specified, to new-name
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rename-window [-t target-window] new-name
    /// (alias: renamew)
    /// ```
    pub fn rename_window(
        &self,
        target_window: Option<&str>,
        new_name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_WINDOW, &args)?;
        Ok(output)
    }

    /// Resize a pane, up, down, left or right
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux resize-pane [-DLMRUZ] [-t target-pane] [-x width] [-y height] [adjustment]
    /// (alias: resizep)
    /// ```
    pub fn resize_pane(&self, resize_pane: Option<&ResizePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(resize_pane) = resize_pane {
            if resize_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if resize_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if resize_pane.mouse.unwrap_or(false) {
                args.push(M_KEY);
            }
            if resize_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if resize_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if resize_pane.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(s) = resize_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(width) = resize_pane.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = resize_pane.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(s) = resize_pane.adjustment {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESIZE_PANE, &args)?;
        Ok(output)
    }

    /// Resize a window, up, down, left or right
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux resize-window [-aADLRU] [-t target-window] [-x width] [-y height] [adjustment]
    /// (alias: resizew)
    /// ```
    pub fn resize_window(&self, resize_window: Option<&ResizeWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(resize_window) = resize_window {
            if resize_window.smallest.unwrap_or(false) {
                args.push(a_KEY);
            }
            if resize_window.largest.unwrap_or(false) {
                args.push(A_KEY);
            }
            if resize_window.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if resize_window.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if resize_window.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if resize_window.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(s) = resize_window.target_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(width) = resize_window.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = resize_window.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(s) = resize_window.adjustment {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESIZE_WINDOW, &args)?;
        Ok(output)
    }

    /// Reactivate a pane in which the command has exited
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane]
    /// [shell-command]
    /// (alias: respawnp)
    /// ```
    pub fn respawn_pane(&self, respawn_pane: Option<&RespawnPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(respawn_pane) = respawn_pane {
            if respawn_pane.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = respawn_pane.start_directory {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = respawn_pane.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            if let Some(s) = respawn_pane.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = respawn_pane.shell_command {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_PANE, &args)?;
        Ok(output)
    }

    /// Reactivate a window in which the command has exited
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
    /// [shell-command]
    /// (alias: respawnw)
    /// ```
    pub fn respawn_window(&self, respawn_window: Option<&RespawnWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(respawn_window) = respawn_window {
            if respawn_window.kill.unwrap_or(false) {
                args.push(k_KEY);
            }
            if let Some(s) = respawn_window.start_directory {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = respawn_window.environment {
                args.extend_from_slice(&[e_KEY, &s])
            }
            if let Some(s) = respawn_window.target_window {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = respawn_window.shell_command {
                args.push(s)
            }
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_WINDOW, &args)?;
        Ok(output)
    }

    /// Rotate the positions of the panes within a window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rotate-window [-DU] [-t target-window]
    /// (alias: rotatew)
    /// ```
    pub fn rotate_window(
        &self,
        down: Option<bool>,
        up: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if down.unwrap_or(false) {
            args.push(D_KEY);
        }
        if up.unwrap_or(false) {
            args.push(U_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::ROTATE_WINDOW, &args)?;
        Ok(output)
    }

    /// Choose a specific layout for a window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-layout [-Enop] [-t target-pane] [layout-name]
    /// (alias: selectl)
    /// ```
    pub fn select_layout(&self, select_layout: Option<&SelectLayot>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_layout) = select_layout {
            if select_layout.spread.unwrap_or(false) {
                args.push(E_KEY);
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
            if let Some(s) = select_layout.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = select_layout.layout_name {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_LAYOUT, &args)?;
        Ok(output)
    }

    /// Make pane `target-pane` the active pane in window `target-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    pub fn select_pane(&self, select_pane: Option<&SelectPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_pane) = select_pane {
            if select_pane.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if select_pane.disable.unwrap_or(false) {
                args.push(d_KEY);
            }
            if select_pane.enable.unwrap_or(false) {
                args.push(e_KEY);
            }
            if select_pane.show_style.unwrap_or(false) {
                args.push(g_KEY);
            }
            if select_pane.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if select_pane.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            if select_pane.set_marked.unwrap_or(false) {
                args.push(M_KEY);
            }
            if select_pane.clear_marked.unwrap_or(false) {
                args.push(m_KEY);
            }
            if select_pane.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if select_pane.up.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(s) = select_pane.style {
                args.extend_from_slice(&[P_KEY, s])
            }
            if let Some(s) = select_pane.title {
                args.extend_from_slice(&[T_KEY, s])
            }
            if let Some(s) = select_pane.target_pane {
                args.extend_from_slice(&[t_KEY, s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_PANE, &args)?;
        Ok(output)
    }

    /// Select the window at target-window
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux select-window [-lnpT] [-t target-window]
    /// (alias: selectw)
    /// ```
    pub fn select_window(&self, select_window: Option<&SelectWindow>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(select_window) = select_window {
            if select_window.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            if select_window.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if select_window.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if select_window.switch.unwrap_or(false) {
                args.push(T_KEY);
            }
            if let Some(s) = select_window.target_window {
                args.extend_from_slice(&[t_KEY, s])
            }
        }
        let output = self.subcommand(TmuxInterface::SELECT_WINDOW, &args)?;
        Ok(output)
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
    pub fn split_window(&self, split_window: Option<&SplitWindow>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let p;
        let s;
        if let Some(split_window) = split_window {
            if split_window.before.unwrap_or(false) {
                args.push(b_KEY);
            }
            if split_window.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if split_window.full.unwrap_or(false) {
                args.push(f_KEY);
            }
            if split_window.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if split_window.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if split_window.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = split_window.cwd {
                args.extend_from_slice(&[c_KEY, &s]);
            }
            if let Some(size) = split_window.size {
                s = size.to_string();
                args.extend_from_slice(&[l_KEY, &s]);
            }
            if let Some(percentage) = split_window.percentage {
                p = percentage.to_string();
                args.extend_from_slice(&[p_KEY, &p]);
            }
            if let Some(s) = split_window.shell_command {
                args.push(&s)
            }
            if let Some(s) = split_window.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = split_window.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SPLIT_WINDOW, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }

    /// Swap two panes
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    pub fn swap_pane(&self, swap_pane: Option<&SwapPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(swap_pane) = swap_pane {
            if swap_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if swap_pane.detached.unwrap_or(false) {
                args.push(D_KEY);
            }
            if swap_pane.detached.unwrap_or(false) {
                args.push(U_KEY);
            }
            if let Some(s) = swap_pane.src_pane {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = swap_pane.dst_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SWAP_PANE, &args)?;
        Ok(output)
    }

    /// This is similar to link-window, except the source and destination windows are swapped
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux swap-window [-d] [-s src-window] [-t dst-window]
    /// (alias: swapw)
    /// ```
    pub fn swap_window(
        &self,
        detached: Option<bool>,
        src_window: Option<&str>,
        dst_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if detached.unwrap_or(false) {
            args.push(d_KEY);
        }
        if let Some(s) = src_window {
            args.extend_from_slice(&[s_KEY, &s])
        }
        if let Some(s) = dst_window {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SWAP_WINDOW, &args)?;
        Ok(output)
    }

    /// Unlink `target-window`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux unlink-window [-k] [-t target-window]
    /// (alias: unlinkw)
    /// ```
    pub fn unlink_window(
        &self,
        k: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if k.unwrap_or(false) {
            args.push(k_KEY);
        }
        if let Some(s) = target_window {
            args.extend_from_slice(&[t_KEY, &s]);
        }
        let output = self.subcommand(TmuxInterface::UNLINK_WINDOW, &args)?;
        Ok(output)
    }
}
