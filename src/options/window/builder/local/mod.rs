pub mod get_local_window_option;
#[cfg(feature = "tmux_1_8")]
pub mod get_local_window_option_value;
pub mod get_local_window_options;
pub mod set_local_window_option;
pub mod set_local_window_options;

pub use get_local_window_option::GetLocalWindowOption;
#[cfg(feature = "tmux_1_8")]
pub use get_local_window_option_value::GetLocalWindowOptionValue;
pub use get_local_window_options::GetLocalWindowOptions;
pub use set_local_window_option::SetLocalWindowOption;
pub use set_local_window_options::SetLocalWindowOptions;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod get_local_window_option_tests;
    #[cfg(feature = "tmux_1_8")]
    pub mod get_local_window_option_value_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod get_local_window_options_tests;

    #[cfg(feature = "tmux_0_8")]
    pub mod set_local_window_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_local_window_options_tests;
}
