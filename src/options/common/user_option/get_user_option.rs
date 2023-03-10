use crate::options::{GetOptionExt, USER_OPTION_MARKER};
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait GetUserOption: GetOptionExt {
    /// ### Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<'a, S: Into<Cow<'a, str>>>(name: S) -> TmuxCommand<'a> {
        Self::get(format!("{}{}", USER_OPTION_MARKER, name.into()))
    }
}
