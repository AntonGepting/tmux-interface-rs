use crate::Error;
use crate::Layout;
use crate::WindowFlag;
#[cfg(feature = "tmux_2_1")]
use std::time::Duration;

pub const WINDOW_ACTIVE: u64 = 1;
pub const WINDOW_ACTIVE_CLIENTS: u64 = 1 << 1;
pub const WINDOW_ACTIVE_CLIENTS_LIST: u64 = 1 << 2;
pub const WINDOW_ACTIVE_SESSIONS: u64 = 1 << 3;
pub const WINDOW_ACTIVE_SESSIONS_LIST: u64 = 1 << 4;
pub const WINDOW_ACTIVITY: u64 = 1 << 5;
pub const WINDOW_ACTIVITY_STRING: u64 = 1 << 6;
pub const WINDOW_ACTIVITY_FLAG: u64 = 1 << 7;
pub const WINDOW_BELL_FLAG: u64 = 1 << 8;
pub const WINDOW_CONTENT_FLAG: u64 = 1 << 9;
pub const WINDOW_BIGGER: u64 = 1 << 10;
pub const WINDOW_CELL_HEIGHT: u64 = 1 << 11;
pub const WINDOW_CELL_WIDTH: u64 = 1 << 12;
pub const WINDOW_END_FLAG: u64 = 1 << 13;
pub const WINDOW_FIND_MATCHES: u64 = 1 << 14;
pub const WINDOW_FLAGS: u64 = 1 << 15;
pub const WINDOW_FORMAT: u64 = 1 << 16;
pub const WINDOW_HEIGHT: u64 = 1 << 17;
pub const WINDOW_ID: u64 = 1 << 18;
pub const WINDOW_INDEX: u64 = 1 << 19;
pub const WINDOW_LAST_FLAG: u64 = 1 << 20;
pub const WINDOW_LAYOUT: u64 = 1 << 21;
pub const WINDOW_LINKED: u64 = 1 << 22;
pub const WINDOW_LINKED_SESSIONS: u64 = 1 << 23;
pub const WINDOW_LINKED_SESSIONS_LIST: u64 = 1 << 24;
pub const WINDOW_MARKED_FLAG: u64 = 1 << 25;
pub const WINDOW_NAME: u64 = 1 << 26;
pub const WINDOW_OFFSET_X: u64 = 1 << 27;
pub const WINDOW_OFFSET_Y: u64 = 1 << 28;
pub const WINDOW_PANES: u64 = 1 << 29;
pub const WINDOW_SILENCE_FLAG: u64 = 1 << 30;
pub const WINDOW_STACK_INDEX: u64 = 1 << 31;
pub const WINDOW_START_FLAG: u64 = 1 << 32;
pub const WINDOW_VISIBLE_LAYOUT: u64 = 1 << 33;
pub const WINDOW_WIDTH: u64 = 1 << 34;
pub const WINDOW_ZOOMED_FLAG: u64 = 1 << 35;

// XXX: number of all flags, needed for array init
// NOTE: variables were first intoduced in tmux 1.6
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_7")))]
pub const WINDOW_FLAGS_NUM: usize = 7;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const WINDOW_FLAGS_NUM: usize = 10;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_FLAGS_NUM: usize = 10;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_1_9a")))]
pub const WINDOW_FLAGS_NUM: usize = 14;
#[cfg(all(feature = "tmux_1_9a", not(feature = "tmux_2_0")))]
pub const WINDOW_FLAGS_NUM: usize = 14;
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_1")))]
pub const WINDOW_FLAGS_NUM: usize = 15;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub const WINDOW_FLAGS_NUM: usize = 18;
#[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_3")))]
pub const WINDOW_FLAGS_NUM: usize = 17;
#[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
pub const WINDOW_FLAGS_NUM: usize = 18;
#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_5")))]
pub const WINDOW_FLAGS_NUM: usize = 18;
#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_6")))]
pub const WINDOW_FLAGS_NUM: usize = 19;
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_7")))]
pub const WINDOW_FLAGS_NUM: usize = 19;
#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_2_8")))]
pub const WINDOW_FLAGS_NUM: usize = 19;
#[cfg(all(feature = "tmux_2_8", not(feature = "tmux_2_9")))]
pub const WINDOW_FLAGS_NUM: usize = 19;
#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_2_9a")))]
pub const WINDOW_FLAGS_NUM: usize = 24;
#[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_0")))]
pub const WINDOW_FLAGS_NUM: usize = 24;
#[cfg(all(feature = "tmux_3_0", not(feature = "tmux_3_0a")))]
pub const WINDOW_FLAGS_NUM: usize = 24;
#[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_1")))]
pub const WINDOW_FLAGS_NUM: usize = 24;
#[cfg(all(feature = "tmux_3_1", not(feature = "tmux_3_1a")))]
pub const WINDOW_FLAGS_NUM: usize = 33;
#[cfg(all(feature = "tmux_3_1a", not(feature = "tmux_3_1b")))]
pub const WINDOW_FLAGS_NUM: usize = 33;
#[cfg(all(feature = "tmux_3_1b", not(feature = "tmux_X_X")))]
pub const WINDOW_FLAGS_NUM: usize = 33;
#[cfg(feature = "tmux_X_X")]
pub const WINDOW_FLAGS_NUM: usize = 33;

pub const WINDOW_NONE: u64 = 0;
//pub const WINDOW_DEFAULT: usize = WINDOW_ID | WINDOW_NAME;
pub const WINDOW_ALL: u64 = WINDOW_ACTIVE
    | WINDOW_ACTIVE_CLIENTS
    | WINDOW_ACTIVE_CLIENTS_LIST
    | WINDOW_ACTIVE_SESSIONS
    | WINDOW_ACTIVE_SESSIONS_LIST
    | WINDOW_ACTIVITY
    | WINDOW_ACTIVITY_STRING
    | WINDOW_ACTIVITY_FLAG
    | WINDOW_BELL_FLAG
    | WINDOW_CONTENT_FLAG
    | WINDOW_BIGGER
    | WINDOW_CELL_HEIGHT
    | WINDOW_CELL_WIDTH
    | WINDOW_END_FLAG
    | WINDOW_FIND_MATCHES
    | WINDOW_FLAGS
    | WINDOW_FORMAT
    | WINDOW_HEIGHT
    | WINDOW_ID
    | WINDOW_INDEX
    | WINDOW_LAST_FLAG
    | WINDOW_LAYOUT
    | WINDOW_LINKED
    | WINDOW_LINKED_SESSIONS
    | WINDOW_LINKED_SESSIONS_LIST
    | WINDOW_MARKED_FLAG
    | WINDOW_NAME
    | WINDOW_OFFSET_X
    | WINDOW_OFFSET_Y
    | WINDOW_PANES
    | WINDOW_SILENCE_FLAG
    | WINDOW_STACK_INDEX
    | WINDOW_START_FLAG
    | WINDOW_VISIBLE_LAYOUT
    | WINDOW_WIDTH
    | WINDOW_ZOOMED_FLAG;

pub const WINDOW_VARS_SEPARATOR: &str = "'";
// FIXME: regex name can be anything, and other keys should be checked better
pub const WINDOW_VARS: [(&str, u64, fn(w: &mut Window, p: &str)); WINDOW_FLAGS_NUM] = [
    #[cfg(feature = "tmux_1_6")]
    ("window_active", WINDOW_ACTIVE, |w, p| {
        w.active = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    ("window_active_clients", WINDOW_ACTIVE_CLIENTS, |w, p| {
        w.active_clients = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    (
        "window_active_clients_list",
        WINDOW_ACTIVE_CLIENTS_LIST,
        |w, p| w.active_clients_list = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_3_1")]
    ("window_active_sessions", WINDOW_ACTIVE_SESSIONS, |w, p| {
        w.active_sessions = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    (
        "window_active_sessions_list",
        WINDOW_ACTIVE_SESSIONS_LIST,
        |w, p| w.active_sessions_list = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_2_1")]
    ("window_activity", WINDOW_ACTIVITY, |w, p| {
        w.activity = p.parse().ok().map(Duration::from_millis)
    }),
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    ("window_activity_string", WINDOW_ACTIVITY_STRING, |w, p| {
        w.activity_string = p.parse().ok()
    }),
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    ("window_activity_flag", WINDOW_ACTIVITY_FLAG, |w, p| {
        w.activity_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_1_9")]
    ("window_bell_flag", WINDOW_BELL_FLAG, |w, p| {
        w.bell_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    ("window_content_flag", WINDOW_CONTENT_FLAG, |w, p| {
        w.content_flag = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_9")]
    ("window_bigger", WINDOW_BIGGER, |w, p| {
        w.bigger = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    ("window_cell_height", WINDOW_CELL_HEIGHT, |w, p| {
        w.cell_height = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    ("window_cell_width", WINDOW_CELL_WIDTH, |w, p| {
        w.cell_width = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_9")]
    ("window_end_flag", WINDOW_END_FLAG, |w, p| {
        w.end_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    ("window_find_matches", WINDOW_FIND_MATCHES, |w, p| {
        w.find_matches = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("window_flags", WINDOW_FLAGS, |w, p| {
        w.flags = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_6")]
    ("window_format", WINDOW_FORMAT, |w, p| {
        w.format = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("window_height", WINDOW_HEIGHT, |w, p| {
        w.height = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_7")]
    ("window_id", WINDOW_ID, |w, p| w.id = p[1..].parse().ok()), // skip `@` char
    #[cfg(feature = "tmux_1_6")]
    ("window_index", WINDOW_INDEX, |w, p| {
        w.index = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_0")]
    ("window_last_flag", WINDOW_LAST_FLAG, |w, p| {
        w.last_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("window_layout", WINDOW_LAYOUT, |w, p| {
        w.layout = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_1")]
    ("window_linked", WINDOW_LINKED, |w, p| {
        w.linked = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    ("window_linked_sessions", WINDOW_LINKED_SESSIONS, |w, p| {
        w.linked_sessions = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    (
        "window_linked_sessions_list",
        WINDOW_LINKED_SESSIONS_LIST,
        |w, p| w.linked_sessions_list = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_3_1")]
    ("window_marked_flag", WINDOW_MARKED_FLAG, |w, p| {
        w.marked_flag = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("window_name", WINDOW_NAME, |w, p| w.name = p.parse().ok()),
    #[cfg(feature = "tmux_2_9")]
    ("window_offset_x", WINDOW_OFFSET_X, |w, p| {
        w.offset_x = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_9")]
    ("window_offset_y", WINDOW_OFFSET_Y, |w, p| {
        w.offset_y = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_7")]
    ("window_panes", WINDOW_PANES, |w, p| {
        w.panes = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_9")]
    ("window_silence_flag", WINDOW_SILENCE_FLAG, |w, p| {
        w.silence_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_2_5")]
    ("window_stack_index", WINDOW_STACK_INDEX, |w, p| {
        w.stack_index = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_9")]
    ("window_start_flag", WINDOW_START_FLAG, |w, p| {
        w.start_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_2_2")]
    ("window_visible_layout", WINDOW_VISIBLE_LAYOUT, |w, p| {
        w.visible_layout = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("window_width", WINDOW_WIDTH, |w, p| {
        w.width = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_0")]
    ("window_zoomed_flag", WINDOW_ZOOMED_FLAG, |w, p| {
        w.zoomed_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
];

// accordingly to tmux.h: Formats
// XXX: check all types, optionality
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Window {
    /// window_active - 1 if window active
    #[cfg(feature = "tmux_1_6")]
    pub active: Option<bool>,
    /// window_active_clients - Number of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub active_clients: Option<usize>,
    /// window_active_clients_list - List of clients viewing this window
    #[cfg(feature = "tmux_3_1")]
    pub active_clients_list: Option<String>,
    /// window_active_sessions - Number of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub active_sessions: Option<usize>,
    /// window_active_sessions_list - List of sessions on which this window is active
    #[cfg(feature = "tmux_3_1")]
    pub active_sessions_list: Option<String>,
    /// window_activity - Time of window last activity
    #[cfg(feature = "tmux_2_1")]
    pub activity: Option<Duration>,
    /// session_activity_string - String time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub activity_string: Option<String>,
    /// window_activity_flag - 1 if window has activity
    #[cfg(any(
        all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
        feature = "tmux_2_3"
    ))]
    pub activity_flag: Option<bool>,
    /// window_bell_flag - 1 if window has bell
    #[cfg(feature = "tmux_1_9")]
    pub bell_flag: Option<bool>,
    /// window_content_flag - 1 if window has content alert
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub content_flag: Option<bool>,
    /// 1 if window is larger than client
    #[cfg(feature = "tmux_2_9")]
    pub bigger: Option<bool>,
    /// window_cell_height - Height of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub cell_height: Option<usize>,
    /// window_cell_width - Width of each cell in pixels
    #[cfg(feature = "tmux_3_1")]
    pub cell_width: Option<usize>,
    /// 1 if window has the highest index
    #[cfg(feature = "tmux_2_9")]
    pub end_flag: Option<bool>,
    /// window_find_matches - Matched data from the find-window command if available
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    pub find_matches: Option<String>,
    /// window_flags - #F Window flags
    #[cfg(feature = "tmux_1_6")]
    pub flags: Option<WindowFlag>,
    /// 1 if format is for a window
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// window_height - Height of window
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    /// window_id - Unique window ID
    #[cfg(feature = "tmux_1_7")]
    pub id: Option<usize>,
    /// window_index - #I Index of window
    #[cfg(feature = "tmux_1_6")]
    pub index: Option<usize>,
    /// window_last_flag - 1 if window is the last used
    #[cfg(feature = "tmux_2_0")]
    pub last_flag: Option<bool>,
    /// window_layout - Window layout description, ignoring zoomed window panes
    #[cfg(feature = "tmux_1_6")]
    pub layout: Option<Layout>,
    /// window_linked - 1 if window is linked across sessions
    #[cfg(feature = "tmux_2_1")]
    pub linked: Option<bool>,
    /// window_linked_sessions - Number of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub linked_sessions: Option<usize>,
    /// window_linked_sessions_list - List of sessions this window is linked to
    #[cfg(feature = "tmux_3_1")]
    pub linked_sessions_list: Option<String>,
    /// window_marked_flag - 1 if window contains the marked pane
    #[cfg(feature = "tmux_3_1")]
    pub marked_flag: Option<bool>,
    /// window_name - #W Name of window
    #[cfg(feature = "tmux_1_6")]
    pub name: Option<String>,
    /// X offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub offset_x: Option<usize>,
    /// Y offset into window if larger than client
    #[cfg(feature = "tmux_2_9")]
    pub offset_y: Option<usize>,
    /// window_panes - Number of panes in window
    #[cfg(feature = "tmux_1_7")]
    pub panes: Option<usize>,
    /// window_silence_flag - 1 if window has silence alert
    #[cfg(feature = "tmux_1_9")]
    pub silence_flag: Option<bool>,
    /// Index in session most recent stack
    #[cfg(feature = "tmux_2_5")]
    pub stack_index: Option<usize>,
    /// 1 if window has the lowest index
    #[cfg(feature = "tmux_2_9")]
    pub start_flag: Option<bool>,
    /// Window layout description, respecting zoomed window panes
    #[cfg(feature = "tmux_2_2")]
    pub visible_layout: Option<Layout>,
    /// window_width - Width of window
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
    /// window_zoomed_flag - 1 if window is zoomed
    #[cfg(feature = "tmux_2_0")]
    pub zoomed_flag: Option<bool>,
}

impl Window {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: mb deserialize like serde something?
    pub fn from_str(s: &str, bitflags: u64) -> Result<Self, Error> {
        let wv: Vec<&str> = s.split(WINDOW_VARS_SEPARATOR).collect();
        let mut wv = wv.iter();
        // XXX: optimize?
        let mut w = Window::new();
        // for all bitflags
        for var in WINDOW_VARS.iter() {
            // is current bitflag given?
            if bitflags & var.1 == var.1 {
                // does vector element exist?
                if let Some(part) = wv.next() {
                    // is vector element not empty
                    if !part.is_empty() {
                        // call corresponding func from array
                        var.2(&mut w, part);
                    }
                }
            }
        }
        Ok(w)
    }
}
