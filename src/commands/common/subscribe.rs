// ```
// tmux >=3.2:
// ```
//
// used by commands:
//  * refresh-client

use std::borrow::Cow;
use std::fmt;

// TODO: enum for what?
/// [-B name:what:format]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Subscribe<'a> {
    pub name: Cow<'a, str>,
    /// empty to check the format only for the attached session
    /// pane ID such as ‘%0’; ‘%*’ for all panes in the attached session
    /// window ID such as ‘@0’; or ‘@*’ for all windows in the attached session
    pub what: Option<usize>,
    pub format: Option<usize>,
}

impl<'a> fmt::Display for Subscribe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        v.push(self.name.to_string());
        if let Some(what) = self.what {
            v.push(what.to_string());
        }
        if let Some(format) = self.format {
            v.push(format.to_string());
        }
        let s = v.join(":");
        write!(f, "{}", s)
    }
}
