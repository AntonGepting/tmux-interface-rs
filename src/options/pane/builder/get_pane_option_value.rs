use crate::options::*;
use crate::{GetOptionExt, GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;
use std::fmt;

pub struct GetPaneOptionValue;

impl GetOptionExt for GetPaneOptionValue {
    fn get<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        name: S,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().pane().value().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl GetPaneOptionTrait for GetPaneOptionValue {}

impl GetUserOption for GetPaneOptionValue {}
