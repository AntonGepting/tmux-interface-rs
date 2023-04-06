//! Traits for building commands for recieving tmux options
//!
//! Server Options
//! * Get
//!     * Single
//!         * [`GetServerOption`] single option with option name
//!         * [`GetServerOptionValue`] single option without option name (value only)
//!     * Multiple
//!         * [`GetServerOptions`] multiple options
//! * Set
//!     * Single
//!         * [`SetServerOption`] single option
//!     * Multiple
//!         * [`SetServerOptions`] multiple options
//!
pub mod get_server_option;
pub mod get_server_option_tr;
pub mod get_server_option_value;
pub mod get_server_options;
pub mod set_server_option;
pub mod set_server_option_tr;
pub mod set_server_options;

pub use get_server_option::*;
pub use get_server_option_tr::GetServerOptionTr;
pub use get_server_option_value::*;
pub use get_server_options::*;
pub use set_server_option::*;
pub use set_server_option_tr::SetServerOptionTr;
pub use set_server_options::*;

#[cfg(test)]
#[path = "."]
mod server_tests {
    pub mod get_server_option_tests;
    pub mod get_server_options_tests;

    pub mod set_server_option_tests;
    pub mod set_server_options_tests;
}
