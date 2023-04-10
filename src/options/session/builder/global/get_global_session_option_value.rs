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
use crate::options::{GetOptionTr, GetSessionOptionTr, GetUserOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalSessionOptionValue;

impl GetOptionTr for GetGlobalSessionOptionValue {
    fn get_ext<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        name: S,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().global().option(name).value();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl GetSessionOptionTr for GetGlobalSessionOptionValue {}

impl GetUserOption for GetGlobalSessionOptionValue {}
