use crate::{GetOptionTr, GetPaneOptionTr, GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetPaneOption;

// XXX: get without option  name, get_ext with name?
impl GetOptionTr for GetPaneOption {
    fn get_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = ShowOptions::new().pane().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl GetPaneOptionTr for GetPaneOption {}

impl GetUserOption for GetPaneOption {}
