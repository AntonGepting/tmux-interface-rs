use crate::options::SetUserOption;
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait SetUserOptions<'a> {
    type Setter: SetUserOption;

    fn push(&mut self, option: TmuxCommand<'a>);

    /// ### Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        mut self,
        name: S,
        value: Option<T>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::user_option(name, value));
        self
    }
}
