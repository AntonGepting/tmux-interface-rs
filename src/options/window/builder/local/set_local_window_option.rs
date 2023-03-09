use crate::options::{SetOptionExt, SetUserOption, SetWindowOptionExt};
use crate::{SetOption, TmuxCommand};
use std::borrow::Cow;

pub struct SetLocalWindowOption;

//impl SetWindowOptionExt for SetLocalWindowOption {
impl SetOptionExt for SetLocalWindowOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().window().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().window().option(name).value(data),
            None => SetOption::new().window().option(name),
        };
        cmd.build()
    }
}

impl SetWindowOptionExt for SetLocalWindowOption {}

impl SetUserOption for SetLocalWindowOption {}
