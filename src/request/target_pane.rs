use crate::request::target_window::TargetWindowEx;
use std::fmt;

pub struct TargetPaneEx<'a> {
    pub target_window: Option<TargetWindowEx<'a>>,
    pub target_pane: Option<TargetPane<'a>>,
}

impl<'a> TargetPaneEx<'a> {
    pub fn new(target_name: &'a str) -> Self {
        TargetPaneEx::start_name(target_name)
    }

    pub fn token(token: TargetPaneToken) -> Self {
        TargetPaneEx {
            target_window: None,
            target_pane: Some(TargetPane::Token(token)),
        }
    }

    pub fn id(id: usize) -> Self {
        TargetPaneEx {
            target_window: None,
            target_pane: Some(TargetPane::Id(id)),
        }
    }

    pub fn exact_name(name: &'a str) -> Self {
        TargetPaneEx {
            target_window: None,
            target_pane: Some(TargetPane::ExactName(name)),
        }
    }

    pub fn start_name(name: &'a str) -> Self {
        TargetPaneEx {
            target_window: None,
            target_pane: Some(TargetPane::StartName(name)),
        }
    }

    pub fn fn_match(name: &'a str) -> Self {
        TargetPaneEx {
            target_window: None,
            target_pane: Some(TargetPane::FnMatch(name)),
        }
    }

    pub fn raw(name: &'a str) -> Self {
        TargetPaneEx {
            target_window: None,
            target_pane: Some(TargetPane::Raw(name)),
        }
    }
}

impl<'a> fmt::Display for TargetPaneEx<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = String::new();
        let mut p = String::new();
        if let Some(ref target_window) = self.target_window {
            w = target_window.to_string();
        }
        if let Some(ref target_pane) = self.target_pane {
            p = target_pane.to_string();
        }
        write!(f, "{}{}", w, p)
    }
}

pub enum TargetPane<'a> {
    Token(TargetPaneToken),
    Index(usize),
    Id(usize),
    ExactName(&'a str),
    StartName(&'a str),
    FnMatch(&'a str),
    Raw(&'a str),
}

impl<'a> fmt::Display for TargetPane<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TargetPane::Token(token) => token.to_string(),
            TargetPane::Index(i) => i.to_string(),
            TargetPane::Id(id) => format!("%{}", id),
            TargetPane::ExactName(name) => format!("={}", name),
            TargetPane::StartName(name) => format!("{}", name),
            TargetPane::FnMatch(name) => format!("{}", name),
            TargetPane::Raw(raw_str) => format!("{}", raw_str),
        };
        write!(f, ".{}", s)
    }
}

pub enum TargetPaneToken {
    /// {next} + The next pane by number
    Next(Option<usize>),
    /// {previous} - The previous pane by number
    Previous(Option<usize>),
    /// {top} The top pane
    Top,
    /// {bottom} The bottom pane
    Bottom,
    /// {left} The leftmost pane
    Left,
    /// {right} The rightmost pane
    Right,
    /// {top-left} The top-left pane
    TopLeft,
    /// {top-right} The top-right pane
    TopRight,
    /// {bottom-left} The bottom-left pane
    BottomLeft,
    /// {bottom-right} The bottom-right pane
    BottomRight,
    /// {up-of} The pane above the active pane
    UpOf,
    /// {down-of} The pane below the active pane
    DownOf,
    /// {left-of} The pane to the left of the active pane
    LeftOf,
    /// {right-of} The pane to the right of the active pane
    RightOf,
    //// {mouse} = most recent mouse event occured
    //Mouse,{last}            !    The last (previously active) pane
}

const TARGET_PANE_TOKEN_NEXT: &'static str = "+"; // {next}
const TARGET_PANE_TOKEN_PREVIOUS: &'static str = "-"; // {previous}
const TARGET_PANE_TOKEN_TOP: &'static str = "{top}"; // {top}
const TARGET_PANE_TOKEN_BOTTOM: &'static str = "{bottom}"; // {bottom}
const TARGET_PANE_TOKEN_LEFT: &'static str = "{left}"; // {left}
const TARGET_PANE_TOKEN_RIGHT: &'static str = "{right}"; // {right}
const TARGET_PANE_TOKEN_TOP_LEFT: &'static str = "{top-left}"; // {top-left}
const TARGET_PANE_TOKEN_TOP_RIGHT: &'static str = "{top-right}"; // {top-right}
const TARGET_PANE_TOKEN_BOTTOM_LEFT: &'static str = "{bottom-left}"; // {bottom-left}
const TARGET_PANE_TOKEN_BOTTOM_RIGHT: &'static str = "{bottom-right}"; // {bottom-right}
const TARGET_PANE_TOKEN_UP_OF: &'static str = "{up-of}"; // {up-of}
const TARGET_PANE_TOKEN_DOWN_OF: &'static str = "{down-of}"; // {down-of}
const TARGET_PANE_TOKEN_LEFT_OF: &'static str = "{left-of}"; // {left-of}
const TARGET_PANE_TOKEN_RIGHT_OF: &'static str = "{right-of}"; // {right-of}

impl fmt::Display for TargetPaneToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a;
        let s = match self {
            TargetPaneToken::Next(offset) => {
                if let Some(n) = offset {
                    a = format!("{}{}", TARGET_PANE_TOKEN_NEXT, n);
                    &a
                } else {
                    TARGET_PANE_TOKEN_NEXT
                }
            }
            TargetPaneToken::Previous(offset) => {
                if let Some(n) = offset {
                    a = format!("{}{}", TARGET_PANE_TOKEN_PREVIOUS, n);
                    &a
                } else {
                    TARGET_PANE_TOKEN_PREVIOUS
                }
            }
            TargetPaneToken::Top => TARGET_PANE_TOKEN_TOP,
            TargetPaneToken::Bottom => TARGET_PANE_TOKEN_BOTTOM,
            TargetPaneToken::Left => TARGET_PANE_TOKEN_LEFT,
            TargetPaneToken::Right => TARGET_PANE_TOKEN_RIGHT,
            TargetPaneToken::TopLeft => TARGET_PANE_TOKEN_TOP_LEFT,
            TargetPaneToken::TopRight => TARGET_PANE_TOKEN_TOP_RIGHT,
            TargetPaneToken::BottomLeft => TARGET_PANE_TOKEN_BOTTOM_LEFT,
            TargetPaneToken::BottomRight => TARGET_PANE_TOKEN_BOTTOM_RIGHT,
            TargetPaneToken::UpOf => TARGET_PANE_TOKEN_UP_OF,
            TargetPaneToken::DownOf => TARGET_PANE_TOKEN_DOWN_OF,
            TargetPaneToken::LeftOf => TARGET_PANE_TOKEN_LEFT_OF,
            TargetPaneToken::RightOf => TARGET_PANE_TOKEN_RIGHT_OF,
        };
        f.write_str(s)
    }
}
