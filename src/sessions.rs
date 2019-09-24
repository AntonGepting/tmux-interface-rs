use crate::session::{SESSION_VARS_REGEX_VEC, SESSION_VARS_SEPARATOR};
use crate::Error;
use crate::Session;
use crate::TmuxInterface;
use std::ops::Index;
use std::str::FromStr;

pub struct Sessions(Vec<Session>);

impl FromStr for Sessions {
    type Err = Error;

    fn from_str(sessions_str: &str) -> Result<Self, Self::Err> {
        let mut sessions: Vec<Session> = Vec::new();
        for line in sessions_str.lines() {
            sessions.push(line.parse()?);
        }
        Ok(Self(sessions))
    }
}

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
    pub fn get() -> Result<Self, Error> {
        let tmux = TmuxInterface::new();
        let ls_format = SESSION_VARS_REGEX_VEC
            .iter()
            .map(|t| format!("#{{{}}}", t))
            .collect::<Vec<String>>()
            .join(SESSION_VARS_SEPARATOR);
        let sessions_str = tmux.list_sessions(Some(&ls_format))?;
        sessions_str.parse()
    }
}
