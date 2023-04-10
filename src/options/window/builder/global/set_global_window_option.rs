use crate::{SetOption, TmuxCommand};
use crate::{SetOptionTr, SetUserOption, SetWindowOptionTr};
use std::borrow::Cow;

pub struct SetGlobalWindowOption;

//impl SetWindowOptionExt for SetGlobalWindowOption {
impl SetOptionTr for SetGlobalWindowOption {
    // unset if value = None
    fn set_ext<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<U>,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().window().global().option(name);
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

    fn unset_ext<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().window().global().option(name).unset();
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

impl SetWindowOptionTr for SetGlobalWindowOption {}

impl SetUserOption for SetGlobalWindowOption {}
