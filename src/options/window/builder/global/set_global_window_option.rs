use crate::{SetOption, TmuxCommand};
use crate::{SetOptionExt, SetUserOption, SetWindowOptionExt};
use std::borrow::Cow;

pub struct SetGlobalWindowOption;

//impl SetWindowOptionExt for SetGlobalWindowOption {
impl SetOptionExt for SetGlobalWindowOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new()
            .global()
            .window()
            .option(name)
            .unset()
            .build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().window().global().option(name).value(data),
            None => SetOption::new().window().global().option(name),
        };
        cmd.build()
    }
}

impl SetWindowOptionExt for SetGlobalWindowOption {}

impl SetUserOption for SetGlobalWindowOption {}
