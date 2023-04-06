use crate::options::{SetOptionExt, USER_OPTION_MARKER};
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait SetUserOption: SetOptionExt {
    fn user_option<'a, S, T>(name: S, value: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set(format!("{}{}", USER_OPTION_MARKER, name.into()), value)
    }

    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option_ext<'a, U, S, T>(target: Option<U>, name: S, value: Option<T>) -> TmuxCommand<'a>
    where
        U: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(
            target,
            format!("{}{}", USER_OPTION_MARKER, name.into()),
            value,
        )
    }
}
