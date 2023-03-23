pub mod get_global_session_option;
pub mod get_global_session_option_value;
pub mod set_global_session_option;

pub mod get_global_session_options;
pub mod set_global_session_options;

pub use get_global_session_option::GetGlobalSessionOption;
pub use get_global_session_option_value::GetGlobalSessionOptionValue;
pub use set_global_session_option::SetGlobalSessionOption;

pub use get_global_session_options::GetGlobalSessionOptions;
pub use set_global_session_options::SetGlobalSessionOptions;

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod get_global_session_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod get_global_session_options_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_global_session_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_global_session_options_tests;
}
