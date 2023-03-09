use crate::options::{GetOptionExt, GetSessionOption};
use crate::{GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;

// NOTE: ADR: compile time or run time parametrisation for global local option set/get
// * compile time: trais
// * runtime: struct field with user given setter/getter

// TODO: all options exist in get/set?

pub struct GetGlobalSessionOption;

impl GetOptionExt for GetGlobalSessionOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().option(name).global().build()
    }
}

impl GetUserOption for GetGlobalSessionOption {}

impl GetSessionOption for GetGlobalSessionOption {}
