use crate::{SetOption, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

/// Common trait for setting options, using [`SetOption`] command.
///
// Allows different implementations for different object options:
//  * [`ServerOptions`] (`set-option -s <NAME>`)
//  * [`SessionOptions`] (`set-option <NAME>`)
//  * [`WindowOptions`] (`set-option -w <NAME>`)
//  * [`PaneOptions`] (`set-option -p <NAME>`)
//
// TODO: optimize set/set_ext are the same
// NOTE: `SetOptionExt`, symbols `SetOption` already taken by tmux command
pub trait SetOptionExt {
    // XXX: unset if value = None, mb merge set_ext?
    //
    /// set:
    /// ```text
    /// set-option [-t target] <NAME> <VALUE>
    /// ```
    ///
    /// unset:
    /// ```text
    /// set-option [-t target] -u <NAME>
    /// ```
    fn set<'a, U, T, S>(target: Option<U>, name: T, value: Option<S>) -> TmuxCommand<'a>
    where
        U: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        let cmd = SetOption::new().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        let cmd = match value {
            Some(value) => cmd.value(value),
            None => cmd.unset(),
        };
        cmd.build()
    }

    /// unset:
    /// ```text
    /// set-option [-t target] -u <NAME>
    /// ```
    fn unset<'a, S, T>(target: Option<S>, name: T) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        let cmd = SetOption::new().option(name).unset();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }

    // fn set_ext<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
    //     target: Option<U>,
    //     name: T,
    //     value: Option<S>,
    // ) -> TmuxCommand<'a> {
    //     let cmd = SetOption::new().option(name);
    //     let cmd = match target {
    //         Some(target) => cmd.target(target),
    //         None => cmd,
    //     };
    //     let cmd = match value {
    //         Some(data) => cmd.value(data),
    //         None => cmd,
    //     };
    //     cmd.build()
    // }

    ///
    /// ```text
    /// set-option [-t target] <NAME>[0] <VALUE>
    /// set-option [-t target] <NAME>[1] <VALUE>
    /// set-option [-t target] <NAME>[2] <VALUE>
    /// set-option [-t target] <NAME>[...] <VALUE>
    /// ```
    ///
    /// ```text
    /// set-option [-t target] <NAME> ""
    /// ```
    ///
    // XXX: check value None branch tmux reaction
    fn set_array<'a, U, S, I, T>(target: Option<U>, name: S, value: Option<I>) -> TmuxCommands<'a>
    where
        S: Into<Cow<'a, str>> + std::fmt::Display,
        I: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>> + Copy,
    {
        let mut cmds = TmuxCommands::new();
        if let Some(value) = value {
            for (i, item) in value.into_iter().enumerate() {
                cmds.push(Self::set(target, format!("{}[{}]", name, i), Some(item)));
            }
        } else {
            cmds.push(Self::set(target, format!("{}", name), Some("")));
        }
        cmds
    }
}
