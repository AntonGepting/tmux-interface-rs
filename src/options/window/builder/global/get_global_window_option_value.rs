use crate::options::{GetOptionExt, GetUserOption, GetWindowOptionTr};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

// NOTE: ADR: compile time or run time parametrisation for global local option set/get
// * compile time: trais
// * runtime: struct field with user given setter/getter

// TODO: all options exist in get/set?
pub struct GetGlobalWindowOptionValue;

impl GetWindowOptionTr for GetGlobalWindowOptionValue {}

impl GetUserOption for GetGlobalWindowOptionValue {}

impl GetOptionExt for GetGlobalWindowOptionValue {
    fn get_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().global().option(name).value();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
