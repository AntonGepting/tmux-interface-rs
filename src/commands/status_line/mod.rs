use crate::TmuxCommand;

/// All functions from man tmux "Status line" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE))
#[cfg(feature = "tmux_3_3")]
pub mod clear_prompt_history;
#[cfg(feature = "tmux_3_3")]
pub mod clear_prompt_history_macro;

#[cfg(feature = "tmux_0_8")]
pub mod command_prompt;
#[cfg(feature = "tmux_0_8")]
pub mod command_prompt_macro;

#[cfg(feature = "tmux_0_9")]
pub mod confirm_before;
#[cfg(feature = "tmux_0_9")]
pub mod confirm_before_macro;

#[cfg(feature = "tmux_3_0")]
pub mod display_menu;
#[cfg(feature = "tmux_3_0")]
pub mod display_menu_macro;

#[cfg(feature = "tmux_1_0")]
pub mod display_message;
#[cfg(feature = "tmux_1_0")]
pub mod display_message_macro;

//#[cfg(feature = "tmux_1_0")]
//pub mod select_prompt;
#[cfg(feature = "tmux_3_2")]
pub mod display_popup;
#[cfg(feature = "tmux_3_2")]
pub mod display_popup_macro;

#[cfg(feature = "tmux_3_3")]
pub mod show_prompt_history;
#[cfg(feature = "tmux_3_3")]
pub mod show_prompt_history_macro;

#[cfg(feature = "tmux_3_3")]
pub use clear_prompt_history::{ClearPHist, ClearPromptHistory};
#[cfg(feature = "tmux_0_8")]
pub use command_prompt::CommandPrompt;
#[cfg(feature = "tmux_0_9")]
pub use confirm_before::{Confirm, ConfirmBefore};
#[cfg(feature = "tmux_3_0")]
pub use display_menu::{DisplayMenu, Menu};
#[cfg(feature = "tmux_1_0")]
pub use display_message::{Display, DisplayMessage};
//#[cfg(feature = "tmux_1_0")]
//pub use select_prompt::SelectPrompt;
#[cfg(feature = "tmux_3_2")]
pub use display_popup::{DisplayPopup, Popup};
#[cfg(feature = "tmux_3_3")]
pub use show_prompt_history::{ShowPHist, ShowPromptHistory};

#[cfg(test)]
#[path = "."]
mod status_line_tests {
    #[cfg(feature = "tmux_3_3")]
    pub mod clear_prompt_history_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod command_prompt_tests;
    #[cfg(feature = "tmux_0_9")]
    pub mod confirm_before_tests;
    #[cfg(feature = "tmux_3_0")]
    pub mod display_menu_tests;
    #[cfg(feature = "tmux_1_0")]
    pub mod display_message_tests;
    #[cfg(feature = "tmux_3_2")]
    pub mod display_popup_tests;
    #[cfg(feature = "tmux_3_3")]
    pub mod show_prompt_history_tests;
}

/// All functions from man tmux "Status line" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_3_3")]
    pub fn clear_prompt_history() -> ClearPromptHistory<'a> {
        ClearPromptHistory::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn command_prompt() -> CommandPrompt<'a> {
        CommandPrompt::new()
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn confirm_before() -> ConfirmBefore<'a> {
        ConfirmBefore::new()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn display_menu() -> DisplayMenu<'a> {
        DisplayMenu::new()
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_message() -> DisplayMessage<'a> {
        DisplayMessage::new()
    }

    #[cfg(feature = "tmux_3_2")]
    pub fn display_popup() -> DisplayPopup<'a> {
        DisplayPopup::new()
    }

    #[cfg(feature = "tmux_3_3")]
    pub fn show_prompt_history() -> ShowPromptHistory<'a> {
        ShowPromptHistory::new()
    }
}

#[cfg(feature = "tmux_3_3")]
impl<'a> From<ClearPromptHistory<'a>> for TmuxCommand<'a> {
    fn from(item: ClearPromptHistory<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<CommandPrompt<'a>> for TmuxCommand<'a> {
    fn from(item: CommandPrompt<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_9")]
impl<'a> From<ConfirmBefore<'a>> for TmuxCommand<'a> {
    fn from(item: ConfirmBefore<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_3_0")]
impl<'a> From<DisplayMenu<'a>> for TmuxCommand<'a> {
    fn from(item: DisplayMenu<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_1_0")]
impl<'a> From<DisplayMessage<'a>> for TmuxCommand<'a> {
    fn from(item: DisplayMessage<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_3_2")]
impl<'a> From<DisplayPopup<'a>> for TmuxCommand<'a> {
    fn from(item: DisplayPopup<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_3_3")]
impl<'a> From<ShowPromptHistory<'a>> for TmuxCommand<'a> {
    fn from(item: ShowPromptHistory<'a>) -> Self {
        item.build()
    }
}
