use crate::options::GetUserOption;
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait GetUserOptions<'a> {
    type Getter: GetUserOption;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn user_option<S>(mut self, name: S) -> Self
    where
        Self: Sized,
        S: Into<Cow<'a, str>>,
    {
        self.push(Self::Getter::user_option(name));
        self
    }

    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option_ext<T, S>(mut self, target: Option<T>, name: S) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        self.push(Self::Getter::user_option_ext(target, name));
        self
    }
}
