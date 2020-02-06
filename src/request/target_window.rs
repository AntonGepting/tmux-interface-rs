use crate::request::target_session::TargetSession;
use std::fmt;

// XXX: borrowing/owning?
/// Extended `target-window` struct, includes `target-session`
#[derive(Debug, Default)]
pub struct TargetWindowEx<'a> {
    /// `target-session`
    pub session: Option<&'a TargetSession<'a>>,
    /// `target-window`
    pub window: Option<TargetWindow<'a>>, // bc. cant return value referencing local / temp value
}

impl<'a> TargetWindowEx<'a> {
    /// simple initializing as start of a name
    pub fn new(target_window: &'a str) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::StartName(target_window)),
        }
    }

    pub fn token(session: Option<&'a TargetSession<'a>>, token: TargetWindowToken) -> Self {
        TargetWindowEx {
            session: session,
            window: Some(TargetWindow::Token(token)),
        }
    }

    pub fn index(session: Option<&'a TargetSession<'a>>, i: usize) -> Self {
        TargetWindowEx {
            session: session,
            window: Some(TargetWindow::Index(i)),
        }
    }

    pub fn id(session: Option<&'a TargetSession<'a>>, id: usize) -> Self {
        TargetWindowEx {
            session: session,
            window: Some(TargetWindow::Id(id)),
        }
    }

    pub fn exact_name(session: Option<&'a TargetSession<'a>>, name: &'a str) -> Self {
        TargetWindowEx {
            session: session,
            window: Some(TargetWindow::ExactName(name)),
        }
    }

    pub fn start_name(session: Option<&'a TargetSession<'a>>, name: &'a str) -> Self {
        TargetWindowEx {
            session: session,
            window: Some(TargetWindow::StartName(name)),
        }
    }

    pub fn fn_match(session: Option<&'a TargetSession<'a>>, name: &'a str) -> Self {
        TargetWindowEx {
            session: session,
            window: Some(TargetWindow::FnMatch(name)),
        }
    }

    // XXX: draft $1:@raw_name or .raw_name or raw_name:raw_name?
    pub fn raw(name: &'a str) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::Raw(name)),
        }
    }
}

impl<'a> fmt::Display for TargetWindowEx<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut w = String::new();
        if let Some(ref session) = self.session {
            s = session.to_string();
        }
        if let Some(ref window) = self.window {
            w = window.to_string();
        }
        write!(f, "{}{}", s, w)
    }
}

/// Enum for possible `target-window` variants
#[derive(Debug)]
pub enum TargetWindow<'a> {
    /// token (^, $, !, +, -) instead of name
    Token(TargetWindowToken),
    /// index instead of name
    Index(usize),
    /// id (@id) instead of name
    Id(usize),
    /// exact name (=name)
    ExactName(&'a str),
    /// start of a name
    StartName(&'a str),
    /// fn_match
    FnMatch(&'a str),
    /// manual define full name (no `:` will be added)
    Raw(&'a str),
}

impl<'a> Default for TargetWindow<'a> {
    fn default() -> Self {
        TargetWindow::Raw("")
    }
}

// TODO: extract simple name, simple parent name
impl<'a> fmt::Display for TargetWindow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetWindow::Token(token) => write!(f, ":{}", token),
            TargetWindow::Index(i) => write!(f, ":{}", i),
            TargetWindow::Id(id) => write!(f, ":@{}", id),
            TargetWindow::ExactName(name) => write!(f, ":={}", name),
            TargetWindow::StartName(name) => write!(f, ":{}", name),
            TargetWindow::FnMatch(name) => write!(f, ":{}", name),
            TargetWindow::Raw(raw_str) => write!(f, "{}", raw_str),
        }
    }
}

/// Enum for `target-window` tokens
#[derive(Debug)]
pub enum TargetWindowToken {
    /// {start} ^ The lowest-numbered window
    Start,
    /// {end} $ The highest-numbered window
    End,
    /// {last} ! The last (previously current) window
    Last,
    /// {next} + The next window by number
    Next(Option<usize>),
    /// {previous} - The previous window by number
    Previous(Option<usize>),
    //// {mouse} = most recent mouse event occured
    //Mouse,
}

const TARGET_WINDOW_TOKEN_START: &'static str = "^"; // {start}
const TARGET_WINDOW_TOKEN_END: &'static str = "$"; // {end}
const TARGET_WINDOW_TOKEN_LAST: &'static str = "!"; // {last}
const TARGET_WINDOW_TOKEN_NEXT: &'static str = "+"; // {next}
const TARGET_WINDOW_TOKEN_PREVIOUS: &'static str = "-"; // {previous}

impl fmt::Display for TargetWindowToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a;
        let s = match self {
            TargetWindowToken::Start => TARGET_WINDOW_TOKEN_START,
            TargetWindowToken::End => TARGET_WINDOW_TOKEN_END,
            TargetWindowToken::Last => TARGET_WINDOW_TOKEN_LAST,
            TargetWindowToken::Next(offset) => {
                if let Some(n) = offset {
                    a = format!("{}{}", TARGET_WINDOW_TOKEN_NEXT, n);
                    &a
                } else {
                    TARGET_WINDOW_TOKEN_NEXT
                }
            }
            TargetWindowToken::Previous(offset) => {
                if let Some(n) = offset {
                    a = format!("{}{}", TARGET_WINDOW_TOKEN_PREVIOUS, n);
                    &a
                } else {
                    TARGET_WINDOW_TOKEN_PREVIOUS
                }
            }
        };
        f.write_str(s)
    }
}
