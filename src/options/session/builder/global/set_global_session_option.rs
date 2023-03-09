use crate::options::{SetOptionExt, SetSessionOption, SetUserOption};
use crate::{SetOption, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct SetGlobalSessionOption;

impl SetOptionExt for SetGlobalSessionOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().global().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().global().option(name).value(data),
            None => SetOption::new().global().option(name),
        };
        cmd.build()
    }
}

impl SetSessionOption for SetGlobalSessionOption {}

impl SetUserOption for SetGlobalSessionOption {}
