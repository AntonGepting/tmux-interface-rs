use crate::{GetOptionTr, GetPaneOptionTr, GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetPaneOptionValue;

impl GetOptionTr for GetPaneOptionValue {
    fn get_ext<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
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

impl GetPaneOptionTr for GetPaneOptionValue {}

impl GetUserOption for GetPaneOptionValue {}
