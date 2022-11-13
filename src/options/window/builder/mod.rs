pub mod get_window_option;
pub mod get_window_options;
pub mod set_window_option;
pub mod set_window_options;

pub use get_window_option::*;
pub use get_window_options::*;
pub use set_window_option::*;
pub use set_window_options::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    //#[cfg(feature = "tmux_0_8")]
    pub mod get_window_option_tests;
    //#[cfg(feature = "tmux_0_8")]
    pub mod get_window_options_tests;

    //#[cfg(feature = "tmux_0_8")]
    pub mod set_window_option_tests;
    //#[cfg(feature = "tmux_0_8")]
    pub mod set_window_options_tests;
}