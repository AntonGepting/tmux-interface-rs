use crate::options::*;
use crate::{GetOptionExt, GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;
use std::fmt;

pub struct GetPaneOptionValue;

impl GetOptionExt for GetPaneOptionValue {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .pane()
            .value()
            .option(name.into())
            .build()
    }
}

impl GetPaneOptionTrait for GetPaneOptionValue {}

impl GetUserOption for GetPaneOptionValue {}
