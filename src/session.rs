use crate::Error;
use crate::SessionStack;
use std::time::Duration;

pub const SESSION_VARS_SEPARATOR: &str = ":";
// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS_REGEX_VEC: [(&str, usize); 15] = [
    ("session_activity", SESSION_ACTIVITY),
    ("session_alerts", SESSION_ALERTS),
    ("session_attached", SESSION_ATTACHED),
    ("session_created", SESSION_CREATED),
    ("session_format", SESSION_FORMAT),
    ("session_group", SESSION_GROUP),
    ("session_group_list", SESSION_GROUP_LIST),
    ("session_group_size", SESSION_GROUP_SIZE),
    ("session_grouped", SESSION_GROUPED),
    ("session_id", SESSION_ID),
    ("session_last_attached", SESSION_LAST_ATTACHED),
    ("session_many_attached", SESSION_MANY_ATTACHED),
    ("session_name", SESSION_NAME),
    ("session_stack", SESSION_STACK),
    ("session_windows", SESSION_WINDOWS),
];

// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Session {
    /// Time of session last activity
    pub activity: Option<Duration>,
    /// List of window indexes with alerts
    pub alerts: Option<String>,
    /// Number of clients session is attached to
    pub attached: Option<usize>,
    /// Time session created
    pub created: Option<Duration>,
    /// 1 if format is for a session (not assuming the current)
    pub format: Option<bool>,
    /// Name of session group
    pub group: Option<String>,
    /// List of sessions in group
    pub group_list: Option<String>,
    /// Size of session group
    pub group_size: Option<String>,
    /// 1 if session in a group
    pub grouped: Option<bool>,
    /// Unique session ID
    pub id: Option<usize>,
    /// Time session last attached
    pub last_attached: Option<Duration>,
    /// 1 if multiple clients attached
    pub many_attached: Option<bool>,
    /// #S Name of session
    pub name: Option<String>,
    /// Window indexes in most recent order
    pub stack: Option<SessionStack>,
    /// Number of windows in session
    pub windows: Option<usize>,
}

// NOTE: u16 mb not enough!
pub const SESSION_ACTIVITY: usize = 1 << 0;
pub const SESSION_ALERTS: usize = 1 << 1;
pub const SESSION_ATTACHED: usize = 1 << 2;
pub const SESSION_CREATED: usize = 1 << 3;
pub const SESSION_FORMAT: usize = 1 << 4;
pub const SESSION_GROUP: usize = 1 << 5;
pub const SESSION_GROUP_LIST: usize = 1 << 6;
pub const SESSION_GROUP_SIZE: usize = 1 << 7;
pub const SESSION_GROUPED: usize = 1 << 8;
pub const SESSION_ID: usize = 1 << 9;
pub const SESSION_LAST_ATTACHED: usize = 1 << 10;
pub const SESSION_MANY_ATTACHED: usize = 1 << 11;
pub const SESSION_NAME: usize = 1 << 12;
pub const SESSION_STACK: usize = 1 << 13;
pub const SESSION_WINDOWS: usize = 1 << 14;

pub const SESSION_NONE: usize = 0;
//pub const SESSION_DEFAULT: usize = SESSION_ID | SESSION_NAME;
pub const SESSION_ALL: usize = SESSION_ACTIVITY
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

impl Session {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(s: &str, bitflags: usize) -> Result<Self, Error> {
        let sv: Vec<&str> = s.split(SESSION_VARS_SEPARATOR).collect();
        let mut sv = sv.iter();
        // XXX: optimize?
        let mut s = Session::new();
        // for all bitflags
        for var in SESSION_VARS_REGEX_VEC.iter() {
            let bitflag = bitflags & var.1;
            // is current bitflag given?
            if bitflag == var.1 {
                // does vector element exist?
                if let Some(part) = sv.next() {
                    // is vector element not empty
                    if !part.is_empty() {
                        // decode it and save as struct field
                        match bitflag {
                            SESSION_ACTIVITY => {
                                s.activity = part.parse().ok().map(Duration::from_millis)
                            }
                            SESSION_ALERTS => s.alerts = part.parse().ok(),
                            SESSION_ATTACHED => s.attached = part.parse().ok(),
                            SESSION_CREATED => {
                                s.created = part.parse().ok().map(Duration::from_millis)
                            }
                            SESSION_FORMAT => s.format = part.parse::<usize>().map(|i| i == 1).ok(),
                            SESSION_GROUP => s.group = part.parse().ok(),
                            SESSION_GROUP_LIST => s.group_list = part.parse().ok(),
                            SESSION_GROUP_SIZE => s.group_size = part.parse().ok(),
                            SESSION_GROUPED => {
                                s.grouped = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            SESSION_ID => s.id = part[1..].parse().ok(), // skip '$' char
                            SESSION_LAST_ATTACHED => {
                                s.last_attached = part.parse().ok().map(Duration::from_millis)
                            }
                            SESSION_MANY_ATTACHED => {
                                s.many_attached = part.parse::<usize>().map(|i| i == 1).ok()
                            }
                            SESSION_NAME => s.name = part.parse().ok(),
                            SESSION_STACK => s.stack = part.parse().ok(),
                            SESSION_WINDOWS => s.windows = part.parse().ok(),
                            // else?
                            _ => (),
                        }
                    }
                }
            }
        }
        Ok(s)
    }
}
