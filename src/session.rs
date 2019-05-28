use std::time::Duration;
use regex::Regex;
//use super::tmux_interface::SESSION_STR_REGEX;

pub const LIST_SESSIONS_FORMAT: &str = "#{session_alerts}:#{session_attached}:#{session_activity}:\
#{session_created}:#{session_format}:#{session_last_attached}:#{session_id}:#{session_name}:#{session_windows}";


// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_STR_REGEX: &str = r"^(\w+)?:(\d+):(\d+):(\d+):(\w+)?:(\d+):(\w+)?:(\w+)?:(\w+)?:(\w+)?:\$(\d+):(\w+)?:(\w+):([\w,]*):(\d+)$";


// accordingly to tmux.h: Formats
//
#[derive(Clone, Debug)]
pub struct Session {
    pub alerts: Option<String>,
    pub attached: usize,
    pub activity: Duration,
    pub created: Duration,
    pub format: Option<String>,
    pub last_attached: Duration,
    pub group: Option<String>,
    pub group_size: Option<String>,
    pub group_list: Option<String>,
    pub grouped: Option<String>,
    pub id: usize,
    pub many_attached: Option<String>,
    pub name: String,
    pub stack: String,
    pub windows: usize,
}


impl Default for Session {
    fn default() -> Self {
        Session {
            alerts: None,
            attached: 0,
            activity: Duration::from_millis(0),
            created: Duration::from_millis(0),
            format: None,
            last_attached: Duration::from_millis(0),
            group: None,
            group_size: None,
            group_list: None,
            grouped: None,
            id: 0,
            many_attached: None,
            name: "".to_string(),
            stack: "".to_string(),
            windows: 0,
        }
    }
}


impl Session {
    pub fn new() -> Session {
        Default::default()
    }

    // XXX: mb deserialize?
    //
    pub fn parse(session_str: &str) -> Result<Session, ()> {
        let regex = Regex::new(SESSION_STR_REGEX).unwrap();
        let caps = regex.captures(session_str).unwrap();
        let mut session = Session::new();
        // XXX: optimize?
        if let Some(alerts) = caps.get(1) {
            session.alerts = Some(alerts.as_str().parse().unwrap());
        }
        session.attached = caps[2].parse().unwrap();
        session.activity = Duration::from_millis(caps[3].parse().unwrap());
        session.created = Duration::from_millis(caps[4].parse().unwrap());
        if let Some(format) = caps.get(5) {
            session.format = Some(format.as_str().parse().unwrap());
        }
        session.last_attached = Duration::from_millis(caps[6].parse().unwrap());
        if let Some(group) = caps.get(7) {
            session.group = Some(group.as_str().parse().unwrap());
        }
        if let Some(group_size) = caps.get(8) {
            session.group_size = Some(group_size.as_str().parse().unwrap());
        }
        if let Some(group_list) = caps.get(9) {
            session.group_list = Some(group_list.as_str().parse().unwrap());
        }
        if let Some(grouped) = caps.get(10) {
            session.grouped = Some(grouped.as_str().parse().unwrap());
        }
        session.id = caps[11].parse().unwrap();
        if let Some(many_attached) = caps.get(12) {
            session.many_attached = Some(many_attached.as_str().parse().unwrap());
        }
        session.name = caps[13].parse().unwrap();
        if let Some(stack) = caps.get(14) {
            session.stack = stack.as_str().parse().unwrap();
        }
        session.windows = caps[15].parse().unwrap();
        Ok(session)
    }
}
