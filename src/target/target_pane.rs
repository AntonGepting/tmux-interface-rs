use super::target_window::TargetWindowExt;
use std::fmt;

/// Extended `target-pane` struct, includes `target-window` (may indirect include `target-session`)
#[derive(Default)]
pub struct TargetPaneExt<'a> {
    /// `target-window`
    pub target_window: Option<&'a TargetWindowExt<'a>>,
    /// `target-pane`
    pub target_pane: Option<TargetPane<'a>>,
}

impl<'a> TargetPaneExt<'a> {
    /// simple initializing as start of a name
    pub fn new(target_pane: &'a str) -> Self {
        TargetPaneExt {
            target_window: None,
            target_pane: Some(TargetPane::StartName(target_pane)),
        }
    }

    pub fn token(target_window: Option<&'a TargetWindowExt>, token: TargetPaneToken) -> Self {
        TargetPaneExt {
            target_window,
            target_pane: Some(TargetPane::Token(token)),
        }
    }

    pub fn index(target_window: Option<&'a TargetWindowExt>, index: usize) -> Self {
        TargetPaneExt {
            target_window,
            target_pane: Some(TargetPane::Index(index)),
        }
    }

    pub fn id(target_window: Option<&'a TargetWindowExt>, id: usize) -> Self {
        TargetPaneExt {
            target_window,
            target_pane: Some(TargetPane::Id(id)),
        }
    }

    pub fn exact_name(target_window: Option<&'a TargetWindowExt>, name: &'a str) -> Self {
        TargetPaneExt {
            target_window,
            target_pane: Some(TargetPane::ExactName(name)),
        }
    }

    pub fn start_name(target_window: Option<&'a TargetWindowExt>, name: &'a str) -> Self {
        TargetPaneExt {
            target_window,
            target_pane: Some(TargetPane::StartName(name)),
        }
    }

    pub fn fn_match(target_window: Option<&'a TargetWindowExt>, name: &'a str) -> Self {
        TargetPaneExt {
            target_window,
            target_pane: Some(TargetPane::FnMatch(name)),
        }
    }

    // XXX: draft $1:@2.raw_name or .raw_name or raw_name:raw_name.raw_name?
    pub fn raw(name: &'a str) -> Self {
        TargetPaneExt {
            target_window: None,
            target_pane: Some(TargetPane::Raw(name)),
        }
    }
}

impl<'a> fmt::Display for TargetPaneExt<'a> {
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

/// Enum for possible `target-pane` variants
//#[derive(Default)]
pub enum TargetPane<'a> {
    /// token (+, -, {...}) instead of name
    Token(TargetPaneToken),
    /// index instead of name
    Index(usize),
    /// id (%id) instead of name
    Id(usize),
    /// exact name (=name)
    ExactName(&'a str),
    /// start of a name
    StartName(&'a str),
    /// fn_match
    FnMatch(&'a str),
    /// manual define full name (no `.` will be added)
    Raw(&'a str),
}

impl<'a> Default for TargetPane<'a> {
    fn default() -> Self {
        TargetPane::Raw("")
    }
}

impl<'a> fmt::Display for TargetPane<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetPane::Token(token) => write!(f, ".{}", token),
            TargetPane::Index(i) => write!(f, ".{}", i),
            TargetPane::Id(id) => write!(f, ".%{}", id),
            TargetPane::ExactName(name) => write!(f, ".={}", name),
            TargetPane::StartName(name) => write!(f, ".{}", name),
            TargetPane::FnMatch(name) => write!(f, ".{}", name),
            TargetPane::Raw(raw_str) => write!(f, "{}", raw_str),
        }
    }
}

/// Enum for `target-pane` tokens
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

// {next}
const TARGET_PANE_TOKEN_NEXT: &str = "+";
// {previous}
const TARGET_PANE_TOKEN_PREVIOUS: &str = "-";
// {top}
const TARGET_PANE_TOKEN_TOP: &str = "{top}";
// {bottom}
const TARGET_PANE_TOKEN_BOTTOM: &str = "{bottom}";
// {left}
const TARGET_PANE_TOKEN_LEFT: &str = "{left}";
// {right}
const TARGET_PANE_TOKEN_RIGHT: &str = "{right}";
// {top-left}
const TARGET_PANE_TOKEN_TOP_LEFT: &str = "{top-left}";
// {top-right}
const TARGET_PANE_TOKEN_TOP_RIGHT: &str = "{top-right}";
// {bottom-left}
const TARGET_PANE_TOKEN_BOTTOM_LEFT: &str = "{bottom-left}";
// {bottom-right}
const TARGET_PANE_TOKEN_BOTTOM_RIGHT: &str = "{bottom-right}";
// {up-of}
const TARGET_PANE_TOKEN_UP_OF: &str = "{up-of}";
// {down-of}
const TARGET_PANE_TOKEN_DOWN_OF: &str = "{down-of}";
// {left-of}
const TARGET_PANE_TOKEN_LEFT_OF: &str = "{left-of}";
// {right-of}
const TARGET_PANE_TOKEN_RIGHT_OF: &str = "{right-of}";

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
