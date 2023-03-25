use crate::options::{SetOptionExt, SetSessionOption, SetUserOption};
use crate::{SetOption, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct SetGlobalSessionOption;

impl SetOptionExt for SetGlobalSessionOption {
    // unset if value = None
    fn set<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<U>,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().global().option(name);
        let cmd = match target {
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            Some(target) => cmd.target(target),
            #[cfg(feature = "tmux_3_0")]
            Some(target) => cmd.target_pane(target),
            None => cmd,
        };
        let cmd = match value {
            Some(value) => cmd.value(value),
            None => cmd.unset(),
        };
        cmd.build()
    }

    fn unset<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().global().option(name).unset();
        let cmd = match target {
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            Some(target) => cmd.target(target),
            #[cfg(feature = "tmux_3_0")]
            Some(target) => cmd.target_pane(target),
            None => cmd,
        };
        cmd.build()
    }
}

impl SetSessionOption for SetGlobalSessionOption {}

impl SetUserOption for SetGlobalSessionOption {}
