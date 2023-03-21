use crate::options::{SetOptionExt, SetSessionOption, SetUserOption};
use crate::{SetOption, TmuxCommand};
use std::borrow::Cow;

pub struct SetLocalSessionOption;

impl SetOptionExt for SetLocalSessionOption {
    // unset if value = None
    fn set<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<U>,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        let cmd = match value {
            Some(value) => cmd.value(value),
            None => cmd.unset(),
        };
        cmd.build()
    }

    fn unset<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().option(name).unset();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl SetSessionOption for SetLocalSessionOption {}

impl SetUserOption for SetLocalSessionOption {}
