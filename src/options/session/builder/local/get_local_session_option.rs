use crate::options::{GetOptionExt, GetSessionOption};
use crate::{GetUserOption, ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetLocalSessionOption;

impl GetOptionExt for GetLocalSessionOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().option(name).build()
    }
}

impl GetUserOption for GetLocalSessionOption {}

impl GetSessionOption for GetLocalSessionOption {}
