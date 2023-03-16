use crate::options::GetUserOption;
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait GetUserOptions<'a> {
    type Getter: GetUserOption;

    fn push(&mut self, option: TmuxCommand<'a>);

    /// # Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<T, S>(mut self, target: Option<T>, name: S) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        self.push(Self::Getter::user_option(target, name));
        self
    }
}
