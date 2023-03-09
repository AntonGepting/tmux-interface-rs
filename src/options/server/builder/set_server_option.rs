use crate::options::*;
use crate::{SetOption, SetOptionExt, TmuxCommand};
use std::borrow::Cow;

pub struct SetServerOption;

impl SetServerOptionTrait for SetServerOption {}

impl SetUserOption for SetServerOption {}

impl SetOptionExt for SetServerOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().server().option(name).unset().build()
    }

    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set_ext(name, value)
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().server().option(name).value(data),
            None => SetOption::new().server().option(name),
        };
        cmd.build()
    }
}

//impl SetOptionExt for SetServerOption {
//fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
//SetOption::new().server().option(name).unset().build()
//}

//fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
//name: T,
//value: Option<S>,
//) -> TmuxCommand<'a> {
//Self::set_ext(name, value)
//}

//// unset if value = None
//fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
//name: T,
//value: Option<S>,
//) -> TmuxCommand<'a> {
//let cmd = match value {
//Some(data) => SetOption::new().server().option(name).value(data),
//None => SetOption::new().server().option(name),
//};
//cmd.build()
//}
//}
