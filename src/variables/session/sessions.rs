use crate::{Error, Session};
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

    //let sessions_str = String::from_utf8_lossy(&output.0.stdout.as_slice());
    //Sessions::from_str(&sessions_str, bitflags)
    //}

    pub fn from_str<S: AsRef<str>>(sessions_str: S) -> Result<Self, Error> {
        let mut sessions = Sessions::new();
        for line in sessions_str.as_ref().lines() {
            sessions.push(Session::from_str(line)?);
        }
        Ok(sessions)
    }
}

//#[cfg(feature = "tmux_2_1")]
//format.session_activity(&mut session.activity);
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
//format.session_activity_string(&mut session.activity_string);
//#[cfg(feature = "tmux_2_1")]
//format.session_alerts(&mut session.alerts);
//#[cfg(feature = "tmux_1_6")]
//format.session_attached(&mut session.attached);
//#[cfg(feature = "tmux_3_1")]
//format.session_attached_list(&mut session.attached_list);
//#[cfg(feature = "tmux_1_6")]
//format.session_created(&mut session.created);
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
//format.session_created_string(&mut session.created_string);
//#[cfg(feature = "tmux_2_6")]
//format.session_format(&mut session.format);
//#[cfg(feature = "tmux_1_6")]
//format.session_group(&mut session.group);
//#[cfg(feature = "tmux_3_1")]
//format.session_group_attached(&mut session.group_attached);
//#[cfg(feature = "tmux_3_1")]
//format.session_group_attached_list(&mut session.group_attached_list);
//#[cfg(feature = "tmux_2_7")]
//format.session_group_list(&mut session.group_list);
//#[cfg(feature = "tmux_3_1")]
//format.session_group_many_attached(&mut session.group_many_attached);
//#[cfg(feature = "tmux_2_7")]
//format.session_group_size(&mut session.group_size);
//#[cfg(feature = "tmux_1_6")]
//format.session_grouped(&mut session.grouped);
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
//format.session_height(&mut session.height);
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
//format.session_width(&mut session.width);
//#[cfg(feature = "tmux_1_8")]
//format.session_id(&mut session.id);
//#[cfg(feature = "tmux_2_1")]
//format.session_last_attached(&mut session.last_attached);
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
//format.session_last_attached_string(&mut session.last_attached_string);
//#[cfg(feature = "tmux_2_0")]
//format.session_many_attached(&mut session.many_attached);
//#[cfg(feature = "tmux_1_6")]
//format.session_name(&mut session.name);
//#[cfg(feature = "tmux_2_5")]
//format.session_stack(&mut session.stack);
//#[cfg(feature = "tmux_1_6")]
//format.session_windows(&mut session.windows);
