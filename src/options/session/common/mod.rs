pub mod constants;

pub mod action;
pub mod activity;
pub mod destroy_unattached;
pub mod detach_on_destroy;
#[cfg(feature = "tmux_3_4")]
pub mod message_line;
pub mod status;
pub mod status_justify;
pub mod status_position;

pub use constants::*;

pub use action::Action;
pub use activity::Activity;
pub use destroy_unattached::DestroyUnattached;
pub use detach_on_destroy::DetachOnDestroy;
#[cfg(feature = "tmux_3_4")]
pub use message_line::MessageLine;
pub use status::Status;
pub use status_justify::StatusJustify;
pub use status_position::StatusPosition;

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod action_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod activity_tests;
    #[cfg(feature = "tmux_1_5")]
    pub mod destroy_unattached_tests;
    #[cfg(feature = "tmux_1_4")]
    pub mod detach_on_destroy_tests;
    #[cfg(feature = "tmux_3_4")]
    pub mod message_line;
    #[cfg(feature = "tmux_1_0")]
    pub mod status_justify_tests;
    #[cfg(feature = "tmux_1_7")]
    pub mod status_position_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod status_tests;
}
