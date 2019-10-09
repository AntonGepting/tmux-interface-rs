use crate::Error;
use std::str::FromStr;
use std::time::Duration;

pub const SESSION_VARS_SEPARATOR: &str = ":";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS_REGEX_VEC: [(&str, usize); 15] = [
    ("session_activity", Session::SESSION_ACTIVITY),
    ("session_alerts", Session::SESSION_ALERTS),
    ("session_attached", Session::SESSION_ATTACHED),
    ("session_created", Session::SESSION_CREATED),
    ("session_format", Session::SESSION_FORMAT),
    ("session_group", Session::SESSION_GROUP),
    ("session_group_list", Session::SESSION_GROUP_LIST),
    ("session_group_size", Session::SESSION_GROUP_SIZE),
    ("session_grouped", Session::SESSION_GROUPED),
    ("session_id", Session::SESSION_ID),
    ("session_last_attached", Session::SESSION_LAST_ATTACHED),
    ("session_many_attached", Session::SESSION_MANY_ATTACHED),
    ("session_name", Session::SESSION_NAME),
    ("session_stack", Session::SESSION_STACK),
    ("session_windows", Session::SESSION_WINDOWS),
];

#[derive(Default, PartialEq, Clone, Debug)]
pub struct SessionStack(pub Vec<usize>);

impl FromStr for SessionStack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let a: Vec<usize> = s.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let mut sv = Vec::new();
        for id in s.split(',').collect::<Vec<&str>>() {
            sv.push(id.parse()?);
        }
        Ok(Self(sv))
    }
}

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

impl Session {
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
    pub const SESSION_ALL: usize = Self::SESSION_ACTIVITY
        | Self::SESSION_ATTACHED
        | Self::SESSION_CREATED
        | Self::SESSION_FORMAT
        | Self::SESSION_GROUP
        | Self::SESSION_GROUP_LIST
        | Self::SESSION_GROUP_SIZE
        | Self::SESSION_GROUPED
        | Self::SESSION_ID
        | Self::SESSION_LAST_ATTACHED
        | Self::SESSION_MANY_ATTACHED
        | Self::SESSION_NAME
        | Self::SESSION_STACK
        | Self::SESSION_WINDOWS;

    pub fn new() -> Self {
        Default::default()
    }

    // XXX: mb deserialize?
    // XXX: mb callback
    // XXX: optimize?
    pub fn from_str(s: &str, bitflags: usize) -> Result<Self, Error> {
        let sv: Vec<&str> = s.split(SESSION_VARS_SEPARATOR).collect();
        // XXX: optimize?
        let mut s = Session::new();
        for (i, format_variable) in SESSION_VARS_REGEX_VEC.iter().enumerate() {
            if !sv[i].is_empty() {
                match bitflags & format_variable.1 {
                    Self::SESSION_ACTIVITY => {
                        s.activity = sv[0].parse().ok().map(Duration::from_millis)
                    }
                    Self::SESSION_ALERTS => s.alerts = sv[1].parse().ok(),
                    Self::SESSION_ATTACHED => s.attached = sv[2].parse().ok(),
                    Self::SESSION_CREATED => {
                        s.created = sv[3].parse().ok().map(Duration::from_millis)
                    }
                    Self::SESSION_FORMAT => s.format = sv[4].parse::<usize>().map(|i| i == 1).ok(),
                    Self::SESSION_GROUP => s.group = sv[5].parse().ok(),
                    Self::SESSION_GROUP_LIST => s.group_list = sv[6].parse().ok(),
                    Self::SESSION_GROUP_SIZE => s.group_size = sv[7].parse().ok(),
                    Self::SESSION_GROUPED => {
                        s.grouped = sv[8].parse::<usize>().map(|i| i == 1).ok()
                    }
                    Self::SESSION_ID => s.id = sv[9][1..].parse().ok(), // skip '$' char
                    Self::SESSION_LAST_ATTACHED => {
                        s.last_attached = sv[10].parse().ok().map(Duration::from_millis)
                    }
                    Self::SESSION_MANY_ATTACHED => {
                        s.many_attached = sv[11].parse::<usize>().map(|i| i == 1).ok()
                    }
                    Self::SESSION_NAME => s.name = sv[12].parse().ok(),
                    Self::SESSION_STACK => s.stack = sv[13].parse().ok(),
                    Self::SESSION_WINDOWS => s.windows = sv[14].parse().ok(),
                    _ => (),
                }
            }
        }
        Ok(s)
    }
}
