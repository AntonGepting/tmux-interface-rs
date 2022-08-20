pub mod action;
pub mod activity;
pub mod detach_on_destroy;
pub mod status;
pub mod status_justify;
pub mod status_position;

pub mod constants;
pub mod get_session_option;
pub mod get_session_options;
pub mod session_options;
pub mod set_session_option;
//pub mod set_session_options;

pub use action::Action;
pub use activity::Activity;
pub use detach_on_destroy::DetachOnDestroy;
pub use status::Status;
pub use status_justify::StatusJustify;
pub use status_position::StatusPosition;

pub use constants::*;
pub use get_session_option::{GetGlobalSessionOption, GetLocalSessionOption, GetSessionOption};
pub use get_session_options::GetSessionOptions;
pub use session_options::*;
pub use set_session_option::{SetGlobalSessionOption, SetLocalSessionOption, SetSessionOption};
//pub use set_session_options::*;

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_1_0")]
    pub mod session_options_tests;

    #[cfg(feature = "tmux_0_8")]
    pub mod action_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod activity_tests;
    #[cfg(feature = "tmux_1_4")]
    pub mod detach_on_destroy_tests;
    #[cfg(feature = "tmux_1_0")]
    pub mod status_justify_tests;
    #[cfg(feature = "tmux_1_7")]
    pub mod status_position_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod status_tests;

    #[cfg(feature = "tmux_0_8")]
    pub mod get_session_options_tests;
}
