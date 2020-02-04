use std::fmt;

impl<'a> TargetSession<'a> {
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

    pub fn raw(name: &'a str) -> Self {
        TargetSession::Raw(name)
    }
}

#[derive(Debug)]
pub enum TargetSession<'a> {
    Id(usize),
    ExactName(&'a str),
    StartName(&'a str),
    FnMatch(&'a str),
    Raw(&'a str),
}

impl<'a> fmt::Display for TargetSession<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TargetSession::Id(id) => format!("${}", id),
            TargetSession::ExactName(name) => format!("={}", name),
            TargetSession::StartName(name) => name.to_string(),
            TargetSession::FnMatch(name) => name.to_string(),
            TargetSession::Raw(name) => name.to_string(),
        };
        write!(f, "{}", s)
    }
}
