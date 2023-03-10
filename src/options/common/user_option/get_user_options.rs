use crate::options::GetUserOption;
use crate::TmuxCommand;
use std::borrow::Cow;

pub trait GetUserOptions<'a> {
    type Getter: GetUserOption;

    fn push(&mut self, option: TmuxCommand<'a>);

    /// ### Manual
    ///
    /// ```text
    /// @user-option-name value
    /// ```
    fn user_option<S: Into<Cow<'a, str>>>(mut self, name: S) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::user_option(name));
        self
    }
}
