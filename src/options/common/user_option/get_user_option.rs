use crate::options::{GetOptionExt, USER_OPTION_MARKER};
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait GetUserOption: GetOptionExt {
    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<'a, T, S>(target: Option<T>, name: S) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        Self::get(target, format!("{}{}", USER_OPTION_MARKER, name.into()))
    }
}
