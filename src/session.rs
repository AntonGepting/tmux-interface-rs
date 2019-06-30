use std::time::Duration;
use regex::Regex;
use crate::TmuxInterfaceError;


pub const SESSION_VARS_SEPARATOR: &str = ":";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS_REGEX_VEC: [(&str, &str); 15] = [
    ("session_alerts",          r"(\w+)?"),
    ("session_attached",        r"(\d+)?"),
    ("session_activity",        r"(\d+)?"),
    ("session_created",         r"(\d+)?"),
    ("session_format",          r"(\w+)?"),
    ("session_last_attached",   r"(\d+)?"),
    ("session_group",           r"(\w+)?"),
    ("session_group_size",      r"(\w+)?"),
    ("session_group_list",      r"(\w+)?"),
    ("session_grouped",         r"(\w+)?"),
    ("session_id",              r"\$(\d+)?"),
    ("session_many_attached",   r"(\w+)?"),
    ("session_name",            r"(\w+)?"),
    ("session_stack",           r"([\w,]*)?"),
    ("session_windows",         r"(\d+)?"),
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
#[derive(Default, Clone, Debug)]
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
        let regex_str = format!("^{}$", SESSION_VARS_REGEX_VEC.iter()
                                .map(|t| t.1).collect::<Vec<&str>>().join(SESSION_VARS_SEPARATOR));
        let regex = Regex::new(&regex_str)?;
        let caps = regex.captures(session_str).unwrap();
        let mut session = Session::new();

        // XXX: optimize?
        if let Some(alerts) = caps.get(1) {
            session.alerts = Some(alerts.as_str().parse()?);
        }
        if let Some(attached) = caps.get(2) {
            session.attached = Some(attached.as_str().parse()?);
        }
        if let Some(activity) = caps.get(3) {
            session.activity = Some(Duration::from_millis(activity.as_str().parse()?));
        }
        if let Some(created) = caps.get(4) {
            session.created = Some(Duration::from_millis(created.as_str().parse()?));
        }
        if let Some(format) = caps.get(5) {
            session.format = Some(format.as_str().parse()?);
        }
        if let Some(last_attached) = caps.get(6) {
            session.last_attached = Some(Duration::from_millis(last_attached.as_str().parse()?));
        }
        if let Some(group) = caps.get(7) {
            session.group = Some(group.as_str().parse()?);
        }
        if let Some(group_size) = caps.get(8) {
            session.group_size = Some(group_size.as_str().parse()?);
        }
        if let Some(group_list) = caps.get(9) {
            session.group_list = Some(group_list.as_str().parse()?);
        }
        if let Some(grouped) = caps.get(10) {
            session.grouped = Some(grouped.as_str().parse()?);
        }
        if let Some(id) = caps.get(11) {
            session.id = Some(id.as_str().parse()?);
        }
        if let Some(many_attached) = caps.get(12) {
            session.many_attached = Some(many_attached.as_str().parse()?);
        }
        if let Some(name) = caps.get(13) {
            session.name = Some(name.as_str().parse()?);
        }
        if let Some(stack) = caps.get(14) {
            session.stack = Some(stack.as_str().parse()?);
        }
        if let Some(windows) = caps.get(15) {
            session.windows = Some(windows.as_str().parse()?);
        }
        Ok(session)
    }
}
