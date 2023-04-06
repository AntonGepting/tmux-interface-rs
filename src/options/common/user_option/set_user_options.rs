use crate::options::SetUserOption;
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait SetUserOptions<'a> {
    type Setter: SetUserOption;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn user_option<S, T>(mut self, name: S, value: Option<T>) -> Self
    where
        Self: Sized,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        self.push(Self::Setter::user_option(name, value));
        self
    }

    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option_ext<U, S, T>(mut self, target: Option<U>, name: S, value: Option<T>) -> Self
    where
        Self: Sized,
        U: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        self.push(Self::Setter::user_option_ext(target, name, value));
        self
    }
}
