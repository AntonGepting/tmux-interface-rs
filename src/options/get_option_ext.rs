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
// NOTE: `GetOptionExt` complementary to `SetOptionExt`, symbols `SetOption`
// already taken by tmux command
pub trait GetOptionExt {
    fn get<'a, T, S>(target: Option<S>, name: T) -> TmuxCommand<'a>
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
}
