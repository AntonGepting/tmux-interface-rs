pub mod constants;

#[cfg(feature = "tmux_1_0")]
pub mod clock_mode_style;
#[cfg(feature = "tmux_2_3")]
pub mod pane_border_status;
#[cfg(feature = "tmux_2_9")]
pub mod window_size;

pub use constants::*;

#[cfg(feature = "tmux_1_0")]
pub use clock_mode_style::ClockModeStyle;
#[cfg(feature = "tmux_2_3")]
pub use pane_border_status::PaneBorderStatus;
#[cfg(feature = "tmux_2_9")]
pub use window_size::WindowSize;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_0")]
    pub mod clock_mode_style_tests;
    #[cfg(feature = "tmux_2_3")]
    pub mod pane_border_status_tests;
    #[cfg(feature = "tmux_2_9")]
    pub mod window_size_tests;
}
