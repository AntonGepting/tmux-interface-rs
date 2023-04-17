//! Module for working with server options
//!
//! # Table of Contents
//!
//! * Command Builder
//!
//!     * 1. Getter Command Builder
//!         * 1.1. [get single option (returned option name value pair)](#11-get-single-option) [`GetServerOption`]
//!         * 1.2. [get single option (returned option value only)](#12-get-single-option-value) [`GetServerOptionValue`]
//!         * 1.3. [get multiple options](#13-get-multiple-options) [`GetServerOptions`]
//!
//!     * 2. Setter Command Builder
//!         * 2.1. [set single option](#21-set-single-option) [`SetServerOption`]
//!         * 2.2. [set multiple options](#22-set-server-options) [`SetServerOptions`]
//!
//!
//! ## 1.1. Get Single Option
//!
//! [`GetServerOption`] Module
//!
//! ### Example
//!
//! ```
//! use tmux_interface::{GetServerOption, GetServerOptionTr, Tmux};
//!
//! let output = Tmux::with_command(GetServerOption::buffer_limit()).output();
//! ```
//!
//! ## 1.2. Get Single Option Value
//!
//! [`GetServerOptionValue`] Module
//!
//! ### Example
//!
//! ```
//! use tmux_interface::{GetServerOptionValue, GetServerOptionTr, Tmux};
//!
//! let output = Tmux::with_command(GetServerOptionValue::buffer_limit()).output();
//! ```
//!
//! ## 1.3. Get Multiple Options
//!
//! [`GetServerOptions`] Module
//!
//! ### Example
//!
//! ```
//! use tmux_interface::{GetServerOptions, GetServerOptionsTr, Tmux};
//!
//! let output = Tmux::with_commands(
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
pub mod ctl;

pub mod server_options;

pub use builder::*;
pub use common::*;
pub use ctl::*;

pub use server_options::*;

#[cfg(test)]
#[path = "."]
mod server_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod server_options_tests;
}
