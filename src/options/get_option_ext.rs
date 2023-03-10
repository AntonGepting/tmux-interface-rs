use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

/// common trait for getting options, allowing different implementations for different object options
pub trait GetOptionExt {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().option(name).value().build()
    }
}
