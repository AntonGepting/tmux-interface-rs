use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetGlobalWindowOptionValue;

impl GetWindowOption for GetGlobalWindowOptionValue {}

impl GetUserOption for GetGlobalWindowOptionValue {}

impl GetOptionExt for GetGlobalWindowOptionValue {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .window()
            .value()
            .option(name)
            .global()
            .build()
    }
}
