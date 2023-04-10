use crate::options::{GetOptionExt, GetSessionOptionTr};
use crate::{GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetLocalSessionOption;

impl GetOptionExt for GetLocalSessionOption {
    fn get_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }

    fn get_all<'a, S: Into<Cow<'a, str>>>(target: Option<S>) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl GetUserOption for GetLocalSessionOption {}

impl GetSessionOptionTr for GetLocalSessionOption {}
