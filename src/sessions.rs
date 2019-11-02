use crate::session::{SESSION_VARS_REGEX_VEC, SESSION_VARS_SEPARATOR};
use crate::Error;
use crate::Session;
use crate::TmuxInterface;
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
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let ls_format = SESSION_VARS_REGEX_VEC
            .iter()
            .filter(|t| bitflags & t.1 == t.1)
            .map(|t| format!("#{{{}}}", t.0))
            .collect::<Vec<String>>()
            .join(SESSION_VARS_SEPARATOR);
        let sessions_str = tmux.list_sessions(Some(&ls_format))?;
        Sessions::from_str(&sessions_str, bitflags)
    }

    pub fn from_str(sessions_str: &str, bitflags: usize) -> Result<Self, Error> {
        let mut sessions: Vec<Session> = Vec::new();
        for line in sessions_str.lines() {
            sessions.push(Session::from_str(line, bitflags)?);
        }
        Ok(Self(sessions))
    }
}
