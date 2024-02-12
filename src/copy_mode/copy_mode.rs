use std::fmt;

// XXX: options for all arguments?
/// Copy mode command with optional parameters, and without repeat count
///
/// for buffer commands:
/// `[<prefix>]` - used to generate the buffer name (default: `buffer`)
///
/// for pipe commands:
/// `[<command>]` - command to which the selected text is piped
///
/// for goto commands:
/// `[<line>]` -
///
/// for jump commands:
/// `[<to>]` -
///
/// for search commands:
/// `[<for>]` -
///
pub enum CopyModeCommand {
    /// `append-selection`
    AppendSelection,
    /// `append-selection-and-cancel`
    AppendSelectionAndCancel,
    /// `back-to-indentation`
    BackToIndentation,
    /// `begin-selection`
    BeginSelection,
    /// `bottom-line`
    BottomLine,
    /// `cancel`
    Cancel,
    /// `clear-selection`
    ClearSelection,
    /// `copy-end-of-line [<prefix>]`
    CopyEndOfLine(Option<String>),
    /// `copy-line [<prefix>]`
    CopyLine(Option<String>),
    /// `copy-pipe [<command>] [<prefix>]`
    CopyPipe(String, Option<String>),
    /// `copy-pipe-no-clear [<command>] [<prefix>]`
    CopyPipeNoClear(String, Option<String>),
    /// `copy-pipe-and-cancel [<command>] [<prefix>]`
    CopyPipeAndCancel(String, Option<String>),
    /// `copy-selection [<prefix>]`
    CopySelection(Option<String>),
    /// `copy-selection-no-clear [<prefix>]`
    CopySelectionNoClear(Option<String>),
    /// `copy-selection-and-cancel [<prefix>]`
    CopySelectionAndCancel(Option<String>),
    /// `cursor-down`
    CursorDown,
    /// `cursor-down-and-cancel`
    CursorDownAndCancel,
    /// `cursor-left`
    CursorLeft,
    /// `cursor-right`
    CursorRight,
    /// `cursor-up`
    CursorUp,
    /// `end-of-line`
    EndOfLine,
    /// `goto-line <line>`
    GotoLine(String),
    /// `halfpage-down`
    HalfpageDown,
    /// `halfpage-down-and-cancel`
    HalfpageDownAndCancel,
    /// `halfpage-up`
    HalfpageUp,
    /// `history-bottom`
    HistoryBottom,
    /// `history-top`
    HistoryTop,
    /// `jump-again`
    JumpAgain,
    /// `jump-backward <to>`
    JumpBackward(String),
    /// `jump-forward <to>`
    JumpForward(String),
    /// `jump-reverse`
    JumpReverse,
    /// `jump-to-backward <to>`
    JumpToBackward(String),
    /// `jump-to-forward <to>`
    JumpToForward(String),
    /// `jump-to-mark`
    JumpToMark,
    /// `middle-line`
    MiddleLine,
    /// `next-matching-bracket`
    NextMatchingBracket,
    /// `next-paragraph`
    NextParagraph,
    /// `next-space`
    NextSpace,
    /// `next-space-end`
    NextSpaceEnd,
    /// `next-word`
    NextWord,
    /// `next-word-end`
    NextWordEnd,
    /// `other-end`
    OtherEnd,
    /// `page-down`
    PageDown,
    /// `page-down-and-cancel`
    PageDownAndCancel,
    /// `page-up`
    PageUp,
    /// `pipe [<command>] [<prefix>]`
    Pipe(String, Option<String>),
    /// `pipe-no-clear [<command>] [<prefix>]`
    PipeNoClear(String, Option<String>),
    /// `pipe-and-cancel [<command>] [<prefix>]`
    PipeAndCancel(String, Option<String>),
    /// `previous-matching-bracket`
    PreviousMatchingBracket,
    /// `previous-paragraph`
    PreviousParagraph,
    /// `previous-space`
    PreviousSpace,
    /// `previous-word`
    PreviousWord,
    /// `rectangle-on`
    RectangleOn,
    /// `rectangle-off`
    RectangleOff,
    /// `rectangle-toggle`
    RectangleToggle,
    /// `refresh-from-pane`
    RefreshFromPane,
    /// `scroll-down`
    ScrollDown,
    /// `scroll-down-and-cancel`
    ScrollDownAndCancel,
    /// `scroll-up`
    ScrollUp,
    /// `search-again`
    SearchAgain,
    /// `search-backward <for>`
    SearchBackward(String),
    /// `search-backward-incremental <for>`
    SearchBackwardIncremental(String),
    /// `search-backward-text <for>`
    SearchBackwardText(String),
    /// `search-forward <for>`
    SearchForward(String),
    /// `search-forward-incremental <for>`
    SearchForwardIncremental(String),
    /// `search-forward-text <for>`
    SearchForwardText(String),
    /// `search-reverse`
    SearchReverse,
    /// `select-line`
    SelectLine,
    /// `select-word`
    SelectWord,
    /// `set-mark`
    SetMark,
    /// `start-of-line`
    StartOfLine,
    /// `stop-selection`
    StopSelection,
    /// `top-line`
    TopLine,
}

// XXX: mb. consts for strings (format!() cant use str as first arg)
impl fmt::Display for CopyModeCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AppendSelection => write!(f, "append-selection"),
            Self::AppendSelectionAndCancel => write!(f, "append-selection-and-cancel"),
            Self::BackToIndentation => write!(f, "back-to-indentation"),
            Self::BeginSelection => write!(f, "begin-selection"),
            Self::BottomLine => write!(f, "bottom-line"),
            Self::Cancel => write!(f, "cancel"),
            Self::ClearSelection => write!(f, "clear-selection"),
            Self::CopyEndOfLine(prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-end-of-line {}", prefix)
                } else {
                    write!(f, "copy-end-of-line")
                }
            }
            Self::CopyLine(prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-line {}", prefix)
                } else {
                    write!(f, "copy-line")
                }
            }
            Self::CopyPipe(command, prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-pipe {} {}", command, prefix)
                } else {
                    write!(f, "copy-pipe {}", command)
                }
            }
            Self::CopyPipeNoClear(command, prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-pipe-no-clear {} {}", command, prefix)
                } else {
                    write!(f, "copy-pipe-no-clear {}", command)
                }
            }
            Self::CopyPipeAndCancel(command, prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-pipe-and-cancel {} {}", command, prefix)
                } else {
                    write!(f, "copy-pipe-and-cancel {}", command)
                }
            }
            Self::CopySelection(prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-selection {}", prefix)
                } else {
                    write!(f, "copy-selection")
                }
            }
            Self::CopySelectionNoClear(prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-selection-no-clear {}", prefix)
                } else {
                    write!(f, "copy-selection-no-clear")
                }
            }
            Self::CopySelectionAndCancel(prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "copy-selection-and-cancel {}", prefix)
                } else {
                    write!(f, "copy-selection-and-cancel")
                }
            }
            Self::CursorDown => write!(f, "cursor-down"),
            Self::CursorDownAndCancel => write!(f, "cursor-down-and-cancel"),
            Self::CursorLeft => write!(f, "cursor-left"),
            Self::CursorRight => write!(f, "cursor-right"),
            Self::CursorUp => write!(f, "cursor-up"),
            Self::EndOfLine => write!(f, "end-of-line"),
            Self::GotoLine(line) => write!(f, "goto-line {}", line),
            Self::HalfpageDown => write!(f, "halfpage-down"),
            Self::HalfpageDownAndCancel => write!(f, "halfpage-down-and-cancel"),
            Self::HalfpageUp => write!(f, "halfpage-up"),
            Self::HistoryBottom => write!(f, "history-bottom"),
            Self::HistoryTop => write!(f, "history-top"),
            Self::JumpAgain => write!(f, "jump-again"),
            Self::JumpBackward(to) => write!(f, "jump-backward {}", to),
            Self::JumpForward(to) => write!(f, "jump-forward {}", to),
            Self::JumpReverse => write!(f, "jump-reverse"),
            Self::JumpToBackward(to) => write!(f, "jump-to-backward {}", to),
            Self::JumpToForward(to) => write!(f, "jump-to-forward {}", to),
            Self::JumpToMark => write!(f, "jump-to-mark"),
            Self::MiddleLine => write!(f, "middle-line"),
            Self::NextMatchingBracket => write!(f, "next-matching-bracket"),
            Self::NextParagraph => write!(f, "next-paragraph"),
            Self::NextSpace => write!(f, "next-space"),
            Self::NextSpaceEnd => write!(f, "next-space-end"),
            Self::NextWord => write!(f, "next-word"),
            Self::NextWordEnd => write!(f, "next-word-end"),
            Self::OtherEnd => write!(f, "other-end"),
            Self::PageDown => write!(f, "page-down"),
            Self::PageDownAndCancel => write!(f, "page-down-and-cancel"),
            Self::PageUp => write!(f, "page-up"),
            Self::Pipe(command, prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "pipe {} {}", command, prefix)
                } else {
                    write!(f, "pipe {}", command)
                }
            }
            Self::PipeNoClear(command, prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "pipe-no-clear {} {}", command, prefix)
                } else {
                    write!(f, "pipe-no-clear {}", command)
                }
            }
            Self::PipeAndCancel(command, prefix) => {
                if let Some(prefix) = prefix {
                    write!(f, "pipe-and-cancel {} {}", command, prefix)
                } else {
                    write!(f, "pipe-and-cancel {}", command)
                }
            }
            Self::PreviousMatchingBracket => write!(f, "previous-matching-bracket"),
            Self::PreviousParagraph => write!(f, "previous-paragraph"),
            Self::PreviousSpace => write!(f, "previous-space"),
            Self::PreviousWord => write!(f, "previous-word"),
            Self::RectangleOn => write!(f, "rectangle-on"),
            Self::RectangleOff => write!(f, "rectangle-off"),
            Self::RectangleToggle => write!(f, "rectangle-toggle"),
            Self::RefreshFromPane => write!(f, "refresh-from-pane"),
            Self::ScrollDown => write!(f, "scroll-down"),
            Self::ScrollDownAndCancel => write!(f, "scroll-down-and-cancel"),
            Self::ScrollUp => write!(f, "scroll-up"),
            Self::SearchAgain => write!(f, "search-again"),
            Self::SearchBackward(for_) => write!(f, "search-backward {}", for_),
            Self::SearchBackwardIncremental(for_) => {
                write!(f, "search-backward-incremental {}", for_)
            }
            Self::SearchBackwardText(for_) => write!(f, "search-backward-text {}", for_),
            Self::SearchForward(for_) => write!(f, "search-forward {}", for_),
            Self::SearchForwardIncremental(for_) => {
                write!(f, "search-forward-incremental {}", for_)
            }
            Self::SearchForwardText(for_) => write!(f, "search-forward-text {}", for_),
            Self::SearchReverse => write!(f, "search-reverse"),
            Self::SelectLine => write!(f, "select-line"),
            Self::SelectWord => write!(f, "select-word"),
            Self::SetMark => write!(f, "set-mark"),
            Self::StartOfLine => write!(f, "start-of-line"),
            Self::StopSelection => write!(f, "stop-selection"),
            Self::TopLine => write!(f, "top-line"),
        }
    }
}
