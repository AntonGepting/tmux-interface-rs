use crate::options::{GetOptionExt, GetUserOption, GetWindowOption};
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

pub struct GetLocalWindowOptionValue;

impl GetWindowOption for GetLocalWindowOptionValue {}

impl GetUserOption for GetLocalWindowOptionValue {}

impl GetOptionExt for GetLocalWindowOptionValue {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().window().value().option(name).build()
    }
}
