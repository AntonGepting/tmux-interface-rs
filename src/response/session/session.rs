use crate::Error;
use crate::SessionStack;
use std::time::Duration;

// NOTE: u16 mb not enough!
pub const SESSION_ACTIVITY: u16 = 1;
pub const SESSION_ALERTS: u16 = 1 << 1;
pub const SESSION_ATTACHED: u16 = 1 << 2;
pub const SESSION_CREATED: u16 = 1 << 3;
pub const SESSION_FORMAT: u16 = 1 << 4;
pub const SESSION_GROUP: u16 = 1 << 5;
pub const SESSION_GROUP_LIST: u16 = 1 << 6;
pub const SESSION_GROUP_SIZE: u16 = 1 << 7;
pub const SESSION_GROUPED: u16 = 1 << 8;
pub const SESSION_ID: u16 = 1 << 9;
pub const SESSION_LAST_ATTACHED: u16 = 1 << 10;
pub const SESSION_MANY_ATTACHED: u16 = 1 << 11;
pub const SESSION_NAME: u16 = 1 << 12;
pub const SESSION_STACK: u16 = 1 << 13;
pub const SESSION_WINDOWS: u16 = 1 << 14;

// number of all flags, needed for array init
pub const SESSION_FLAGS_NUM: usize = 15;

pub const SESSION_NONE: u16 = 0;
//pub const SESSION_DEFAULT: usize = SESSION_ID | SESSION_NAME;
pub const SESSION_ALL: u16 = SESSION_ACTIVITY
    | SESSION_ATTACHED
    | SESSION_ALERTS
    | SESSION_CREATED
    | SESSION_FORMAT
    | SESSION_GROUP
    | SESSION_GROUP_LIST
    | SESSION_GROUP_SIZE
    | SESSION_GROUPED
    | SESSION_ID
    | SESSION_LAST_ATTACHED
    | SESSION_MANY_ATTACHED
    | SESSION_NAME
    | SESSION_STACK
    | SESSION_WINDOWS;

pub const SESSION_VARS_SEPARATOR: &str = ":";

// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS: [(&str, u16, fn(s: &mut Session, p: &str)); SESSION_FLAGS_NUM] = [
    ("session_activity", SESSION_ACTIVITY, |s, p| {
        s.activity = p.parse().ok().map(Duration::from_millis)
    }),
    ("session_alerts", SESSION_ALERTS, |s, p| {
        s.alerts = p.parse().ok()
    }),
    ("session_attached", SESSION_ATTACHED, |s, p| {
        s.attached = p.parse().ok()
    }),
    ("session_created", SESSION_CREATED, |s, p| {
        s.created = p.parse().ok().map(Duration::from_millis)
    }),
    ("session_format", SESSION_FORMAT, |s, p| {
        s.format = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("session_group", SESSION_GROUP, |s, p| {
        s.group = p.parse().ok()
    }),
    ("session_group_list", SESSION_GROUP_LIST, |s, p| {
        s.group_list = p.parse().ok()
    }),
    ("session_group_size", SESSION_GROUP_SIZE, |s, p| {
        s.group_size = p.parse().ok()
    }),
    ("session_grouped", SESSION_GROUPED, |s, p| {
        s.grouped = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("session_id", SESSION_ID, |s, p| s.id = p[1..].parse().ok()), // skip '$' char
    ("session_last_attached", SESSION_LAST_ATTACHED, |s, p| {
        s.last_attached = p.parse().ok().map(Duration::from_millis)
    }),
    ("session_many_attached", SESSION_MANY_ATTACHED, |s, p| {
        s.many_attached = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    ("session_name", SESSION_NAME, |s, p| s.name = p.parse().ok()),
    ("session_stack", SESSION_STACK, |s, p| {
        s.stack = p.parse().ok()
    }),
    ("session_windows", SESSION_WINDOWS, |s, p| {
        s.windows = p.parse().ok()
    }),
];

// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Session {
    /// Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    pub activity: Option<Duration>,
    /// List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    pub alerts: Option<String>,
    /// Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    pub attached: Option<usize>,
    /// Time session created
    #[cfg(feature = "tmux_1_6")]
    pub created: Option<Duration>,
    /// String time session created
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_2")))]
    pub created_string: Option<String>,
    /// 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// Name of session group
    #[cfg(feature = "tmux_1_6")]
    pub group: Option<String>,
    /// List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    pub group_list: Option<String>,
    /// Size of session group
    #[cfg(feature = "tmux_2_7")]
    pub group_size: Option<String>,
    /// 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    pub grouped: Option<bool>,
    /// Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub height: Option<usize>,
    /// Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub width: Option<usize>,
    /// Unique session ID
    #[cfg(feature = "tmux_1_8")]
    pub id: Option<usize>,
    /// Time session last attached
    #[cfg(feature = "tmux_2_1")]
    pub last_attached: Option<Duration>,
    /// 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    pub many_attached: Option<bool>,
    /// #S Name of session
    #[cfg(feature = "tmux_1_6")]
    pub name: Option<String>,
    /// Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    pub stack: Option<SessionStack>,
    /// Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    pub windows: Option<usize>,
}

impl Session {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(s: &str, bitflags: u16) -> Result<Self, Error> {
        let sv: Vec<&str> = s.split(SESSION_VARS_SEPARATOR).collect();
        let mut sv = sv.iter();
        // XXX: optimize?
        let mut s = Session::new();
        // for all bitflags
        for var in SESSION_VARS.iter() {
            // is current bitflag given?
            if bitflags & var.1 == var.1 {
                // does vector element exist?
                if let Some(part) = sv.next() {
                    // is vector element not empty
                    if !part.is_empty() {
                        // call corresponding func from array
                        var.2(&mut s, part);
                    }
                }
            }
        }
        Ok(s)
    }

    // XXX: wrapper with format generating and result parsing using callback
}
