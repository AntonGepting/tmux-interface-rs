use std::fmt;

// TODO: tmux ^3.2 display-menu, display-popup

// XXX: names shorter?
/// C        Both    The centre of the terminal
/// R        -x      The right side of the terminal
/// P        Both    The bottom left of the pane
/// M        Both    The mouse position
/// W        Both    The window position on the status line
/// S        -y      The line above or below the status line
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg(feature = "tmux_3_2")]
pub enum PositionX {
    ColumnNumber(usize),
    TerminalCenter,
    TerminalRight,
    PaneBottom,
    MousePosition,
    StatusLineWindowPosition,
    PositionFormat(PositionFormat),
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for PositionX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ColumnNumber(n) => write!(f, "{}", n),
            Self::TerminalCenter => write!(f, "C"),
            Self::TerminalRight => write!(f, "R"),
            Self::PaneBottom => write!(f, "P"),
            Self::MousePosition => write!(f, "M"),
            Self::StatusLineWindowPosition => write!(f, "W"),
            Self::PositionFormat(s) => write!(f, "{}", s),
        }
    }
}

/// C        Both    The centre of the terminal
/// R        -x      The right side of the terminal
/// P        Both    The bottom left of the pane
/// M        Both    The mouse position
/// W        Both    The window position on the status line
/// S        -y      The line above or below the status line
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg(feature = "tmux_3_2")]
pub enum PositionY {
    ColumnNumber(usize),
    TerminalCenter,
    PaneBottom,
    MousePosition,
    StatusLineWindowPosition,
    AboveBelowStatusLine,
    PositionFormat(PositionFormat),
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for PositionY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ColumnNumber(n) => write!(f, "{}", n),
            Self::TerminalCenter => write!(f, "C"),
            Self::PaneBottom => write!(f, "P"),
            Self::MousePosition => write!(f, "M"),
            Self::StatusLineWindowPosition => write!(f, "W"),
            Self::AboveBelowStatusLine => write!(f, "S"),
            Self::PositionFormat(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg(feature = "tmux_3_2")]
pub enum PositionFormat {
    /// `popup_centre_x` - Centered in the client
    PopupCentreX,
    /// `popup_centre_y` - Centered in the client
    PopupCentreY,
    /// `popup_height` - Height of menu or popup
    PopupHeight,
    /// `popup_mouse_bottom` - Bottom of at the mouse
    PopupMouseBottom,
    /// `popup_mouse_centre_x` - Horizontal centre at the mouse
    PopupMouseCentreX,
    /// `popup_mouse_centre_y` - Vertical centre at the mouse
    PopupMouseCentreY,
    /// `popup_mouse_top` - Top at the mouse
    PopupMouseTop,
    /// `popup_mouse_x` - Mouse X position
    PopupMouseX,
    /// `popup_mouse_y` - Mouse Y position
    PopupMouseY,
    /// `popup_pane_bottom` - Bottom of the pane
    PopupPaneBottom,
    /// `popup_pane_left` - Left of the pane
    PopupPaneLeft,
    /// `popup_pane_right` - Right of the pane
    PopupPaneRight,
    /// `popup_pane_top` - Top of the pane
    PopupPaneTop,
    /// `popup_status_line_y` - Above or below the status line
    PopupStatusLineY,
    /// `popup_width` - Width of menu or popup
    PopupWidth,
    /// `popup_window_status_line_x` - At the window position in status line
    PopupWindowStatusLineX,
    /// `popup_window_status_line_y` - At the status line showing the window
    PopupWindowStatusLineY,
}

#[cfg(feature = "tmux_3_2")]
impl fmt::Display for PositionFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::PopupCentreX => "popup_centre_x",
            Self::PopupCentreY => "popup_centre_y",
            Self::PopupHeight => "popup_height",
            Self::PopupMouseBottom => "popup_mouse_bottom",
            Self::PopupMouseCentreX => "popup_mouse_centre_x",
            Self::PopupMouseCentreY => "popup_mouse_centre_y",
            Self::PopupMouseTop => "popup_mouse_top",
            Self::PopupMouseX => "popup_mouse_x",
            Self::PopupMouseY => "popup_mouse_y",
            Self::PopupPaneBottom => "popup_pane_bottom",
            Self::PopupPaneLeft => "popup_pane_left",
            Self::PopupPaneRight => "popup_pane_right",
            Self::PopupPaneTop => "popup_pane_top",
            Self::PopupStatusLineY => "popup_status_line_y",
            Self::PopupWidth => "popup_width",
            Self::PopupWindowStatusLineX => "popup_window_status_line_x",
            Self::PopupWindowStatusLineY => "popup_window_status_line_y",
        };
        write!(f, "#{{{}}}", output)
    }
}
