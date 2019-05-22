use std::time::Duration;
use regex::Regex;
//use super::tmux_interface::SESSION_STR_REGEX;

// in order like: $ man tmux
//session_alerts session_attached session_activity session_created session_format
//session_last_attached session_group session_group_size session_group_list session_grouped
//session_id session_many_attached session_name session_stack session_windows
//
pub const LIST_SESSIONS_FORMAT: &str = "#{session_attached} #{session_activity} \
#{session_created} #{session_last_attached} #{session_id} #{session_name} #{session_windows}";

// FIXME: regex name can be anything, and other keys should be checked better
pub const SESSION_STR_REGEX: &str = r"^(\d+) (\d+) (\d+) (\d+) \$(\d+) (\w+) (\d+)$";

// accordingly to tmux.h: Formats
//
#[derive(Clone, Debug)]
pub struct Session {
    //session_alerts
    pub attached: usize,
    pub activity: Duration,
    pub created: Duration,
    //session_format
    pub last_attached: Duration,
    //session_group
    //session_group_size
    //session_group_list
    //session_grouped
    pub id: usize,
    //session_many_attached
    pub name: String,
    //session_stack
    pub windows: usize,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            attached: 0,
            activity: Duration::from_millis(0),
            created: Duration::from_millis(0),
            last_attached: Duration::from_millis(0),
            id: 0,
            name: "".to_string(),
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
        let cap = regex.captures(session_str).unwrap();
        let mut session = Session::new();
        session.attached = cap[1].parse().unwrap();
        session.activity = Duration::from_millis(cap[2].parse().unwrap());
        session.created = Duration::from_millis(cap[3].parse().unwrap());
        session.last_attached = Duration::from_millis(cap[4].parse().unwrap());
        session.id = cap[5].parse().unwrap();
        session.name = cap[6].parse().unwrap();
        session.windows = cap[7].parse().unwrap();
        Ok(session)
    }


}


pub struct Sessions {
    //sessions: Vec<Session>
}


impl Sessions {

    pub fn parse(sessions_str: &str) -> Result<Vec<Session>, ()> {
        let mut sessions: Vec<Session> = Vec::new();
        for line in sessions_str.lines() {
            sessions.push(Session::parse(line).unwrap());
        }
        Ok(sessions)
    }
}


