pub mod get_global_window_option;
pub mod get_global_window_option_value;
pub mod get_global_window_options;
pub mod set_global_window_option;
pub mod set_global_window_options;

pub use get_global_window_option::GetGlobalWindowOption;
pub use get_global_window_option_value::GetGlobalWindowOptionValue;
pub use get_global_window_options::GetGlobalWindowOptions;
pub use set_global_window_option::SetGlobalWindowOption;
pub use set_global_window_options::SetGlobalWindowOptions;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod get_global_window_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod get_global_window_option_value_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod get_global_window_options_tests;

    #[cfg(feature = "tmux_0_8")]
    pub mod set_global_window_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_global_window_options_tests;
}
