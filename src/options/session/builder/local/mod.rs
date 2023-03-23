pub mod get_local_session_option;
pub mod get_local_session_option_value;
pub mod set_local_session_option;

pub mod get_local_session_options;
pub mod set_local_session_options;

pub use get_local_session_option::GetLocalSessionOption;
pub use get_local_session_option_value::GetLocalSessionOptionValue;
pub use set_local_session_option::SetLocalSessionOption;

pub use get_local_session_options::GetLocalSessionOptions;
pub use set_local_session_options::SetLocalSessionOptions;

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod get_local_session_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod get_local_session_options_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_local_session_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_local_session_options_tests;
}
