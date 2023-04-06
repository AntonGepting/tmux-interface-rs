use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

// NOTE: ADR: compile time or run time parametrisation for global local option set/get
// * compile time: trais
// * runtime: struct field with user given setter/getter

// TODO: all options exist in get/set?
pub struct GetLocalWindowOptionValue;

impl GetWindowOption for GetLocalWindowOptionValue {}

impl GetUserOption for GetLocalWindowOptionValue {}

impl GetOptionExt for GetLocalWindowOptionValue {
    fn get_ext<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        name: S,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().value().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
