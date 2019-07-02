use std::time::Duration;
use crate::TmuxInterfaceError;


pub const SESSION_VARS_SEPARATOR: &str = ":";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS_REGEX_VEC: [&str; 15] = [
    "session_alerts",
    "session_attached",
    "session_activity",
    "session_created",
    "session_format",
    "session_last_attached",
    "session_group",
    "session_group_size",
    "session_group_list",
    "session_grouped",
    "session_id",
    "session_many_attached",
    "session_name",
    "session_stack",
    "session_windows",
];


//struct asdf<'a> {
    //vec: Vec<(&'a str, &'a str)>,
    //separator: &'a str
//}


//impl<'a> asdf<'a> {
    //fn new(vec: &Vec<(&str, &str)>, separator: &str) {
    //}


    //fn get_format(&self) -> String {
        //self.vec.iter()
            //.map(|t| format!("#{{{}}}", t.0))
            //.collect::<Vec<String>>()
            //.join(self.separator)
    //}


    //fn get_regex(&self) -> String {
        //format!("^{}$", self.vec.iter()
                //.map(|t| t.1)
                //.collect::<Vec<&str>>()
                //.join(self.separator)
        //)
    //}
//}


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Session {
    pub alerts: Option<String>,
    pub attached: Option<usize>,
    pub activity: Option<Duration>,
    pub created: Option<Duration>,
    pub format: Option<String>,
    pub last_attached: Option<Duration>,
    pub group: Option<String>,
    pub group_size: Option<String>,
    pub group_list: Option<String>,
    pub grouped: Option<String>,
    pub id: Option<usize>,
    pub many_attached: Option<String>,
    pub name: Option<String>,
    pub stack: Option<String>,
    pub windows: Option<usize>,
}


impl Session {

    pub fn new() -> Self {
        Default::default()
    }


    // XXX: mb deserialize?
    // XXX: mb callback
    pub fn parse(session_str: &str) -> Result<Session, TmuxInterfaceError> {
        let sv: Vec<&str> = session_str.split(SESSION_VARS_SEPARATOR).collect();
        let mut s = Session::new();
        // XXX: optimize?
        if !sv[0].is_empty() { s.alerts = sv[0].parse().ok(); }
        if !sv[1].is_empty() { s.attached = sv[1].parse().ok(); }
        if !sv[2].is_empty() { s.activity = sv[2].parse().ok().map(Duration::from_millis); }
        if !sv[3].is_empty() { s.created = sv[3].parse().ok().map(Duration::from_millis); }
        if !sv[4].is_empty() { s.format = sv[4].parse().ok(); }
        if !sv[5].is_empty() { s.last_attached = sv[5].parse().ok().map(Duration::from_millis); }
        if !sv[6].is_empty() { s.group = sv[6].parse().ok(); }
        if !sv[7].is_empty() { s.group_size = sv[7].parse().ok(); }
        if !sv[8].is_empty() { s.group_list = sv[8].parse().ok(); }
        if !sv[9].is_empty() { s.grouped = sv[9].parse().ok(); }
        if !sv[10].is_empty() { s.id = sv[10][1..].parse().ok(); } // skip '$' char
        if !sv[11].is_empty() { s.many_attached = sv[11].parse().ok(); }
        if !sv[12].is_empty() { s.name = sv[12].parse().ok(); }
        if !sv[13].is_empty() { s.stack = sv[13].parse().ok(); }
        if !sv[14].is_empty() { s.windows = sv[14].parse().ok(); }
        Ok(s)
    }

}
