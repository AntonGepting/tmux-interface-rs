use crate::Session;


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
