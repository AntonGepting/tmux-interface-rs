use crate::Error;
#[cfg(feature = "tmux_2_5")]
use crate::SessionStack;
#[cfg(feature = "tmux_1_6")]
use std::time::Duration;

pub const SESSION_ACTIVITY: u32 = 1;
pub const SESSION_ACTIVITY_STRING: u32 = 1 << 1;
pub const SESSION_ALERTS: u32 = 1 << 2;
pub const SESSION_ATTACHED: u32 = 1 << 3;
pub const SESSION_ATTACHED_LIST: u32 = 1 << 4;
pub const SESSION_CREATED: u32 = 1 << 5;
pub const SESSION_CREATED_STRING: u32 = 1 << 6;
pub const SESSION_FORMAT: u32 = 1 << 7;
pub const SESSION_GROUP: u32 = 1 << 8;
pub const SESSION_GROUP_ATTACHED: u32 = 1 << 9;
pub const SESSION_GROUP_ATTACHED_LIST: u32 = 1 << 10;
pub const SESSION_GROUP_LIST: u32 = 1 << 11;
pub const SESSION_GROUP_MANY_ATTACHED: u32 = 1 << 12;
pub const SESSION_GROUP_SIZE: u32 = 1 << 13;
pub const SESSION_GROUPED: u32 = 1 << 14;
pub const SESSION_HEIGHT: u32 = 1 << 15;
pub const SESSION_WIDTH: u32 = 1 << 16;
pub const SESSION_ID: u32 = 1 << 17;
pub const SESSION_LAST_ATTACHED: u32 = 1 << 18;
pub const SESSION_LAST_ATTACHED_STRING: u32 = 1 << 19;
pub const SESSION_MANY_ATTACHED: u32 = 1 << 20;
pub const SESSION_NAME: u32 = 1 << 21;
pub const SESSION_STACK: u32 = 1 << 22;
pub const SESSION_WINDOWS: u32 = 1 << 23;

// XXX: number of all flags, needed for array init
// NOTE: variables were first intoduced in tmux 1.6
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_7")))]
pub const SESSION_FLAGS_NUM: usize = 9;
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const SESSION_FLAGS_NUM: usize = 9;
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const SESSION_FLAGS_NUM: usize = 10;
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_1_9a")))]
pub const SESSION_FLAGS_NUM: usize = 10;
#[cfg(all(feature = "tmux_1_9a", not(feature = "tmux_2_0")))]
pub const SESSION_FLAGS_NUM: usize = 10;
#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_1")))]
pub const SESSION_FLAGS_NUM: usize = 11;
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
pub const SESSION_FLAGS_NUM: usize = 16;
#[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_3")))]
pub const SESSION_FLAGS_NUM: usize = 13;
#[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
pub const SESSION_FLAGS_NUM: usize = 13;
#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_5")))]
pub const SESSION_FLAGS_NUM: usize = 13;
#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_6")))]
pub const SESSION_FLAGS_NUM: usize = 14;
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_7")))]
pub const SESSION_FLAGS_NUM: usize = 15;
#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_2_8")))]
pub const SESSION_FLAGS_NUM: usize = 17;
#[cfg(all(feature = "tmux_2_8", not(feature = "tmux_2_9")))]
pub const SESSION_FLAGS_NUM: usize = 17;
#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_2_9a")))]
pub const SESSION_FLAGS_NUM: usize = 15;
#[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_0")))]
pub const SESSION_FLAGS_NUM: usize = 15;
#[cfg(all(feature = "tmux_3_0", not(feature = "tmux_3_0a")))]
pub const SESSION_FLAGS_NUM: usize = 15;
#[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_1")))]
pub const SESSION_FLAGS_NUM: usize = 15;
#[cfg(all(feature = "tmux_3_1", not(feature = "tmux_3_1a")))]
pub const SESSION_FLAGS_NUM: usize = 19;
#[cfg(all(feature = "tmux_3_1a", not(feature = "tmux_3_1b")))]
pub const SESSION_FLAGS_NUM: usize = 19;
#[cfg(all(feature = "tmux_3_1b", not(feature = "tmux_X_X")))]
pub const SESSION_FLAGS_NUM: usize = 19;
#[cfg(feature = "tmux_X_X")]
pub const SESSION_FLAGS_NUM: usize = 19;

pub const SESSION_NONE: u32 = 0;
//pub const SESSION_DEFAULT: usize = SESSION_ID | SESSION_NAME;
pub const SESSION_ALL: u32 = SESSION_ACTIVITY
    | SESSION_ACTIVITY_STRING
    | SESSION_ALERTS
    | SESSION_ATTACHED
    | SESSION_ATTACHED_LIST
    | SESSION_CREATED
    | SESSION_CREATED_STRING
    | SESSION_FORMAT
    | SESSION_GROUP
    | SESSION_GROUP_ATTACHED
    | SESSION_GROUP_ATTACHED_LIST
    | SESSION_GROUP_LIST
    | SESSION_GROUP_MANY_ATTACHED
    | SESSION_GROUP_SIZE
    | SESSION_GROUPED
    | SESSION_HEIGHT
    | SESSION_WIDTH
    | SESSION_ID
    | SESSION_LAST_ATTACHED
    | SESSION_LAST_ATTACHED_STRING
    | SESSION_MANY_ATTACHED
    | SESSION_NAME
    | SESSION_STACK
    | SESSION_WINDOWS;

pub const SESSION_VARS_SEPARATOR: &str = ":";

// FIXME: keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS: [(&str, u32, fn(s: &mut Session, p: &str)); SESSION_FLAGS_NUM] = [
    #[cfg(feature = "tmux_2_1")]
    ("session_activity", SESSION_ACTIVITY, |s, p| {
        s.activity = p.parse().ok().map(Duration::from_millis)
    }),
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    (
        "session_activity_string",
        SESSION_ACTIVITY_STRING,
        |s, p| s.activity_string = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_2_1")]
    ("session_alerts", SESSION_ALERTS, |s, p| {
        s.alerts = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("session_attached", SESSION_ATTACHED, |s, p| {
        s.attached = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    ("session_attached_list", SESSION_ATTACHED_LIST, |s, p| {
        s.attached_list = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("session_created", SESSION_CREATED, |s, p| {
        s.created = p.parse().ok().map(Duration::from_millis)
    }),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    ("session_created_string", SESSION_CREATED_STRING, |s, p| {
        s.created_string = p.parse().ok()
    }),
    #[cfg(feature = "tmux_2_6")]
    ("session_format", SESSION_FORMAT, |s, p| {
        s.format = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("session_group", SESSION_GROUP, |s, p| {
        s.group = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    ("session_group_attached", SESSION_GROUP_ATTACHED, |s, p| {
        s.group_attached = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    (
        "session_group_attached_list",
        SESSION_GROUP_ATTACHED_LIST,
        |s, p| s.group_attached_list = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_2_7")]
    ("session_group_list", SESSION_GROUP_LIST, |s, p| {
        s.group_list = p.parse().ok()
    }),
    #[cfg(feature = "tmux_3_1")]
    (
        "session_group_many_attached",
        SESSION_GROUP_MANY_ATTACHED,
        |s, p| s.group_many_attached = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_2_7")]
    ("session_group_size", SESSION_GROUP_SIZE, |s, p| {
        s.group_size = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("session_grouped", SESSION_GROUPED, |s, p| {
        s.grouped = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    ("session_height", SESSION_HEIGHT, |s, p| {
        s.height = p.parse().ok()
    }),
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    ("session_width", SESSION_WIDTH, |s, p| {
        s.width = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_8")]
    ("session_id", SESSION_ID, |s, p| s.id = p[1..].parse().ok()), // skip '$' char
    #[cfg(feature = "tmux_2_1")]
    ("session_last_attached", SESSION_LAST_ATTACHED, |s, p| {
        s.last_attached = p.parse().ok().map(Duration::from_millis)
    }),
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    (
        "session_last_attached_string",
        SESSION_LAST_ATTACHED_STRING,
        |s, p| s.last_attached_string = p.parse().ok(),
    ),
    #[cfg(feature = "tmux_2_0")]
    ("session_many_attached", SESSION_MANY_ATTACHED, |s, p| {
        s.many_attached = p.parse::<usize>().map(|i| i == 1).ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("session_name", SESSION_NAME, |s, p| s.name = p.parse().ok()),
    #[cfg(feature = "tmux_2_5")]
    ("session_stack", SESSION_STACK, |s, p| {
        s.stack = p.parse().ok()
    }),
    #[cfg(feature = "tmux_1_6")]
    ("session_windows", SESSION_WINDOWS, |s, p| {
        s.windows = p.parse().ok()
    }),
];

// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Session {
    /// session_activity - Time of session last activity
    #[cfg(feature = "tmux_2_1")]
    pub activity: Option<Duration>,
    /// session_activity_string - String time of session last activity
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub activity_string: Option<String>,
    /// session_alerts - List of window indexes with alerts
    #[cfg(feature = "tmux_2_1")]
    pub alerts: Option<String>,
    /// session_attached - Number of clients session is attached to
    #[cfg(feature = "tmux_1_6")]
    pub attached: Option<usize>,
    /// session_attached_list - List of clients session is attached to
    #[cfg(feature = "tmux_3_1")]
    pub attached_list: Option<usize>,
    /// session_created - Time session created
    #[cfg(feature = "tmux_1_6")]
    pub created: Option<Duration>,
    /// String time session created
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
    pub created_string: Option<String>,
    /// 1 if format is for a session (not assuming the current)
    #[cfg(feature = "tmux_2_6")]
    pub format: Option<bool>,
    /// session_group - Name of session group
    #[cfg(feature = "tmux_1_6")]
    pub group: Option<String>,
    /// session_group_attached - Number of clients sessions in group are attached >
    #[cfg(feature = "tmux_3_1")]
    pub group_attached: Option<usize>,
    /// session_group_attached_list - List of clients sessions in group are attached to
    #[cfg(feature = "tmux_3_1")]
    pub group_attached_list: Option<String>,
    /// session_group_list - List of sessions in group
    #[cfg(feature = "tmux_2_7")]
    pub group_list: Option<String>,
    /// session_group_many_attached - 1 if multiple clients attached to sessions in gro
    #[cfg(feature = "tmux_3_1")]
    pub group_many_attached: Option<bool>,
    /// session_size - Size of session group
    #[cfg(feature = "tmux_2_7")]
    pub group_size: Option<String>,
    /// session_grouped - 1 if session in a group
    #[cfg(feature = "tmux_1_6")]
    pub grouped: Option<bool>,
    /// session_height - Height of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub height: Option<usize>,
    /// session_width - Width of session
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
    pub width: Option<usize>,
    /// session_id - Unique session ID
    #[cfg(feature = "tmux_1_8")]
    pub id: Option<usize>,
    /// session_last_attached - Time session last attached
    #[cfg(feature = "tmux_2_1")]
    pub last_attached: Option<Duration>,
    /// session_last_attached_string - String time session last attached
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
    pub last_attached_string: Option<String>,
    /// session_many_attached - 1 if multiple clients attached
    #[cfg(feature = "tmux_2_0")]
    pub many_attached: Option<bool>,
    /// session_name - #S Name of session
    #[cfg(feature = "tmux_1_6")]
    pub name: Option<String>,
    /// session_stack - Window indexes in most recent order
    #[cfg(feature = "tmux_2_5")]
    pub stack: Option<SessionStack>,
    /// session_windows - Number of windows in session
    #[cfg(feature = "tmux_1_6")]
    pub windows: Option<usize>,
}

impl Session {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(s: &str, bitflags: u32) -> Result<Self, Error> {
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
