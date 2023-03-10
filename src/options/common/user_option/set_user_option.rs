use crate::options::{SetOptionExt, USER_OPTION_MARKER};
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait SetUserOption: SetOptionExt {
    /// ### Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        name: S,
        value: Option<T>,
    ) -> TmuxCommand<'a>
    where
        Self: Sized,
    {
        Self::set(format!("{}{}", USER_OPTION_MARKER, name.into()), value)
    }
}
