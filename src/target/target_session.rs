use std::fmt;

impl<'a> TargetSession<'a> {
    /// simple initializing as start of a name
    pub fn new(target_name: &'a str) -> Self {
        TargetSession::StartName(target_name)
    }

    pub fn exact_name(name: &'a str) -> Self {
        TargetSession::ExactName(name)
    }

    pub fn start_name(name: &'a str) -> Self {
        TargetSession::StartName(name)
    }

    pub fn fn_match(name: &'a str) -> Self {
        TargetSession::FnMatch(name)
    }

    // XXX: draft
    pub fn raw(name: &'a str) -> Self {
        TargetSession::Raw(name)
    }
}

// XXX: remove unnecessary fields
// XXX: mb impl Into<String>, generics?
/// Enum for possible `target-session` variants
#[derive(Debug)]
pub enum TargetSession<'a> {
    /// id ($id) instead of name
    Id(usize),
    /// exact name (=name)
    ExactName(&'a str),
    /// start of a name
    StartName(&'a str),
    /// fn_match
    FnMatch(&'a str),
    // NOTE: not really needed, just to be analogous to `TargetWindow` `TargetPane`
    /// manual define full name
    Raw(&'a str),
}

impl<'a> fmt::Display for TargetSession<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetSession::Id(id) => write!(f, "${}", id),
            TargetSession::ExactName(name) => write!(f, "={}", name),
            TargetSession::StartName(name) => f.write_str(name),
            TargetSession::FnMatch(name) => f.write_str(name),
            TargetSession::Raw(name) => f.write_str(name),
        }
    }
}
