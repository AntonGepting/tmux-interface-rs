pub mod global;
pub mod local;

pub mod get_session_option;
pub mod set_session_option;

pub mod get_session_options;
pub mod set_session_options;

pub use global::*;
pub use local::*;

pub use get_session_option::GetSessionOption;
pub use set_session_option::SetSessionOption;

pub use get_session_options::GetSessionOptions;
pub use set_session_options::SetSessionOptions;

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
