pub mod get_session_option;
pub mod get_session_options;
pub mod set_session_option;
pub mod set_session_options;

pub use get_session_option::{GetGlobalSessionOption, GetLocalSessionOption, GetSessionOption};
pub use get_session_options::{GetGlobalSessionOptions, GetLocalSessionOptions, GetSessionOptions};
pub use set_session_option::{SetGlobalSessionOption, SetLocalSessionOption, SetSessionOption};
pub use set_session_options::{SetGlobalSessionOptions, SetLocalSessionOptions, SetSessionOptions};

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod get_session_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod get_session_options_tests;

    #[cfg(feature = "tmux_0_8")]
    pub mod set_session_option_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod set_session_options_tests;
}