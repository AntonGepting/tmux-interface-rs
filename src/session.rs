use std::time::Duration;
use crate::Error;
use std::str::FromStr;


pub const SESSION_VARS_SEPARATOR: &str = ":";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS_REGEX_VEC: [&str; 15] = [
    "session_activity",
    "session_alerts",
    "session_attached",
    "session_created",
    "session_format",
    "session_group",
    "session_group_list",
    "session_group_size",
    "session_grouped",
    "session_id",
    "session_last_attached",
    "session_many_attached",
    "session_name",
    "session_stack",
    "session_windows",
];


#[derive(Default, PartialEq, Clone, Debug)]
pub struct SessionStack(pub Vec<usize>);


impl FromStr for SessionStack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let a: Vec<usize> = s.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let mut sv = Vec::new();
        for id in s.split(",").collect::<Vec<&str>>() {
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


impl FromStr for Session {
    type Err = Error;

    // XXX: mb deserialize?
    // XXX: mb callback
    // XXX: optimize?
    fn from_str(s: &str) -> Result<Session, Error> {
        let sv: Vec<&str> = s.split(SESSION_VARS_SEPARATOR).collect();
        let mut s = Session::new();
        if !sv[0].is_empty() { s.activity = sv[0].parse().ok().map(Duration::from_millis); }
        if !sv[1].is_empty() { s.alerts = sv[1].parse().ok(); }
        if !sv[2].is_empty() { s.attached = sv[2].parse().ok(); }
        if !sv[3].is_empty() { s.created = sv[3].parse().ok().map(Duration::from_millis); }
        if !sv[4].is_empty() { s.format = sv[4].parse::<usize>().map(|i| i == 1).ok(); }
        if !sv[5].is_empty() { s.group = sv[5].parse().ok(); }
        if !sv[6].is_empty() { s.group_list = sv[6].parse().ok(); }
        if !sv[7].is_empty() { s.group_size = sv[7].parse().ok(); }
        if !sv[8].is_empty() { s.grouped = sv[8].parse::<usize>().map(|i| i == 1).ok(); }
        if !sv[9].is_empty() { s.id = sv[9][1..].parse().ok(); } // skip '$' char
        if !sv[10].is_empty() { s.last_attached = sv[10].parse().ok().map(Duration::from_millis); }
        if !sv[11].is_empty() { s.many_attached = sv[11].parse::<usize>().map(|i| i == 1).ok(); }
        if !sv[12].is_empty() { s.name = sv[12].parse().ok(); }
        if !sv[13].is_empty() { s.stack = sv[13].parse().ok(); }
        if !sv[14].is_empty() { s.windows = sv[14].parse().ok(); }
        Ok(s)
    }
}


impl Session {

    pub fn new() -> Self {
        Default::default()
    }

}
