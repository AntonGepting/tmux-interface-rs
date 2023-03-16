use crate::options::{SetOptionExt, USER_OPTION_MARKER};
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait SetUserOption: SetOptionExt {
    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<'a, U, S, T>(target: Option<U>, name: S, value: Option<T>) -> TmuxCommand<'a>
    where
        U: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set(
            target,
            format!("{}{}", USER_OPTION_MARKER, name.into()),
            value,
        )
    }
}
