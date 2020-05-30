use crate::Error;
use crate::PaneTabs;

// NOTE: u32 mb not enough!
pub const PANE_ACTIVE: u32 = 1;
pub const PANE_AT_BOTTOM: u32 = 1 << 1;
pub const PANE_AT_LEFT: u32 = 1 << 2;
pub const PANE_AT_RIGHT: u32 = 1 << 3;
pub const PANE_AT_TOP: u32 = 1 << 4;
pub const PANE_BOTTOM: u32 = 1 << 5;
pub const PANE_CURRENT_COMMAND: u32 = 1 << 6;
pub const PANE_CURRENT_PATH: u32 = 1 << 7;
pub const PANE_DEAD: u32 = 1 << 8;
pub const PANE_DEAD_STATUS: u32 = 1 << 9;
pub const PANE_FORMAT: u32 = 1 << 10;
pub const PANE_HEIGHT: u32 = 1 << 11;
pub const PANE_ID: u32 = 1 << 12;
pub const PANE_IN_MODE: u32 = 1 << 13;
pub const PANE_INDEX: u32 = 1 << 14;
pub const PANE_INPUT_OFF: u32 = 1 << 15;
pub const PANE_LEFT: u32 = 1 << 16;
pub const PANE_MARKED: u32 = 1 << 17;
pub const PANE_MARKED_SET: u32 = 1 << 18;
pub const PANE_MODE: u32 = 1 << 19;
pub const PANE_PID: u32 = 1 << 20;
pub const PANE_PIPE: u32 = 1 << 21;
pub const PANE_RIGHT: u32 = 1 << 22;
pub const PANE_SEARCH_STRING: u32 = 1 << 23;
pub const PANE_START_COMMMAND: u32 = 1 << 24;
pub const PANE_SYNCHRONIZED: u32 = 1 << 25;
pub const PANE_TABS: u32 = 1 << 26;
pub const PANE_TITLE: u32 = 1 << 27;
pub const PANE_TOP: u32 = 1 << 28;
pub const PANE_TTY: u32 = 1 << 29;
pub const PANE_WIDTH: u32 = 1 << 30;

pub const PANE_FLAGS_NUM: usize = 31;

pub const PANE_NONE: u32 = 0;
//pub const PANE_DEFAULT: usize = PANE_ID | PANE_TITLE;
pub const PANE_ALL: u32 = PANE_ACTIVE
    | PANE_AT_BOTTOM
    | PANE_AT_LEFT
    | PANE_AT_RIGHT
    | PANE_AT_TOP
    | PANE_BOTTOM
    | PANE_CURRENT_COMMAND
    | PANE_CURRENT_PATH
    | PANE_DEAD
    | PANE_DEAD_STATUS
    | PANE_FORMAT
    | PANE_HEIGHT
    | PANE_ID
    | PANE_IN_MODE
    | PANE_INDEX
    | PANE_INPUT_OFF
    | PANE_LEFT
    | PANE_MARKED
    | PANE_MARKED_SET
    | PANE_MODE
    | PANE_PID
    | PANE_PIPE
    | PANE_RIGHT
    | PANE_SEARCH_STRING
    | PANE_START_COMMMAND
    | PANE_SYNCHRONIZED
    | PANE_TABS
    | PANE_TITLE
    | PANE_TOP
    | PANE_TTY
    | PANE_WIDTH;

pub const PANE_VARS_SEPARATOR: &str = "'";
// FIXME: regex name can be anything, and other keys should be checked better
pub const PANE_VARS: [(&str, u32, fn(p: &mut Pane, t: &str)); PANE_FLAGS_NUM] = [
    ("pane_active", PANE_ACTIVE, |p, t| {
        p.active = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_at_bottom", PANE_AT_BOTTOM, |p, t| {
        p.at_bottom = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_at_left", PANE_AT_LEFT, |p, t| {
        p.at_left = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_at_right", PANE_AT_RIGHT, |p, t| {
        p.at_right = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_at_top", PANE_AT_TOP, |p, t| {
        p.at_top = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_bottom", PANE_BOTTOM, |p, t| p.bottom = t.parse().ok()),
    ("pane_current_command", PANE_CURRENT_COMMAND, |p, t| {
        p.current_command = t.parse().ok()
    }),
    ("pane_current_path", PANE_CURRENT_PATH, |p, t| {
        p.current_path = t.parse().ok()
    }),
    ("pane_dead", PANE_DEAD, |p, t| {
        p.dead = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_dead_status", PANE_DEAD, |p, t| {
        p.dead_status = t.parse().ok()
    }),
    ("pane_format", PANE_FORMAT, |p, t| {
        p.format = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_height", PANE_HEIGHT, |p, t| p.height = t.parse().ok()),
    ("pane_id", PANE_ID, |p, t| p.id = t[1..].parse().ok()), // skip '%' char
    ("pane_in_mode", PANE_IN_MODE, |p, t| {
        p.in_mode = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_index", PANE_INDEX, |p, t| p.index = t.parse().ok()),
    ("pane_input_off", PANE_INPUT_OFF, |p, t| {
        p.input_off = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_left", PANE_LEFT, |p, t| p.left = t.parse().ok()),
    ("pane_marked", PANE_MARKED, |p, t| {
        p.marked = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_marked_set", PANE_MARKED_SET, |p, t| {
        p.marked_set = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_mode", PANE_MODE, |p, t| p.mode = t.parse().ok()),
    ("pane_pid", PANE_PID, |p, t| p.pid = t.parse().ok()),
    ("pane_pipe", PANE_PIPE, |p, t| {
        p.pipe = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_right", PANE_RIGHT, |p, t| p.right = t.parse().ok()),
    ("pane_search_string", PANE_SEARCH_STRING, |p, t| {
        p.search_string = t.parse().ok()
    }),
    ("pane_start_command", PANE_START_COMMMAND, |p, t| {
        p.start_command = t.parse().ok()
    }),
    ("pane_synchronized", PANE_SYNCHRONIZED, |p, t| {
        p.synchronized = t.parse::<usize>().map(|b| b == 1).ok()
    }),
    ("pane_tabs", PANE_TABS, |p, t| p.tabs = t.parse().ok()),
    ("pane_title", PANE_TITLE, |p, t| p.title = t.parse().ok()),
    ("pane_top", PANE_TOP, |p, t| p.top = t.parse().ok()),
    ("pane_tty", PANE_TTY, |p, t| p.tty = t.parse().ok()),
    ("pane_width", PANE_WIDTH, |p, t| p.width = t.parse().ok()),
];

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
    /// pane_width - Width of pane
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
}

impl Pane {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(pane_str: &str, bitflags: u32) -> Result<Self, Error> {
        let pv: Vec<&str> = pane_str.split(PANE_VARS_SEPARATOR).collect();
        let mut pv = pv.iter();
        // XXX: optimize?
        let mut p = Pane::new();
        // for all bitflags
        for var in PANE_VARS.iter() {
            // is current bitflag given?
            if bitflags & var.1 == var.1 {
                // does vector element exist?
                if let Some(part) = pv.next() {
                    // is vector element not empty
                    if !part.is_empty() {
                        // call corresponding func from array
                        var.2(&mut p, part);
                    }
                }
            }
        }
        Ok(p)
    }
}
