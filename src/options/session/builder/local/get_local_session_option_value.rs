//! [`GetLocalSessionOptionValue`] trait is used for getting tmux session options, using tmux command
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
//! let show_option = ShowOptions::new().value().option(option_name).build();
//! let output = Tmux::with_command(show_option).output();
//! let value = output.unwrap();
//! ```
//!
use crate::options::{GetOptionTr, GetSessionOptionTr, GetUserOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct GetLocalSessionOptionValue;

impl GetOptionTr for GetLocalSessionOptionValue {
    fn get_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().value().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl GetSessionOptionTr for GetLocalSessionOptionValue {}

impl GetUserOption for GetLocalSessionOptionValue {}
