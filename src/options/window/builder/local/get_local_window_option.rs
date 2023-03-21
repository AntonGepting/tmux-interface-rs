use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetLocalWindowOption;

impl GetWindowOption for GetLocalWindowOption {}

impl GetUserOption for GetLocalWindowOption {}

impl GetOptionExt for GetLocalWindowOption {
    fn get<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().window().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
