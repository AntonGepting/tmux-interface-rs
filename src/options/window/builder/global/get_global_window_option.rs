use crate::options::{GetOptionTr, GetUserOption, GetWindowOptionTr};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalWindowOption;

impl GetWindowOptionTr for GetGlobalWindowOption {}

impl GetUserOption for GetGlobalWindowOption {}

impl GetOptionTr for GetGlobalWindowOption {
    fn get_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().global().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }

    fn get_all<'a, S: Into<Cow<'a, str>>>(target: Option<S>) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().global();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
