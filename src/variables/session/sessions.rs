use crate::variables::session::session::{SESSION_VARS, SESSION_VARS_SEPARATOR};
use crate::Error;
use crate::ListSessions;
use crate::Session;
use std::ops::Index;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Sessions(pub Vec<Session>);

impl IntoIterator for Sessions {
    type Item = Session;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<usize> for Sessions {
    type Output = Session;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl Sessions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, session: Session) {
        self.0.push(session);
    }

    pub fn get(bitflags: u32) -> Result<Self, Error> {
        let ls_format = SESSION_VARS
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(SESSION_VARS_SEPARATOR);
        //let format = SessionFormat::create(bitflags);
        let output = ListSessions::new().format(&ls_format).output()?;
        let sessions_str = String::from_utf8_lossy(&output.0.stdout.as_slice());
        Sessions::from_str(&sessions_str, bitflags)
    }

    pub fn from_str(sessions_str: &str, bitflags: u32) -> Result<Self, Error> {
        let mut sessions = Sessions::new();
        for line in sessions_str.lines() {
            sessions.push(Session::from_str(line, bitflags)?);
        }
        Ok(sessions)
    }
}
