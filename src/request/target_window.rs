use crate::request::target_session::TargetSession;
use std::fmt;

#[derive(Debug)]
pub struct TargetWindowEx<'a> {
    pub session: Option<TargetSession<'a>>,
    pub window: Option<TargetWindow<'a>>,
}

impl<'a> TargetWindowEx<'a> {
    pub fn new(target_name: &'a str) -> Self {
        TargetWindowEx::start_name(target_name)
    }

    pub fn token(token: TargetWindowToken) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::Token(token)),
        }
    }

    pub fn id(id: usize) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::Id(id)),
        }
    }

    pub fn exact_name(name: &'a str) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::ExactName(name)),
        }
    }

    pub fn start_name(name: &'a str) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::StartName(name)),
        }
    }

    pub fn fn_match(name: &'a str) -> Self {
        TargetWindowEx {
            session: None,
            window: Some(TargetWindow::FnMatch(name)),
        }
    }

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

#[derive(Debug)]
pub enum TargetWindow<'a> {
    Token(TargetWindowToken),
    Index(usize),
    Id(usize),
    ExactName(&'a str),
    StartName(&'a str),
    FnMatch(&'a str),
    Raw(&'a str),
}

impl<'a> fmt::Display for TargetWindow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TargetWindow::Token(token) => token.to_string(),
            TargetWindow::Index(i) => i.to_string(),
            TargetWindow::Id(id) => format!("@{}", id),
            TargetWindow::ExactName(name) => format!("={}", name),
            TargetWindow::StartName(name) => format!("{}", name),
            TargetWindow::FnMatch(name) => format!("{}", name),
            TargetWindow::Raw(raw_str) => format!("{}", raw_str),
        };
        write!(f, ":{}", s)
    }
}

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
