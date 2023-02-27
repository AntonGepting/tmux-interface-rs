//! Module for working with server options
//!
//! # Table of Contents
//!
//! * Command Builder
//!
//!     * Getter Command Builder
//!
//!         * [`GetServerOption`](#get-single-option) - get single option
//!         * [`GetServerOptions`](#get-multiple-options) - get multiple options
//!
//!     * Setter Command Builder
//!
//!         * [`SetServerOption`](crate::SetServerOption) - set single option
//!         * [`SetServerOptions`](crate::SetServerOptions) - set multiple options
//!
//! * Parser
//!    * ServerOptionOutput
//!    * ServerOptionsOutput
//!
//!
//! ## Get Single Option
//!
//! [`GetServerOption`](crate::GetServerOption) Module
//!
//! ### Example
//!
//! ```
//! use crate::{GetServerOption, Tmux};
//!
//! let output = Tmux::with_command(GetServerOption::buffer_limit()).output();
//! ```
//!
//!
//! ## Get Multiple Options
//!
//! [`GetServerOptions`](crate::GetServerOptions) Module
//!
//! ### Example
//!
//! ```
//! use crate::{GetServerOptions, Tmux};
//!
//! let output = Tmux::with_command(
//!     GetServerOptions::new()
//!         .buffer_limit()
//!         .history_file()
//!         .build())
//!     .output();
//! ```
//!
//!
//!

// 1. approach we know what options we are expecting
// 2. approach we don't know what option we are expecting
//
//

pub mod builder;
pub mod common;
pub mod parser;

pub mod server_option;
pub mod server_options;

pub use builder::*;
pub use common::*;
pub use parser::*;

pub use server_option::*;
pub use server_options::*;

#[cfg(test)]
#[path = "."]
mod server_tests {
    //#[cfg(feature = "tmux_1_2")]
    //pub mod server_options_tests;

    #[cfg(feature = "tmux_1_2")]
    pub mod server_option_tests;
}
