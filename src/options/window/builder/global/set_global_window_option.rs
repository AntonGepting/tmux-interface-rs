use crate::{SetOption, TmuxCommand};
use crate::{SetOptionExt, SetUserOption, SetWindowOptionExt};
use std::borrow::Cow;

pub struct SetGlobalWindowOption;

//impl SetWindowOptionExt for SetGlobalWindowOption {
impl SetOptionExt for SetGlobalWindowOption {
    // unset if value = None
    fn set<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<U>,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().window().global().option(name);
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
        let cmd = SetOption::new().window().global().option(name).unset();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl SetWindowOptionExt for SetGlobalWindowOption {}

impl SetUserOption for SetGlobalWindowOption {}
