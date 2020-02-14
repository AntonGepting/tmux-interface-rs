use crate::Error;
use crate::Layout;
use crate::WindowFlag;
use std::time::Duration;

// NOTE: u32 mb not enough!
pub const WINDOW_ACTIVE: usize = 1 << 0;
pub const WINDOW_ACTIVITY: usize = 1 << 1;
pub const WINDOW_ACTIVITY_FLAG: usize = 1 << 2;
pub const WINDOW_BELL_FLAG: usize = 1 << 3;
pub const WINDOW_BIGGER: usize = 1 << 4;
pub const WINDOW_END_FLAG: usize = 1 << 5;
pub const WINDOW_FLAGS: usize = 1 << 6;
pub const WINDOW_FORMAT: usize = 1 << 7;
pub const WINDOW_HEIGHT: usize = 1 << 8;
pub const WINDOW_ID: usize = 1 << 9;
pub const WINDOW_INDEX: usize = 1 << 10;
pub const WINDOW_LAST_FLAG: usize = 1 << 11;
pub const WINDOW_LAYOUT: usize = 1 << 12;
pub const WINDOW_LINKED: usize = 1 << 13;
pub const WINDOW_NAME: usize = 1 << 14;
pub const WINDOW_OFFSET_X: usize = 1 << 15;
pub const WINDOW_OFFSET_Y: usize = 1 << 16;
pub const WINDOW_PANES: usize = 1 << 17;
pub const WINDOW_SILENCE_FLAG: usize = 1 << 18;
pub const WINDOW_STACK_INDEX: usize = 1 << 19;
pub const WINDOW_START_FLAG: usize = 1 << 20;
pub const WINDOW_VISIBLE_LAYOUT: usize = 1 << 21;
pub const WINDOW_WIDTH: usize = 1 << 22;
pub const WINDOW_ZOOMED_FLAG: usize = 1 << 23;

pub const WINDOW_FLAGS_NUM: usize = 24;

pub const WINDOW_NONE: usize = 0;
//pub const WINDOW_DEFAULT: usize = WINDOW_ID | WINDOW_NAME;
pub const WINDOW_ALL: usize = WINDOW_ACTIVE
    | WINDOW_ACTIVITY
    | WINDOW_ACTIVITY_FLAG
    | WINDOW_BELL_FLAG
    | WINDOW_BIGGER
    | WINDOW_END_FLAG
    | WINDOW_FLAGS
    | WINDOW_FORMAT
    | WINDOW_HEIGHT
    | WINDOW_ID
    | WINDOW_INDEX
    | WINDOW_LAST_FLAG
    | WINDOW_LAYOUT
    | WINDOW_LINKED
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
pub const WINDOW_VARS: [(&str, usize, fn(w: &mut Window, p: &str)); WINDOW_FLAGS_NUM] = [
    ("window_active", WINDOW_ACTIVE, |w, p| {
        w.active = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_activity", WINDOW_ACTIVITY, |w, p| {
        w.activity = p.parse().ok().map(Duration::from_millis)
    }),
    ("window_activity_flag", WINDOW_ACTIVITY_FLAG, |w, p| {
        w.activity_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_bell_flag", WINDOW_BELL_FLAG, |w, p| {
        w.bell_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_bigger", WINDOW_BIGGER, |w, p| {
        w.bigger = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_end_flag", WINDOW_END_FLAG, |w, p| {
        w.end_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_flags", WINDOW_FLAGS, |w, p| {
        w.flags = p.parse().ok()
    }),
    ("window_format", WINDOW_FORMAT, |w, p| {
        w.format = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_height", WINDOW_HEIGHT, |w, p| {
        w.height = p.parse().ok()
    }),
    ("window_id", WINDOW_ID, |w, p| w.id = p[1..].parse().ok()), // skip `@` char
    ("window_index", WINDOW_INDEX, |w, p| {
        w.index = p.parse().ok()
    }),
    ("window_last_flag", WINDOW_LAST_FLAG, |w, p| {
        w.last_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_layout", WINDOW_LAYOUT, |w, p| {
        w.layout = p.parse().ok()
    }),
    ("window_linked", WINDOW_LINKED, |w, p| {
        w.linked = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_name", WINDOW_NAME, |w, p| w.name = p.parse().ok()),
    ("window_offset_x", WINDOW_OFFSET_X, |w, p| {
        w.offset_x = p.parse().ok()
    }),
    ("window_offset_y", WINDOW_OFFSET_Y, |w, p| {
        w.offset_y = p.parse().ok()
    }),
    ("window_panes", WINDOW_PANES, |w, p| {
        w.panes = p.parse().ok()
    }),
    ("window_silence_flag", WINDOW_SILENCE_FLAG, |w, p| {
        w.silence_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_stack_index", WINDOW_STACK_INDEX, |w, p| {
        w.stack_index = p.parse().ok()
    }),
    ("window_start_flag", WINDOW_START_FLAG, |w, p| {
        w.start_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("window_visible_layout", WINDOW_VISIBLE_LAYOUT, |w, p| {
        w.visible_layout = p.parse().ok()
    }),
    ("window_width", WINDOW_WIDTH, |w, p| {
        w.width = p.parse().ok()
    }),
    ("window_zoomed_flag", WINDOW_ZOOMED_FLAG, |w, p| {
        w.zoomed_flag = p.parse::<usize>().map(|i| i == 1).ok()
    }),
];

// accordingly to tmux.h: Formats
// XXX: check all types, optionality
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Window {
    /// 1 if window active
    pub active: Option<bool>,
    /// Time of window last activity
    pub activity: Option<Duration>,
    /// 1 if window has activity
    pub activity_flag: Option<bool>,
    /// 1 if window has bell
    pub bell_flag: Option<bool>,
    /// 1 if window is larger than client
    pub bigger: Option<bool>,
    /// 1 if window has the highest index
    pub end_flag: Option<bool>,
    /// #F Window flags
    pub flags: Option<WindowFlag>,
    /// 1 if format is for a window (not assuming the current)
    pub format: Option<bool>,
    /// Height of window
    pub height: Option<usize>,
    /// Unique window ID
    pub id: Option<usize>,
    /// #I Index of window
    pub index: Option<usize>,
    /// 1 if window is the last used
    pub last_flag: Option<bool>,
    /// Window layout description, ignoring zoomed window panes
    pub layout: Option<Layout>,
    /// 1 if window is linked across sessions
    pub linked: Option<bool>,
    /// #W Name of window
    pub name: Option<String>,
    /// X offset into window if larger than client
    pub offset_x: Option<usize>,
    /// Y offset into window if larger than client
    pub offset_y: Option<usize>,
    /// Number of panes in window
    pub panes: Option<usize>,
    /// 1 if window has silence alert
    pub silence_flag: Option<bool>,
    /// Index in session most recent stack
    pub stack_index: Option<usize>,
    /// 1 if window has the lowest index
    pub start_flag: Option<bool>,
    /// Window layout description, respecting zoomed window panes
    pub visible_layout: Option<Layout>,
    /// Width of window
    pub width: Option<usize>,
    /// 1 if window is zoomed
    pub zoomed_flag: Option<bool>,
}

impl Window {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: mb deserialize like serde something?
    pub fn from_str(s: &str, bitflags: usize) -> Result<Self, Error> {
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
