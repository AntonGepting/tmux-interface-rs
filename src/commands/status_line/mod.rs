#[cfg(feature = "tmux_0_8")]
use crate::CommandPrompt;
#[cfg(feature = "tmux_0_9")]
use crate::ConfirmBefore;
#[cfg(feature = "tmux_3_0")]
use crate::DisplayMenu;
use crate::TmuxCommand;

#[cfg(feature = "tmux_1_0")]
use crate::DisplayMessage;

/// All functions from man tmux "Status line" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE))
#[cfg(feature = "tmux_0_8")]
pub mod command_prompt;
#[cfg(feature = "tmux_0_9")]
pub mod confirm_before;
#[cfg(feature = "tmux_3_0")]
pub mod display_menu;
#[cfg(feature = "tmux_1_0")]
pub mod display_message;
//#[cfg(feature = "tmux_1_0")]
//pub mod select_prompt;

#[cfg(feature = "tmux_0_8")]
pub mod command_prompt_tests;
#[cfg(feature = "tmux_0_9")]
pub mod confirm_before_tests;
#[cfg(feature = "tmux_3_0")]
pub mod display_menu_tests;
#[cfg(feature = "tmux_1_0")]
pub mod display_message_tests;

/// All functions from man tmux "Status line" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn command_prompt(&self) -> CommandPrompt<'a> {
        CommandPrompt::from(self)
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn confirm_before(&self) -> ConfirmBefore<'a> {
        ConfirmBefore::from(self)
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn display_menu(&self) -> DisplayMenu<'a> {
        DisplayMenu::from(self)
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_message(&self) -> DisplayMessage<'a> {
        DisplayMessage::from(self)
    }
}
