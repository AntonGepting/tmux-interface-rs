pub mod common;

pub mod get_session_option;
pub mod get_session_options;
pub mod session_options;
pub mod set_session_option;
//pub mod set_session_options;

pub use common::*;

pub use get_session_option::{GetGlobalSessionOption, GetLocalSessionOption, GetSessionOption};
pub use get_session_options::GetSessionOptions;
pub use session_options::*;
pub use set_session_option::{SetGlobalSessionOption, SetLocalSessionOption, SetSessionOption};
//pub use set_session_options::*;

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_1_0")]
    pub mod session_options_tests;

    #[cfg(feature = "tmux_0_8")]
    pub mod get_session_options_tests;
}
