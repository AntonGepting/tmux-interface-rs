#[cfg(feature = "tmux_3_2")]
pub mod allow_actions;
#[cfg(feature = "tmux_2_9a")]
pub mod client_flags;
pub mod pane_size;
#[cfg(feature = "tmux_3_3")]
pub mod popup_border_lines_type;
#[cfg(feature = "tmux_3_2")]
pub mod position;
#[cfg(feature = "tmux_3_3")]
pub mod prompt_type;
#[cfg(feature = "tmux_3_3")]
pub mod refresh_client_size;
#[cfg(feature = "tmux_3_2")]
pub mod state;
#[cfg(feature = "tmux_3_2")]
pub mod subscribe;

#[cfg(feature = "tmux_3_2")]
pub use self::allow_actions::AllowActions;
#[cfg(feature = "tmux_2_9a")]
pub use self::client_flags::ClientFlags;
pub use self::pane_size::PaneSize;
#[cfg(feature = "tmux_3_2")]
pub use self::pane_size::Size;
#[cfg(feature = "tmux_3_3")]
pub use self::popup_border_lines_type::PopupBorderLinesType;
#[cfg(feature = "tmux_3_2")]
pub use self::position::PositionX;
#[cfg(feature = "tmux_3_2")]
pub use self::position::PositionY;
#[cfg(feature = "tmux_3_3")]
pub use self::prompt_type::PromptType;
#[cfg(feature = "tmux_3_3")]
pub use self::refresh_client_size::RefreshClientSize;
#[cfg(feature = "tmux_3_2")]
pub use self::state::State;
#[cfg(feature = "tmux_3_2")]
pub use self::subscribe::Subscribe;

#[cfg(test)]
#[path = "."]
mod common_tests {
    #[cfg(feature = "tmux_2_9a")]
    pub mod client_flags_tests;
}
