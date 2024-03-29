use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

/// Common trait for getting options, using [`ShowOptions`] command.
///
// Allows different implementations for different objects options:
//
//  * [`ServerOptions`] (`show-options -s <NAME>`)
//  * [`SessionOptions`] (`show-options <NAME>`)
//  * [`WindowOptions`] (`show-options -w <NAME>`)
//  * [`PaneOptions`] (`show-options -p <NAME>`)
//
// NOTE: `GetOptionTr` complementary to `SetOptionTr`, symbols `SetOption`
// already taken by tmux command builder
pub trait GetOptionTr {
    /// Command builder for get single option command using only option name
    ///
    /// # Arguments
    ///
    /// * `name` - option name
    ///
    /// # Manual
    ///
    /// ```text
    /// show-options name
    /// ```
    fn get<'a, S>(name: S) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        ShowOptions::new().option(name).build()
    }

    /// Command builder for get single option command using target and option name
    ///
    /// # Arguments
    ///
    /// * `target` - optional target server/session/window/pane
    /// * `name` - option name
    ///
    /// # Manual
    ///
    /// ```text
    /// show-options [-t target] name
    /// ```
    fn get_ext<'a, T, S>(target: Option<S>, name: T) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        let cmd = ShowOptions::new().option(name);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }

    /// Command builder for get all options command without specifying option name
    ///
    /// # Arguments
    ///
    /// * `target` - target server/session/window/pane
    ///
    /// # Manual
    ///
    /// ```text
    /// show-options [-t target]
    /// ```
    fn get_all<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        let cmd = ShowOptions::new();
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd,
        };
        cmd.build()
    }
}
