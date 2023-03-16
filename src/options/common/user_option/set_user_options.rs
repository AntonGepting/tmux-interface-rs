use crate::options::SetUserOption;
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait SetUserOptions<'a> {
    type Setter: SetUserOption;

    fn push(&mut self, option: TmuxCommand<'a>);

    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<U, S, T>(mut self, target: Option<U>, name: S, value: Option<T>) -> Self
    where
        Self: Sized,
        U: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        self.push(Self::Setter::user_option(target, name, value));
        self
    }
}
