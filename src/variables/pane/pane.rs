use crate::Error;
use crate::FormatsOutput;
#[cfg(feature = "tmux_1_8")]
use crate::PaneTabs;
use std::str::FromStr;

pub const PANE_VARS_SEPARATOR: char = '\'';

//pub fn get_fmt_string(bitflags: usize) -> String {
//let lsp_format = PANE_VARS_REGEX_VEC
//.iter()
//.filter(|t| bitflags & t.1 == t.1)
//.map(|t| format!("#{{{}}}", t.0))
//.collect::<Vec<String>>()
//.join(PANE_VARS_SEPARATOR);
//lsp_format
//}

// XXX: mb macro with custom struct fields
// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Pane {
    /// pane_active - 1 if active pane
    #[cfg(feature = "tmux_1_6")]
    pub active: Option<bool>,
    /// 1 if pane is at the bottom of window
    #[cfg(feature = "tmux_2_6")]
    pub at_bottom: Option<bool>,
    /// 1 if pane is at the left of window
    #[cfg(feature = "tmux_2_6")]
    pub at_left: Option<bool>,
    /// 1 if pane is at the right of window
    #[cfg(feature = "tmux_2_6")]
    pub at_right: Option<bool>,
    /// 1 if pane is at the top of window
    #[cfg(feature = "tmux_2_6")]
    pub at_top: Option<bool>,
    /// pane_bottom - Bottom of pane
    #[cfg(feature = "tmux_2_0")]
    pub bottom: Option<usize>,
    /// pane_current_command - Current command if available
    #[cfg(feature = "tmux_1_8")]
    pub current_command: Option<String>,
    /// pane_current_path - Current path if available
    #[cfg(feature = "tmux_1_7")]
    pub current_path: Option<String>,
    /// pane_dead - 1 if pane is dead
    #[cfg(feature = "tmux_1_6")]
    pub dead: Option<bool>,
    /// pane_dead_status - Exit status of process in dead pane
    #[cfg(feature = "tmux_2_0")]
    pub dead_status: Option<usize>,
    /// 1 if format is for a pane
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// pane_height - Height of pane
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    /// pane_id - #D Unique pane ID
    #[cfg(feature = "tmux_1_6")]
    pub id: Option<usize>,
    /// pane_in_mode - 1 if pane is in a mode
    #[cfg(feature = "tmux_1_8")]
    pub in_mode: Option<bool>,
    /// pane_index - #P Index of pane
    #[cfg(feature = "tmux_1_7")]
    pub index: Option<usize>,
    /// pane_input_off - 1 if input to pane is disabled
    #[cfg(feature = "tmux_2_0")]
    pub input_off: Option<bool>,
    /// pane_left - Left of pane
    #[cfg(feature = "tmux_2_0")]
    pub left: Option<usize>,
    /// 1 if this is the marked pane
    #[cfg(feature = "tmux_3_0")]
    pub marked: Option<bool>,
    /// 1 if a marked pane is set
    #[cfg(feature = "tmux_3_0")]
    pub marked_set: Option<bool>,
    /// Name of pane mode, if any
    #[cfg(feature = "tmux_2_5")]
    pub mode: Option<usize>,
    /// pane_path - #T Path of pane (can be set by application)
    #[cfg(feature = "tmux_3_1")]
    pub path: Option<String>,
    /// pane_pid - PID of first process in pane
    #[cfg(feature = "tmux_1_6")]
    pub pid: Option<usize>,
    /// pane_pipe - 1 if pane is being piped
    #[cfg(feature = "tmux_2_6")]
    pub pipe: Option<bool>,
    /// pane_right - Right of pane
    #[cfg(feature = "tmux_2_0")]
    pub right: Option<usize>,
    /// Last search string in copy mode
    #[cfg(feature = "tmux_2_5")]
    pub search_string: Option<usize>,
    /// pane_start_command - Command pane started with
    #[cfg(feature = "tmux_1_6")]
    pub start_command: Option<usize>,
    /// pane_start_path - Path pane started with
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
    pub start_path: Option<usize>,
    /// pane_synchronized - 1 if pane is synchronized
    #[cfg(feature = "tmux_1_9")]
    pub synchronized: Option<bool>,
    /// pane_tabs - Pane tab positions
    #[cfg(feature = "tmux_1_8")]
    pub tabs: Option<PaneTabs>,
    /// pane_title - #T Title of pane (can be set by application)
    #[cfg(feature = "tmux_1_6")]
    pub title: Option<String>,
    /// pane_top - Top of pane
    #[cfg(feature = "tmux_2_0")]
    pub top: Option<usize>,
    /// pane_tty - Pseudo terminal of pane
    #[cfg(feature = "tmux_1_6")]
    pub tty: Option<String>,
    /// pane_unseen_changes - 1 if there were changes in pane while in mode
    #[cfg(feature = "tmux_3_4")]
    pub unseen_changes: Option<bool>,
    /// pane_width - Width of pane
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
}

impl FromStr for Pane {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let mut pane = Pane::new();
        let mut format = FormatsOutput::new();
        format.separator(PANE_VARS_SEPARATOR);

        #[cfg(feature = "tmux_1_6")]
        format.pane_active(&mut pane.active);
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_bottom(&mut pane.at_bottom);
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_left(&mut pane.at_left);
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_right(&mut pane.at_right);
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_top(&mut pane.at_top);
        #[cfg(feature = "tmux_2_0")]
        format.pane_bottom(&mut pane.bottom);
        #[cfg(feature = "tmux_1_8")]
        format.pane_current_command(&mut pane.current_command);
        #[cfg(feature = "tmux_1_7")]
        format.pane_current_path(&mut pane.current_path);
        #[cfg(feature = "tmux_1_6")]
        format.pane_dead(&mut pane.dead);
        #[cfg(feature = "tmux_2_0")]
        format.pane_dead_status(&mut pane.dead_status);
        #[cfg(feature = "tmux_2_6")]
        format.pane_format(&mut pane.format);
        #[cfg(feature = "tmux_1_6")]
        format.pane_height(&mut pane.height);
        #[cfg(feature = "tmux_1_6")]
        format.pane_id(&mut pane.id);
        #[cfg(feature = "tmux_1_8")]
        format.pane_in_mode(&mut pane.in_mode);
        #[cfg(feature = "tmux_1_7")]
        format.pane_index(&mut pane.index);
        #[cfg(feature = "tmux_2_0")]
        format.pane_input_off(&mut pane.input_off);
        #[cfg(feature = "tmux_2_0")]
        format.pane_left(&mut pane.left);
        #[cfg(feature = "tmux_3_0")]
        format.pane_marked(&mut pane.marked);
        #[cfg(feature = "tmux_3_0")]
        format.pane_marked_set(&mut pane.marked_set);
        #[cfg(feature = "tmux_2_5")]
        format.pane_mode(&mut pane.mode);
        #[cfg(feature = "tmux_3_1")]
        format.pane_path(&mut pane.path);
        #[cfg(feature = "tmux_1_6")]
        format.pane_pid(&mut pane.pid);
        #[cfg(feature = "tmux_2_6")]
        format.pane_pipe(&mut pane.pipe);
        #[cfg(feature = "tmux_2_0")]
        format.pane_right(&mut pane.right);
        #[cfg(feature = "tmux_2_5")]
        format.pane_search_string(&mut pane.search_string);
        #[cfg(feature = "tmux_1_6")]
        format.pane_start_command(&mut pane.start_command);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
        format.pane_start_path(&mut pane.start_path);
        #[cfg(feature = "tmux_1_9")]
        format.pane_synchronized(&mut pane.synchronized);
        #[cfg(feature = "tmux_1_8")]
        format.pane_tabs(&mut pane.tabs);
        #[cfg(feature = "tmux_1_6")]
        format.pane_title(&mut pane.title);
        #[cfg(feature = "tmux_2_0")]
        format.pane_top(&mut pane.top);
        #[cfg(feature = "tmux_1_6")]
        format.pane_tty(&mut pane.tty);
        #[cfg(feature = "tmux_3_4")]
        format.pane_unseen_changes(&mut pane.unseen_changes);
        #[cfg(feature = "tmux_1_6")]
        format.pane_width(&mut pane.width);

        FormatsOutput::from_string_ext(s, &mut format);
        Ok(pane)
    }
}

impl Pane {
    pub fn new() -> Self {
        Default::default()
    }
}
