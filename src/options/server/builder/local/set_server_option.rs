use crate::options::*;
use crate::{SetOption, SetOptionTr, TmuxCommand};
use std::borrow::Cow;

pub struct SetServerOption;

impl SetServerOptionTr for SetServerOption {}

impl SetUserOption for SetServerOption {}

impl SetOptionTr for SetServerOption {
    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().server().option(name);
        let cmd = match value {
            Some(value) => cmd.value(value),
            None => cmd.unset(),
        };
        cmd.build()
    }

    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        let cmd = SetOption::new().server().option(name).unset();
        cmd.build()
    }
}
