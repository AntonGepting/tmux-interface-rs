//! [`GetGlobalSessionOptionValue`] trait is used for getting tmux session options, using tmux command
//! builder. The returned result is only the option value **not including** the name of the option.
//!
//! Tmux command example:
//! ```text
//! show-option -s -g -v backspace
//! # output
//! C-?
//! ```
//!
//! Library equivalent example:
//! ```
//! use tmux_interface::{ShowOptions, Tmux};
//!
//! let option_name = "backspace";
//! let show_option = ShowOptions::new().value().option(option_name).build();
//! let output = Tmux::with_command(show_option).output();
//! let value = output.unwrap();
//! ```
//!
use crate::options::{GetOptionExt, GetSessionOption, GetUserOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalSessionOptionValue;

impl GetOptionExt for GetGlobalSessionOptionValue {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .global()
            .value()
            .option(name.into())
            .build()
    }
}

impl GetSessionOption for GetGlobalSessionOptionValue {}

impl GetUserOption for GetGlobalSessionOptionValue {}
