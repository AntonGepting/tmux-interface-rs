//! [`GetServerOptionTr`] trait is used for getting tmux server options, using tmux command
//! builder. The returned result is only the option value **not including** the name of the option.
//!
//! Tmux command example:
//! ```text
//! show-options -s -v backspace
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
use crate::{GetOptionTr, ShowOptions, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct GetServerOptionValue;

impl GetOptionTr for GetServerOptionValue {
    fn get<'a, S: Into<Cow<'a, str>>>(name: S) -> TmuxCommand<'a> {
        ShowOptions::new().server().option(name).value().build()
    }
}

impl GetServerOptionTr for GetServerOptionValue {}

impl GetUserOption for GetServerOptionValue {}
