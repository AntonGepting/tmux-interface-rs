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

pub const APPEND_SELECTION: &str = "append-selection";
pub const APPEND_SELECTION_AND_CANCEL: &str = "append-selection-and-cancel";
pub const BACK_TO_INDENTATION: &str = "back-to-indentation";
pub const BEGIN_SELECTION: &str = "begin-selection";
pub const BOTTOM_LINE: &str = "bottom-line";
pub const CANCEL: &str = "cancel";
pub const CLEAR_SELECTION: &str = "clear-selection";
pub const COPY_END_OF_LINE: &str = "copy-end-of-line";
pub const COPY_LINE: &str = "copy-line";
pub const COPY_PIPE: &str = "copy-pipe";
pub const COPY_PIPE_NO_CLEAR: &str = "copy-pipe-no-clear";
pub const COPY_PIPE_AND_CANCEL: &str = "copy-pipe-and-cancel";
pub const COPY_SELECTION: &str = "copy-selection";
pub const COPY_SELECTION_NO_CLEAR: &str = "copy-selection-no-clear";
pub const COPY_SELECTION_AND_CANCEL: &str = "copy-selection-and-cancel";
pub const CURSOR_DOWN: &str = "cursor-down";
pub const CURSOR_DOWN_AND_CANCEL: &str = "cursor-down-and-cancel";
pub const CURSOR_LEFT: &str = "cursor-left";
pub const CURSOR_RIGHT: &str = "cursor-right";
pub const CURSOR_UP: &str = "cursor-up";
pub const END_OF_LINE: &str = "end-of-line";
pub const GOTO_LINE: &str = "goto-line";
pub const HALFPAGE_DOWN: &str = "halfpage-down";
pub const HALFPAGE_DOWN_AND_CANCEL: &str = "halfpage-down-and-cancel";
pub const HALFPAGE_UP: &str = "halfpage-up";
pub const HISTORY_BOTTOM: &str = "history-bottom";
pub const HISTORY_TOP: &str = "history-top";
pub const JUMP_AGAIN: &str = "jump-again";
pub const JUMP_BACKWARD: &str = "jump-backward";
pub const JUMP_FORWARD: &str = "jump-forward";
pub const JUMP_REVERSE: &str = "jump-reverse";
pub const JUMP_TO_BACKWARD: &str = "jump-to-backward";
pub const JUMP_TO_FORWARD: &str = "jump-to-forward";
pub const JUMP_TO_MARK: &str = "jump-to-mark";
pub const MIDDLE_LINE: &str = "middle-line";
pub const NEXT_MATCHING_BRACKET: &str = "next-matching-bracket";
pub const NEXT_PARAGRAPH: &str = "next-paragraph";
pub const NEXT_SPACE: &str = "next-space";
pub const NEXT_SPACE_END: &str = "next-space-end";
pub const NEXT_WORD: &str = "next-word";
pub const NEXT_WORD_END: &str = "next-word-end";
pub const OTHER_END: &str = "other-end";
pub const PAGE_DOWN: &str = "page-down";
pub const PAGE_DOWN_AND_CANCEL: &str = "page-down-and-cancel";
pub const PAGE_UP: &str = "page-up";
pub const PIPE: &str = "pipe";
pub const PIPE_NO_CLEAR: &str = "pipe-no-clear";
pub const PIPE_AND_CANCEL: &str = "pipe-and-cancel";
pub const PREVIOUS_MATCHING_BRACKET: &str = "previous-matching-bracket";
pub const PREVIOUS_PARAGRAPH: &str = "previous-paragraph";
pub const PREVIOUS_SPACE: &str = "previous-space";
pub const PREVIOUS_WORD: &str = "previous-word";
pub const RECTANGLE_ON: &str = "rectangle-on";
pub const RECTANGLE_OFF: &str = "rectangle-off";
pub const RECTANGLE_TOGGLE: &str = "rectangle-toggle";
pub const REFRESH_FROM_PANE: &str = "refresh-from-pane";
pub const SCROLL_DOWN: &str = "scroll-down";
pub const SCROLL_DOWN_AND_CANCEL: &str = "scroll-down-and-cancel";
pub const SCROLL_UP: &str = "scroll-up";
pub const SEARCH_AGAIN: &str = "search-again";
pub const SEARCH_BACKWARD: &str = "search-backward";
pub const SEARCH_BACKWARD_INCREMENTAL: &str = "search-backward-incremental";
pub const SEARCH_BACKWARD_TEXT: &str = "search-backward-text";
pub const SEARCH_FORWARD: &str = "search-forward";
pub const SEARCH_FORWARD_INCREMENTAL: &str = "search-forward-incremental";
pub const SEARCH_FORWARD_TEXT: &str = "search-forward-text";
pub const SEARCH_REVERSE: &str = "search-reverse";
pub const SELECT_LINE: &str = "select-line";
pub const SELECT_WORD: &str = "select-word";
pub const SET_MARK: &str = "set-mark";
pub const START_OF_LINE: &str = "start-of-line";
pub const STOP_SELECTION: &str = "stop-selection";
pub const TOP_LINE: &str = "top-line";

// XXX: mb. consts for strings (format!() cant use str as first arg)
impl fmt::Display for CopyModeCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::AppendSelection => APPEND_SELECTION.to_string(),
            Self::AppendSelectionAndCancel => APPEND_SELECTION_AND_CANCEL.to_string(),
            Self::BackToIndentation => BACK_TO_INDENTATION.to_string(),
            Self::BeginSelection => BEGIN_SELECTION.to_string(),
            Self::BottomLine => BOTTOM_LINE.to_string(),
            Self::Cancel => CANCEL.to_string(),
            Self::ClearSelection => CLEAR_SELECTION.to_string(),
            Self::CopyEndOfLine(prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {}", COPY_END_OF_LINE, prefix)
                } else {
                    COPY_END_OF_LINE.to_string()
                }
            }
            Self::CopyLine(prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {}", COPY_LINE, prefix)
                } else {
                    COPY_LINE.to_string()
                }
            }
            Self::CopyPipe(command, prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {} {}", COPY_PIPE, command, prefix)
                } else {
                    format!("{} {}", COPY_PIPE, command)
                }
            }
            Self::CopyPipeNoClear(command, prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {} {}", COPY_PIPE_NO_CLEAR, command, prefix)
                } else {
                    format!("{} {}", COPY_PIPE_NO_CLEAR, command)
                }
            }
            Self::CopyPipeAndCancel(command, prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {} {}", COPY_PIPE_AND_CANCEL, command, prefix)
                } else {
                    format!("{} {}", COPY_PIPE_AND_CANCEL, command)
                }
            }
            Self::CopySelection(prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {}", COPY_SELECTION, prefix)
                } else {
                    COPY_SELECTION.to_string()
                }
            }
            Self::CopySelectionNoClear(prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {}", COPY_SELECTION_NO_CLEAR, prefix)
                } else {
                    COPY_SELECTION_NO_CLEAR.to_string()
                }
            }
            Self::CopySelectionAndCancel(prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {}", COPY_SELECTION_AND_CANCEL, prefix)
                } else {
                    COPY_SELECTION_AND_CANCEL.to_string()
                }
            }
            Self::CursorDown => CURSOR_DOWN.to_string(),
            Self::CursorDownAndCancel => CURSOR_DOWN_AND_CANCEL.to_string(),
            Self::CursorLeft => CURSOR_LEFT.to_string(),
            Self::CursorRight => CURSOR_RIGHT.to_string(),
            Self::CursorUp => CURSOR_UP.to_string(),
            Self::EndOfLine => END_OF_LINE.to_string(),
            Self::GotoLine(line) => format!("{} {}", GOTO_LINE, line),
            Self::HalfpageDown => HALFPAGE_DOWN.to_string(),
            Self::HalfpageDownAndCancel => HALFPAGE_DOWN_AND_CANCEL.to_string(),
            Self::HalfpageUp => HALFPAGE_UP.to_string(),
            Self::HistoryBottom => HISTORY_BOTTOM.to_string(),
            Self::HistoryTop => HISTORY_TOP.to_string(),
            Self::JumpAgain => JUMP_AGAIN.to_string(),
            Self::JumpBackward(to) => format!("{} {}", JUMP_BACKWARD, to),
            Self::JumpForward(to) => format!("{} {}", JUMP_FORWARD, to),
            Self::JumpReverse => JUMP_REVERSE.to_string(),
            Self::JumpToBackward(to) => format!("{} {}", JUMP_TO_BACKWARD, to),
            Self::JumpToForward(to) => format!("{} {}", JUMP_TO_FORWARD, to),
            Self::JumpToMark => JUMP_TO_MARK.to_string(),
            Self::MiddleLine => MIDDLE_LINE.to_string(),
            Self::NextMatchingBracket => NEXT_MATCHING_BRACKET.to_string(),
            Self::NextParagraph => NEXT_PARAGRAPH.to_string(),
            Self::NextSpace => NEXT_SPACE.to_string(),
            Self::NextSpaceEnd => NEXT_SPACE_END.to_string(),
            Self::NextWord => NEXT_WORD.to_string(),
            Self::NextWordEnd => NEXT_WORD_END.to_string(),
            Self::OtherEnd => OTHER_END.to_string(),
            Self::PageDown => PAGE_DOWN.to_string(),
            Self::PageDownAndCancel => PAGE_DOWN_AND_CANCEL.to_string(),
            Self::PageUp => PAGE_UP.to_string(),
            Self::Pipe(command, prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {} {}", PIPE, command, prefix)
                } else {
                    format!("{} {}", PIPE, command)
                }
            }
            Self::PipeNoClear(command, prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {} {}", PIPE_NO_CLEAR, command, prefix)
                } else {
                    format!("{} {}", PIPE_NO_CLEAR, command)
                }
            }
            Self::PipeAndCancel(command, prefix) => {
                if let Some(prefix) = prefix {
                    format!("{} {} {}", PIPE_AND_CANCEL, command, prefix)
                } else {
                    format!("{} {}", PIPE_AND_CANCEL, command)
                }
            }
            Self::PreviousMatchingBracket => PREVIOUS_MATCHING_BRACKET.to_string(),
            Self::PreviousParagraph => PREVIOUS_PARAGRAPH.to_string(),
            Self::PreviousSpace => PREVIOUS_SPACE.to_string(),
            Self::PreviousWord => PREVIOUS_WORD.to_string(),
            Self::RectangleOn => RECTANGLE_ON.to_string(),
            Self::RectangleOff => RECTANGLE_OFF.to_string(),
            Self::RectangleToggle => RECTANGLE_TOGGLE.to_string(),
            Self::RefreshFromPane => REFRESH_FROM_PANE.to_string(),
            Self::ScrollDown => SCROLL_DOWN.to_string(),
            Self::ScrollDownAndCancel => SCROLL_DOWN_AND_CANCEL.to_string(),
            Self::ScrollUp => SCROLL_UP.to_string(),
            Self::SearchAgain => SEARCH_AGAIN.to_string(),
            Self::SearchBackward(for_) => format!("{} {}", SEARCH_BACKWARD, for_),
            Self::SearchBackwardIncremental(for_) => {
                format!("{} {}", SEARCH_BACKWARD_INCREMENTAL, for_)
            }
            Self::SearchBackwardText(for_) => format!("{} {}", SEARCH_BACKWARD_TEXT, for_),
            Self::SearchForward(for_) => format!("{} {}", SEARCH_FORWARD, for_),
            Self::SearchForwardIncremental(for_) => {
                format!("{} {}", SEARCH_FORWARD_INCREMENTAL, for_)
            }
            Self::SearchForwardText(for_) => format!("{} {}", SEARCH_FORWARD_TEXT, for_),
            Self::SearchReverse => SEARCH_REVERSE.to_string(),
            Self::SelectLine => SELECT_LINE.to_string(),
            Self::SelectWord => SELECT_WORD.to_string(),
            Self::SetMark => SET_MARK.to_string(),
            Self::StartOfLine => START_OF_LINE.to_string(),
            Self::StopSelection => STOP_SELECTION.to_string(),
            Self::TopLine => TOP_LINE.to_string(),
        };

        write!(f, "{}", s)
    }
}
