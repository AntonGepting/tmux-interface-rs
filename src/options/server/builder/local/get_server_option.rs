//! [`GetServerOption`] trait is used for getting tmux server options, using
//! tmux command builder. The returned result is the option name **with** the option value.
//!
//! Tmux command example:
//! ```text
//! show-option -s backspace
//! # output
//! backspace C-?
//! ```
//!
//! Library equivalent example:
//! ```
//! use tmux_interface::{ShowOptions, Tmux};
//!
//! let option_name = "backspace";
//! let show_option = ShowOptions::new().server().option(option_name).build();
//! let output = Tmux::with_command(show_option).output();
//! let value = output.unwrap();
//! ```
//!
use crate::options::*;
use crate::{GetOptionTr, ShowOptions, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct GetServerOption;

impl GetOptionTr for GetServerOption {
    fn get<'a, S: Into<Cow<'a, str>>>(name: S) -> TmuxCommand<'a> {
        ShowOptions::new().server().option(name).build()
    }
}

impl GetServerOptionTr for GetServerOption {}

impl GetUserOption for GetServerOption {}
