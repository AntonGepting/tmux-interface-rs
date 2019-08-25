use crate::Session;
use crate::TmuxInterface;
use crate::Error;
use crate::session::{SESSION_VARS_REGEX_VEC, SESSION_VARS_SEPARATOR};


pub struct Sessions {
    //sessions: Vec<Session>
}


impl Sessions {

    pub fn get() -> Result<Vec<Session>, Error> {
        let tmux = TmuxInterface::new();
        let ls_format = SESSION_VARS_REGEX_VEC.iter().map(|t| format!("#{{{}}}", t))
            .collect::<Vec<String>>().join(SESSION_VARS_SEPARATOR);
        let sessions_str = tmux.list_sessions(Some(&ls_format))?;
        Sessions::parse(&sessions_str)
    }


    pub fn parse(sessions_str: &str) -> Result<Vec<Session>, Error> {
        let mut sessions: Vec<Session> = Vec::new();
        for line in sessions_str.lines() {
            sessions.push(line.parse()?);
        }
        Ok(sessions)
    }
}
