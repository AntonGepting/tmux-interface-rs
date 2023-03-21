use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalWindowOption;

impl GetWindowOption for GetGlobalWindowOption {}

impl GetUserOption for GetGlobalWindowOption {}

impl GetOptionExt for GetGlobalWindowOption {
    fn get<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
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
}
