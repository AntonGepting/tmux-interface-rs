use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetLocalWindowOption;

impl GetWindowOption for GetLocalWindowOption {}

impl GetUserOption for GetLocalWindowOption {}

impl GetOptionExt for GetLocalWindowOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().window().option(name).build()
    }
}
