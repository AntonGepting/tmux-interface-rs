#[cfg(feature = "tmux_1_0")]
pub mod clock_mode_style;
#[cfg(feature = "tmux_2_3")]
pub mod pane_border_status;
#[cfg(feature = "tmux_2_9")]
pub mod window_size;

pub mod constants;
pub mod get_window_option;
pub mod set_window_option_ext;
pub mod window_options;

#[cfg(feature = "tmux_1_0")]
pub use clock_mode_style::ClockModeStyle;
#[cfg(feature = "tmux_2_3")]
pub use pane_border_status::PaneBorderStatus;
#[cfg(feature = "tmux_2_9")]
pub use window_size::WindowSize;

pub use window_options::*;

pub use constants::*;
pub use get_window_option::*;
pub use set_window_option_ext::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod window_options_tests;

    #[cfg(feature = "tmux_1_0")]
    pub mod clock_mode_style_tests;
    #[cfg(feature = "tmux_2_3")]
    pub mod pane_border_status_tests;
    #[cfg(feature = "tmux_2_9")]
    pub mod window_size_tests;
}
