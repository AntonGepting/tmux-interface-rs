use crate::options::{GetOptionTr, GetSessionOptionTr};
use crate::{GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;

// NOTE: ADR: compile time or run time parametrisation for global local option set/get
// * compile time: trais
// * runtime: struct field with user given setter/getter

// TODO: all options exist in get/set?

pub struct GetGlobalSessionOption;

impl GetOptionTr for GetGlobalSessionOption {
    fn get_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().global().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }

    fn get_all<'a, S: Into<Cow<'a, str>>>(target: Option<S>) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().global();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl GetUserOption for GetGlobalSessionOption {}

impl GetSessionOptionTr for GetGlobalSessionOption {}
