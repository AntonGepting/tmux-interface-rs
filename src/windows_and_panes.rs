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
    /// [-d] - the new window does not become the current window
    pub detached: Option<bool>,
    /// [-P] - option prints information about the new window after it has been created
    pub print: Option<bool>,
    /// [-F format] - specify format
    pub format: Option<&'a str>,
    /// [-n] - window-name
    pub window_name: Option<&'a str>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a str>,
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
/// tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line]
/// [-t target-pane]
/// (alias: capturep)
/// ```
#[derive(Default, Debug)]
pub struct CapturePane<'a> {
    /// [-a] - the alternate screen is used, and the history is not accessible
    pub alternate_screen: Option<bool>,
    /// [-e] - the output includes escape sequences for text and background attributes
    pub escape_sequences: Option<bool>,
    /// [-p] - the output goes to stdout
    pub stdout: Option<bool>,
    /// [-P] - capture only any output that the pane has received that is the beginning of an
    /// as-yet incomplete escape sequence
    pub pane: Option<bool>,
    /// [-q] - quite
    pub quite: Option<bool>,
    /// [-C] - escape non-printable characters as octal \xxx
    pub escape_non_printable: Option<bool>,
    /// [-J] - preserve trailing spaces and joins any wrapped lines
    pub join: Option<bool>,
    /// [-N] - preserves trailing spaces at each line's end
    pub trailing_spaces: Option<bool>,
    /// [-b buffer-name] - buffer-name
    pub buffer_name: Option<&'a str>,
    /// [-E end-line] - specify the ending line number
    pub end_line: Option<&'a str>,
    /// [-S start-line] - specify the starting line number
    pub start_line: Option<&'a str>,
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
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseClient<'a> {
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    /// [-r] - reverse the sort order
    pub reverse_sort_order: Option<bool>,
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specify the initial sort field
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [template] - template
    pub template: Option<&'a str>,
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
/// tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseTree<'a> {
    /// [-G] - include all sessions in any session groups in the tree rather than only the first
    pub all: Option<bool>,
    /// [-N] - start without the preview
    pub without_preview: Option<bool>,
    /// [-r] - reverses the sort order
    pub reverse_sort_order: Option<bool>,
    /// [-s] - start with collapsed sessions
    pub collapsed_sessions: Option<bool>,
    /// [-w] - start with collapsed windows
    pub collapsed_windows: Option<bool>,
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

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// ```text
/// tmux find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Default, Debug)]
pub struct FindWindow<'a> {
    /// [-r] - regular expression
    pub regex: Option<bool>,
    /// [-C] - match only visible window contents
    pub only_visible: Option<bool>,
    /// [-N] - match only the window name
    pub only_name: Option<bool>,
    /// [-T] - match only the window title
    pub only_title: Option<bool>,
    /// [-Z] - zoom the pane
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    // match-string
    //pub match_string: &'a str,
}

impl<'a> FindWindow<'a> {
    pub fn new() -> FindWindow<'a> {
        Default::default()
    }
}

#[derive(Debug)]
pub enum PaneSize {
    Size(usize),
    Percentage(usize),
}

/// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
/// and move `src-pane` into the space
///
/// # Manual
///
/// ```text
/// tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: joinp)
/// ```
#[derive(Default, Debug)]
pub struct JoinPane<'a> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window height/width
    pub full_size: Option<bool>,
    /// [-h] - full height
    pub horizontal: Option<bool>,
    /// [-v] - full width
    pub vertical: Option<bool>,
    /// [-l size] - specify the size of the new pane in lines/columns
    pub size: Option<PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
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
    /// [-a] - the window is moved to the next index up
    pub add: Option<bool>,
    /// [-d] - the newly linked window is not selected
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    pub src_window: Option<&'a str>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a str>,
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
/// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
/// (alias: movep)
/// ```
#[derive(Default, Debug)]
pub struct MovePane<'a> {
    /// [-b] - cause src-pane to be joined to left of or above dst-pane
    pub left_above: Option<bool>,
    /// [-d] -
    pub detached: Option<bool>,
    /// [-h] - full height
    pub horizontal: Option<bool>,
    /// [-v] - full width
    pub vertical: Option<bool>,
    /// [-l size] - specify the size of the new pane in lines/columns
    pub size: Option<PaneSize>,
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
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
    /// [-a] - the window is moved to the next index up
    pub add: Option<bool>,
    /// [-r] - all windows in the session are renumbered in sequential order
    pub renumber: Option<bool>,
    /// [-d] - the newly linked window is not selected
    pub detached: Option<bool>,
    /// [-k] - if dst-window exists, it is killed, otherwise an error is generated
    pub kill: Option<bool>,
    /// [-s src-window] - src-window
    pub src_window: Option<&'a str>,
    /// [-t dst-window] - dst-window
    pub dst_window: Option<&'a str>,
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
/// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format]
/// [-n window-name] [-t target-window] [shell-command]
/// (alias: neww)
/// ```
#[derive(Default, Debug)]
pub struct NewWindow<'a> {
    /// [-a] - new window is inserted at the next index up from the specified target-window
    pub add: Option<bool>,
    /// [-d] - the session does not make the new window the current window
    pub detached: Option<bool>,
    /// [-k] - destroy if already exists
    pub kill: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    pub print: Option<bool>,
    /// [-c start-directory] - start-directory
    pub cwd: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-F format] - format
    pub format: Option<&'a str>,
    /// [-n window-name] - window-name
    pub window_name: Option<&'a str>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

impl<'a> NewWindow<'a> {
    pub fn new() -> NewWindow<'a> {
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
    /// [-I] - stdin is connected
    pub stdout: Option<bool>,
    /// [-O] - stdout is connected
    pub stdin: Option<bool>,
    /// [-o] - only open a new pipe if no previous pipe exists
    pub open: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

impl<'a> PipePane<'a> {
    pub fn new() -> PipePane<'a> {
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
    /// [-D] - resize down by adjustment
    pub down: Option<bool>,
    /// [-L] - resize left by adjustment
    pub left: Option<bool>,
    /// [-M] - begin mouse resizing
    pub mouse: Option<bool>,
    /// [-R] - resize right by adjustment
    pub right: Option<bool>,
    /// [-U] - resize up by adjustment
    pub up: Option<bool>,
    /// [-Z] - the active pane is toggled between zoomed and unzoomed
    pub zoom: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [-x width] - absolute size
    pub width: Option<usize>,
    /// [-y height] - absolute size
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    pub adjustment: Option<&'a str>,
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
    /// [-a] - set the size of the smallest session containing the window
    pub smallest: Option<bool>, // [-a]
    /// [-A] - set the size of the largest session containing the window
    pub largest: Option<bool>,
    /// [-D] - resize down by adjustment
    pub down: Option<bool>,
    /// [-L] - resize left by adjustment
    pub left: Option<bool>,
    /// [-R] - resize right by adjustment
    pub right: Option<bool>,
    /// [-U] - resize up by adjustment
    pub up: Option<bool>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a str>,
    /// [-x width] - absolute size
    pub width: Option<usize>,
    /// [-y height] - absolute size
    pub height: Option<usize>,
    /// [adjustment] - adjustment
    pub adjustment: Option<&'a str>,
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
    /// [-k] - any existing command is killed
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
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
    /// [-k] - any existing command is killed
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_window: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
}

impl<'a> RespawnWindow<'a> {
    pub fn new() -> RespawnWindow<'a> {
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
    /// [-E] - spread the current pane and any panes next to it out evenly
    pub spread: Option<bool>,
    /// [-n] - next-layout equivalent
    pub next: Option<bool>,
    /// [-o] - apply the last set layout if possible
    pub last: Option<bool>,
    /// [-p] - previous-layout equivalent
    pub previous: Option<bool>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [layout-name] - layout-name
    pub layout_name: Option<&'a str>,
}

impl<'a> SelectLayot<'a> {
    pub fn new() -> SelectLayot<'a> {
        Default::default()
    }
}

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// ```text
/// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
#[derive(Default, Debug)]
pub struct SelectPane<'a> {
    /// [-D] - pane below
    pub down: Option<bool>,
    /// [-d] - disable input
    pub disable: Option<bool>,
    /// [-e] - enable input
    pub enable: Option<bool>,
    /// [-L] - pane left
    pub left: Option<bool>,
    /// [-l] - equivalent to last-pane command
    pub last: Option<bool>,
    /// [-M] - clear marked pane
    pub set_marked: Option<bool>,
    /// [-m] - set marked pane
    pub clear_marked: Option<bool>,
    /// [-R] - pane right
    pub right: Option<bool>,
    /// [-U] - pane above
    pub up: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    pub keep_zoomed: Option<bool>,
    /// [-T title] - title
    pub title: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
}

impl<'a> SelectPane<'a> {
    pub fn new() -> SelectPane<'a> {
        Default::default()
    }
}

/// Select the window at target-window.
///
/// # Manual
///
/// ```text
/// tmux select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
#[derive(Default, Debug)]
pub struct SelectWindow<'a> {
    /// [-l] - equivalent to last-window
    pub last: Option<bool>,
    /// [-n] - equivalent to next-window
    pub next: Option<bool>,
    /// [-p] - equivalent to previous-window
    pub previous: Option<bool>,
    /// [-T] - if the selected window is already the current window, behave like last-window
    pub switch: Option<bool>,
    /// [-t target-window] - target-window
    pub target_window: Option<&'a str>,
}

impl<'a> SelectWindow<'a> {
    pub fn new() -> SelectWindow<'a> {
        Default::default()
    }
}

/// Create a new pane by splitting target-pane
///
/// # Manual
///
/// ```text
/// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size]
/// [-t target-pane] [shell-command] [-F format]
/// (alias: splitw)
/// ```
#[derive(Default, Debug)]
pub struct SplitWindow<'a> {
    /// [-b] - cause the new pane to be created to the left of or above target-pane
    pub before: Option<bool>,
    /// [-d] - do not make the new window the current window
    pub detached: Option<bool>,
    /// [-f] - creates a new pane spanning the full window size (h, v)
    pub full: Option<bool>,
    /// [-h] - horizontal split
    pub horizontal: Option<bool>,
    /// [-I] - create an empty pane and forward any output from stdin to it
    pub stdin_forward: Option<bool>,
    /// [-v] - vertical split
    pub vertical: Option<bool>,
    /// [-P] - print information about the new window after it has been created
    pub print: Option<bool>,
    /// [-c start_directory] - start-directory
    pub cwd: Option<&'a str>,
    /// [-e environment] - environment
    pub environment: Option<&'a str>,
    /// [-l size] - specify the size of the new pane in lines
    pub size: Option<PaneSize>,
    /// [-t target-pane] -
    pub target_pane: Option<&'a str>,
    /// [shell-command] - shell-command
    pub shell_command: Option<&'a str>,
    /// [-F format] - format
    pub format: Option<&'a str>,
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
/// tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
#[derive(Default, Debug)]
pub struct SwapPane<'a> {
    /// [-d] - instruct tmux not to change the active pane
    pub detached: Option<bool>,
    /// [-D] - swap with the next pane
    pub previous: Option<&'a str>,
    /// [-U] - swap with the previous pane
    pub next: Option<&'a str>, // [-U]
    /// [-Z] - keep the window zoomed if it was zoomed
    pub keep_zoomed: Option<bool>, // [-Z]
    /// [-s src-pane] - src-pane
    pub src_pane: Option<&'a str>,
    /// [-t dst-pane] - dst-pane
    pub dst_pane: Option<&'a str>,
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
        &mut self,
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
    pub fn break_pane(&mut self, break_pane: Option<&BreakPane>) -> Result<Output, Error> {
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
    /// tmux capture-pane [-aepPqCJN] [-b buffer-name] [-E end-line] [-S start-line]
    /// [-t target-pane]
    /// (alias: capturep)
    /// ```
    pub fn capture_pane(&mut self, capture_pane: Option<&CapturePane>) -> Result<Output, Error> {
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
            if capture_pane.trailing_spaces.unwrap_or(false) {
                args.push(N_KEY);
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
    /// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    pub fn choose_client(&mut self, choose_client: Option<&ChooseClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_client) = choose_client {
            if choose_client.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if choose_client.reverse_sort_order.unwrap_or(false) {
                args.push(r_KEY);
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
    /// tmux choose-tree [-GNrswZ] [-F format] [-f filter] [-O sort-order] [-t target-pane]
    /// [template]
    /// ```
    pub fn choose_tree(&mut self, choose_tree: Option<&ChooseTree>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_tree) = choose_tree {
            if choose_tree.all.unwrap_or(false) {
                args.push(G_KEY);
            }
            if choose_tree.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            if choose_tree.reverse_sort_order.unwrap_or(false) {
                args.push(r_KEY);
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
        &mut self,
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
    /// tmux find-window [-rCNTZ] [-t target-pane] match-string
    /// (alias: findw)
    /// ```
    pub fn find_window(
        &mut self,
        find_window: Option<&FindWindow>,
        match_string: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(find_window) = find_window {
            if find_window.regex.unwrap_or(false) {
                args.push(r_KEY);
            }
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
        }
        args.push(match_string);
        let output = self.subcommand(TmuxInterface::FIND_WINDOW, &args)?;
        Ok(output)
    }

    /// Like split-window, but instead of splitting `dst-pane` and creating a new pane, split it
    /// and move `src-pane` into the space
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux join-pane [-bdfhv] [-l size] [-s src-pane] [-t dst-pane]
    /// (alias: joinp)
    /// ```
    pub fn join_pane(&mut self, join_pane: Option<&JoinPane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(join_pane) = join_pane {
            if join_pane.left_above.unwrap_or(false) {
                args.push(b_KEY);
            }
            if join_pane.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if join_pane.full_size.unwrap_or(false) {
                args.push(f_KEY);
            }
            if join_pane.horizontal.unwrap_or(false) {
                args.push(h_KEY);
            }
            if join_pane.vertical.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(size) = &join_pane.size {
                match size {
                    PaneSize::Size(size) => s = size.to_string(),
                    PaneSize::Percentage(size) => s = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &s]);
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
    pub fn kill_pane(
        &mut self,
        all: Option<bool>,
        target_pane: Option<&str>,
    ) -> Result<Output, Error> {
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
        &mut self,
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
    /// tmux last-pane [-deZ] [-t target-window]
    /// (alias: lastp)
    /// ```
    pub fn last_pane(
        &mut self,
        disable: Option<bool>,
        enable: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if disable.unwrap_or(false) {
            args.push(d_KEY);
        }
        if enable.unwrap_or(false) {
            args.push(e_KEY);
        }
        if keep_zoomed.unwrap_or(false) {
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
    pub fn last_window(&mut self, target_session: Option<&str>) -> Result<Output, Error> {
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
    pub fn link_window(&mut self, link_window: Option<&LinkWindow>) -> Result<Output, Error> {
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
        &mut self,
        all: Option<bool>,
        session: Option<bool>,
        format: Option<&str>,
        target: Option<&str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if session.unwrap_or(false) {
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
        &mut self,
        all: Option<bool>,
        format: Option<&str>,
        target_session: Option<&str>,
    ) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
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
    /// tmux move-pane [-bdhv] [-l size] [-s src-pane] [-t dst-pane]
    /// (alias: movep)
    /// ```
    pub fn move_pane(&mut self, move_pane: Option<&MovePane>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(size) = &move_pane.size {
                match size {
                    PaneSize::Size(size) => s = size.to_string(),
                    PaneSize::Percentage(size) => s = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &s]);
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
    pub fn move_window(&mut self, move_window: Option<&MoveWindow>) -> Result<Output, Error> {
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
    /// tmux new-window [-adkP] [-c start-directory] [-e environment] [-F format] [-n window-name]
    /// [-t target-window] [shell-command]
    /// (alias: neww)
    /// ```
    // TODO: target_window exitst error
    pub fn new_window(&mut self, new_window: Option<&NewWindow>) -> Result<String, Error> {
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
            if let Some(s) = new_window.environment {
                args.extend_from_slice(&[e_KEY, &s])
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
    pub fn next_layout(&mut self, target_window: Option<&str>) -> Result<Output, Error> {
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
        &mut self,
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
    pub fn pipe_pane(&mut self, pipe_pane: Option<&PipePane>) -> Result<Output, Error> {
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
    pub fn previous_layout(&mut self, target_window: Option<&str>) -> Result<Output, Error> {
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
        &mut self,
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
        &mut self,
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
    pub fn resize_pane(&mut self, resize_pane: Option<&ResizePane>) -> Result<Output, Error> {
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
    pub fn resize_window(&mut self, resize_window: Option<&ResizeWindow>) -> Result<Output, Error> {
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
    pub fn respawn_pane(&mut self, respawn_pane: Option<&RespawnPane>) -> Result<Output, Error> {
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
    pub fn respawn_window(
        &mut self,
        respawn_window: Option<&RespawnWindow>,
    ) -> Result<Output, Error> {
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
    /// tmux rotate-window [-DUZ] [-t target-window]
    /// (alias: rotatew)
    /// ```
    pub fn rotate_window(
        &mut self,
        down: Option<bool>,
        up: Option<bool>,
        keep_zoomed: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if down.unwrap_or(false) {
            args.push(D_KEY);
        }
        if up.unwrap_or(false) {
            args.push(U_KEY);
        }
        if keep_zoomed.unwrap_or(false) {
            args.push(Z_KEY);
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
    pub fn select_layout(&mut self, select_layout: Option<&SelectLayot>) -> Result<Output, Error> {
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
    /// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
    /// (alias: selectp)
    /// ```
    pub fn select_pane(&mut self, select_pane: Option<&SelectPane>) -> Result<Output, Error> {
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
            if select_pane.keep_zoomed.unwrap_or(false) {
                args.push(U_KEY);
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
    pub fn select_window(&mut self, select_window: Option<&SelectWindow>) -> Result<Output, Error> {
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
    /// tmux split-window [-bdfhIvP] [-c start-directory] [-e environment] [-l size]
    /// [-t target-pane] [shell-command] [-F format]
    /// (alias: splitw)
    /// ```
    pub fn split_window(&mut self, split_window: Option<&SplitWindow>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
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
            if split_window.stdin_forward.unwrap_or(false) {
                args.push(I_KEY);
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
            if let Some(s) = split_window.environment {
                args.extend_from_slice(&[e_KEY, &s]);
            }
            if let Some(size) = &split_window.size {
                match size {
                    PaneSize::Size(size) => s = size.to_string(),
                    PaneSize::Percentage(size) => s = format!("{}%", size),
                };
                args.extend_from_slice(&[l_KEY, &s]);
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
    /// tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
    /// (alias: swapp)
    /// ```
    pub fn swap_pane(&mut self, swap_pane: Option<&SwapPane>) -> Result<Output, Error> {
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
            if swap_pane.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
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
        &mut self,
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
        &mut self,
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
