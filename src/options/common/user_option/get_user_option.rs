use crate::options::{GetOptionTr, USER_OPTION_MARKER};
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait GetUserOption: GetOptionTr {
    fn user_option<'a, S>(name: S) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get(format!("{}{}", USER_OPTION_MARKER, name.into()))
    }

    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option_ext<'a, T, S>(target: Option<T>, name: S) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, format!("{}{}", USER_OPTION_MARKER, name.into()))
    }
}
