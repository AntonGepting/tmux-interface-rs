use crate::{SetOption, TmuxCommand, TmuxCommands};
use std::borrow::Cow;
use std::fmt;

// TODO: optimize set/set_ext are the same
/// common trait for setting options, allowing different implementations for different object options
pub trait SetOptionExt {
    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        match value {
            Some(data) => Self::set_ext(name, Some(data)),
            None => Self::unset(name),
        }
    }

    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        //(self.setter)(name.into(), value.map(|s| s.into()))
        let cmd = match value {
            Some(data) => SetOption::new().option(name).value(data),
            None => SetOption::new().option(name),
        };
        cmd.build()
    }

    fn set_array<'a, S, I, T>(name: S, value: Option<I>) -> TmuxCommands<'a>
    where
        S: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        let mut cmds = TmuxCommands::new();
        let name = name.into();
        if let Some(data) = value {
            for (i, item) in data.into_iter().enumerate() {
                cmds.push(Self::set(format!("{}[{}]", name, i), Some(item.into())));
            }
        } else {
            cmds.push(Self::set(format!("{}", name), Some("")));
        }
        cmds
    }

    fn set_array_original<'a, S: fmt::Display>(
        name: S,
        value: Option<Vec<String>>,
    ) -> TmuxCommands<'a> {
        let mut cmds = TmuxCommands::new();
        if let Some(data) = value {
            for (i, item) in data.iter().enumerate() {
                cmds.push(Self::set(format!("{}[{}]", name, i), Some(item.to_owned())));
            }
        } else {
            cmds.push(Self::set(format!("{}", name), Some("")));
        }
        cmds
    }

    // ### Manual
    //
    // ```text
    // user option
    // ```
    //fn user_option<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
    //name: S,
    //value: Option<T>,
    //) -> TmuxCommand<'a> {
    //Self::set(format!("{}{}", USER_OPTION_MARKER, name.into()), value)
    //}
}
