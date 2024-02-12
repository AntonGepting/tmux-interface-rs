#[test]
fn to_string() {
    use crate::copy_mode::copy_mode::CopyModeCommand;

    assert_eq!(
        CopyModeCommand::AppendSelection.to_string(),
        "append-selection"
    );

    assert_eq!(
        CopyModeCommand::AppendSelectionAndCancel.to_string(),
        "append-selection-and-cancel"
    );
    assert_eq!(
        CopyModeCommand::BackToIndentation.to_string(),
        "back-to-indentation"
    );
    assert_eq!(
        CopyModeCommand::BeginSelection.to_string(),
        "begin-selection"
    );
    assert_eq!(CopyModeCommand::BottomLine.to_string(), "bottom-line");
    assert_eq!(CopyModeCommand::Cancel.to_string(), "cancel");
    assert_eq!(
        CopyModeCommand::ClearSelection.to_string(),
        "clear-selection"
    );
    assert_eq!(
        CopyModeCommand::CopyEndOfLine("[<prefix>]".to_string()).to_string(),
        "copy-end-of-line [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopyLine("[<prefix>]".to_string()).to_string(),
        "copy-line [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopyPipe("[<command>]".to_string(), "[<prefix>]".to_string()).to_string(),
        "copy-pipe [<command>] [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopyPipeNoClear("[<command>]".to_string(), "[<prefix>]".to_string())
            .to_string(),
        "copy-pipe-no-clear [<command>] [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopyPipeAndCancel("[<command>]".to_string(), "[<prefix>]".to_string())
            .to_string(),
        "copy-pipe-and-cancel [<command>] [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopySelection("[<prefix>]".to_string()).to_string(),
        "copy-selection [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopySelectionNoClear("[<prefix>]".to_string()).to_string(),
        "copy-selection-no-clear [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::CopySelectionAndCancel("[<prefix>]".to_string()).to_string(),
        "copy-selection-and-cancel [<prefix>]"
    );
    assert_eq!(CopyModeCommand::CursorDown.to_string(), "cursor-down");
    assert_eq!(
        CopyModeCommand::CursorDownAndCancel.to_string(),
        "cursor-down-and-cancel"
    );
    assert_eq!(CopyModeCommand::CursorLeft.to_string(), "cursor-left");
    assert_eq!(CopyModeCommand::CursorRight.to_string(), "cursor-right");
    assert_eq!(CopyModeCommand::CursorUp.to_string(), "cursor-up");
    assert_eq!(CopyModeCommand::EndOfLine.to_string(), "end-of-line");
    assert_eq!(
        CopyModeCommand::GotoLine("<line>".to_string()).to_string(),
        "goto-line <line>"
    );
    assert_eq!(CopyModeCommand::HalfpageDown.to_string(), "halfpage-down");
    assert_eq!(
        CopyModeCommand::HalfpageDownAndCancel.to_string(),
        "halfpage-down-and-cancel"
    );
    assert_eq!(CopyModeCommand::HalfpageUp.to_string(), "halfpage-up");
    assert_eq!(CopyModeCommand::HistoryBottom.to_string(), "history-bottom");
    assert_eq!(CopyModeCommand::HistoryTop.to_string(), "history-top");
    assert_eq!(CopyModeCommand::JumpAgain.to_string(), "jump-again");
    assert_eq!(
        CopyModeCommand::JumpBackward("<to>".to_string()).to_string(),
        "jump-backward <to>"
    );
    assert_eq!(
        CopyModeCommand::JumpForward("<to>".to_string()).to_string(),
        "jump-forward <to>"
    );
    assert_eq!(CopyModeCommand::JumpReverse.to_string(), "jump-reverse");
    assert_eq!(
        CopyModeCommand::JumpToBackward("<to>".to_string()).to_string(),
        "jump-to-backward <to>"
    );
    assert_eq!(
        CopyModeCommand::JumpToForward("<to>".to_string()).to_string(),
        "jump-to-forward <to>"
    );
    assert_eq!(CopyModeCommand::JumpToMark.to_string(), "jump-to-mark");
    assert_eq!(CopyModeCommand::MiddleLine.to_string(), "middle-line");
    assert_eq!(
        CopyModeCommand::NextMatchingBracket.to_string(),
        "next-matching-bracket"
    );
    assert_eq!(CopyModeCommand::NextParagraph.to_string(), "next-paragraph");
    assert_eq!(CopyModeCommand::NextSpace.to_string(), "next-space");
    assert_eq!(CopyModeCommand::NextSpaceEnd.to_string(), "next-space-end");
    assert_eq!(CopyModeCommand::NextWord.to_string(), "next-word");
    assert_eq!(CopyModeCommand::NextWordEnd.to_string(), "next-word-end");
    assert_eq!(CopyModeCommand::OtherEnd.to_string(), "other-end");
    assert_eq!(CopyModeCommand::PageDown.to_string(), "page-down");
    assert_eq!(
        CopyModeCommand::PageDownAndCancel.to_string(),
        "page-down-and-cancel"
    );
    assert_eq!(CopyModeCommand::PageUp.to_string(), "page-up");
    assert_eq!(
        CopyModeCommand::Pipe("[<command>]".to_string(), "[<prefix>]".to_string()).to_string(),
        "pipe [<command>] [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::PipeNoClear("[<command>]".to_string(), "[<prefix>]".to_string())
            .to_string(),
        "pipe-no-clear [<command>] [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::PipeAndCancel("[<command>]".to_string(), "[<prefix>]".to_string())
            .to_string(),
        "pipe-and-cancel [<command>] [<prefix>]"
    );
    assert_eq!(
        CopyModeCommand::PreviousMatchingBracket.to_string(),
        "previous-matching-bracket"
    );
    assert_eq!(
        CopyModeCommand::PreviousParagraph.to_string(),
        "previous-paragraph"
    );
    assert_eq!(CopyModeCommand::PreviousSpace.to_string(), "previous-space");
    assert_eq!(CopyModeCommand::PreviousWord.to_string(), "previous-word");
    assert_eq!(CopyModeCommand::RectangleOn.to_string(), "rectangle-on");
    assert_eq!(CopyModeCommand::RectangleOff.to_string(), "rectangle-off");
    assert_eq!(
        CopyModeCommand::RectangleToggle.to_string(),
        "rectangle-toggle"
    );
    assert_eq!(
        CopyModeCommand::RefreshFromPane.to_string(),
        "refresh-from-pane"
    );
    assert_eq!(CopyModeCommand::ScrollDown.to_string(), "scroll-down");
    assert_eq!(
        CopyModeCommand::ScrollDownAndCancel.to_string(),
        "scroll-down-and-cancel"
    );
    assert_eq!(CopyModeCommand::ScrollUp.to_string(), "scroll-up");
    assert_eq!(CopyModeCommand::SearchAgain.to_string(), "search-again");
    assert_eq!(
        CopyModeCommand::SearchBackward("<for>".to_string()).to_string(),
        "search-backward <for>"
    );
    assert_eq!(
        CopyModeCommand::SearchBackwardIncremental("<for>".to_string()).to_string(),
        "search-backward-incremental <for>"
    );
    assert_eq!(
        CopyModeCommand::SearchBackwardText("<for>".to_string()).to_string(),
        "search-backward-text <for>"
    );
    assert_eq!(
        CopyModeCommand::SearchForward("<for>".to_string()).to_string(),
        "search-forward <for>"
    );
    assert_eq!(
        CopyModeCommand::SearchForwardIncremental("<for>".to_string()).to_string(),
        "search-forward-incremental <for>"
    );
    assert_eq!(
        CopyModeCommand::SearchForwardText("<for>".to_string()).to_string(),
        "search-forward-text <for>"
    );
    assert_eq!(CopyModeCommand::SearchReverse.to_string(), "search-reverse");
    assert_eq!(CopyModeCommand::SelectLine.to_string(), "select-line");
    assert_eq!(CopyModeCommand::SelectWord.to_string(), "select-word");
    assert_eq!(CopyModeCommand::SetMark.to_string(), "set-mark");
    assert_eq!(CopyModeCommand::StartOfLine.to_string(), "start-of-line");
    assert_eq!(CopyModeCommand::StopSelection.to_string(), "stop-selection");
    assert_eq!(CopyModeCommand::TopLine.to_string(), "top-line");
}
