// ```
// tmux >=3.2:
// ```
//
// used by commands:
//  * refresh-client

use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use crate::state::ParseStateError;
use crate::State;

// XXX: struct or tuple?
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AllowActions<'a> {
    pub pane: Cow<'a, str>,
    pub state: State,
}

// convert struct to `[pane]:[state]` string
impl<'a> fmt::Display for AllowActions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{}:{}", self.pane, self.state);
        write!(f, "{}", s)
    }
}

// XXX: error propagation
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ParseAllowActionsError;

impl From<ParseStateError> for ParseAllowActionsError {
    fn from(err: ParseStateError) -> Self {
        ParseAllowActionsError
    }
}

// convert `[pane]:[state]` str to struct
impl<'a> FromStr for AllowActions<'a> {
    type Err = ParseAllowActionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, ':').collect();
        if v.len() == 2 {
            let pane = Cow::Owned(v[0].to_string());
            let state = v[1].parse()?;
            let allow_actions = AllowActions { pane, state };
            Ok(allow_actions)
        } else {
            Err(ParseAllowActionsError)
        }
    }
}
