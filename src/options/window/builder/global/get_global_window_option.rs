use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalWindowOption;

impl GetWindowOption for GetGlobalWindowOption {}

impl GetUserOption for GetGlobalWindowOption {}

impl GetOptionExt for GetGlobalWindowOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().window().option(name).global().build()
    }
}
