//! [`GetServerOptionTrait`] trait is used for getting tmux server options, using tmux command
//! builder. The returned result is only the option value **not including** the name of the option.
//!
//! Tmux command example:
//! ```text
//! show-option -s -v backspace
//! # output
//! C-?
//! ```
//!
//! Library equivalent example:
//! ```
//! use tmux_interface::{ShowOptions, Tmux};
//!
//! let option_name = "backspace";
//! let show_option = ShowOptions::new().server().value().option(option_name).build();
//! let output = Tmux::with_command(show_option).output();
//! let value = output.unwrap();
//! ```
//!
use crate::options::*;
use crate::{GetOptionExt, ShowOptions, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct GetServerOptionValue;

impl GetOptionExt for GetServerOptionValue {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .server()
            .value()
            .option(name.into())
            .build()
    }
}

impl GetServerOptionTrait for GetServerOptionValue {}

impl GetUserOption for GetServerOptionValue {}