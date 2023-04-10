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

pub mod local;

pub mod get_server_option_tr;
pub mod get_server_options_tr;
pub mod set_server_option_tr;
pub mod set_server_options_tr;

pub use local::*;

pub use get_server_option_tr::GetServerOptionTr;
pub use get_server_options_tr::GetServerOptionsTr;
pub use set_server_option_tr::SetServerOptionTr;
pub use set_server_options_tr::SetServerOptionsTr;
