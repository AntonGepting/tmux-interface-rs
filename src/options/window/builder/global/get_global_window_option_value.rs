use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalWindowOptionValue;

impl GetWindowOption for GetGlobalWindowOptionValue {}

impl GetUserOption for GetGlobalWindowOptionValue {}

impl GetOptionExt for GetGlobalWindowOptionValue {
    fn get<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().global().option(name).value();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
