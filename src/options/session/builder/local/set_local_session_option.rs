use crate::options::{SetOptionExt, SetSessionOption, SetUserOption};
use crate::{SetOption, TmuxCommand};
use std::borrow::Cow;

pub struct SetLocalSessionOption;

impl SetOptionExt for SetLocalSessionOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().option(name).value(data),
            None => SetOption::new().option(name),
        };
        cmd.build()
    }
}

impl SetSessionOption for SetLocalSessionOption {}

impl SetUserOption for SetLocalSessionOption {}
