use crate::options::*;
use crate::{SetOption, SetOptionExt, TmuxCommand};
use std::borrow::Cow;

pub struct SetServerOption;

impl SetServerOptionTrait for SetServerOption {}

impl SetUserOption for SetServerOption {}

impl SetOptionExt for SetServerOption {
    fn set<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<U>,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().server().option(name);
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
        let cmd = SetOption::new().server().option(name).unset();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
