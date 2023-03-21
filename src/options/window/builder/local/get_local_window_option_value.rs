use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetLocalWindowOptionValue;

impl GetWindowOption for GetLocalWindowOptionValue {}

impl GetUserOption for GetLocalWindowOptionValue {}

impl GetOptionExt for GetLocalWindowOptionValue {
    fn get<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        name: S,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().value().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
