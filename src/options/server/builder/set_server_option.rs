use crate::options::*;
use crate::{SetOption, SetOptionExt, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

pub struct SetServerOption;

impl SetServerOptionTrait for SetServerOption {}

impl SetUserOption for SetServerOption {}

impl SetOptionExt for SetServerOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().server().option(name).unset().build()
    }

    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set_ext(name, value)
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().server().option(name).value(data),
            None => SetOption::new().server().option(name),
        };
        cmd.build()
    }
}

//impl SetOptionExt for SetServerOption {
//fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
//SetOption::new().server().option(name).unset().build()
//}

//fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
//name: T,
//value: Option<S>,
//) -> TmuxCommand<'a> {
//Self::set_ext(name, value)
//}

//// unset if value = None
//fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
//name: T,
//value: Option<S>,
//) -> TmuxCommand<'a> {
//let cmd = match value {
//Some(data) => SetOption::new().server().option(name).value(data),
//None => SetOption::new().server().option(name),
//};
//cmd.build()
//}
//}

pub trait SetServerOptionTrait: SetOptionExt {
    //fn set_array<'a, S: fmt::Display>(name: S, value: Option<Vec<String>>) -> TmuxCommands<'a> {
    //let mut cmds = TmuxCommands::new();
    //if let Some(data) = value {
    //for (i, item) in data.iter().enumerate() {
    //cmds.push(Self::set(format!("{}[{}]", name, i), Some(item.to_owned())));
    //}
    //} else {
    //cmds.push(Self::set(format!("{}", name), Some("")));
    //}
    //cmds
    //}

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    fn backspace<'a, S: Into<Cow<'a, str>>>(backspace: Option<S>) -> TmuxCommand<'a> {
        Self::set(BACKSPACE, backspace)
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit<'a>(buffer_limit: Option<usize>) -> TmuxCommand<'a> {
        Self::set(BUFFER_LIMIT, buffer_limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias<'a, I, S>(command_alias: Option<I>) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(COMMAND_ALIAS, command_alias)
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias_ext<'a, S: Into<Cow<'a, str>>>(
        i: usize,
        command_alias: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(format!("{}[{}]", COMMAND_ALIAS, i), command_alias)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command<'a, S: Into<Cow<'a, str>>>(copy_command: Option<S>) -> TmuxCommand<'a> {
        Self::set(COPY_COMMAND, copy_command)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal<'a, S: Into<Cow<'a, str>>>(default_terminal: Option<S>) -> TmuxCommand<'a> {
        Self::set(DEFAULT_TERMINAL, default_terminal)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time<'a>(escape_time: Option<usize>) -> TmuxCommand<'a> {
        Self::set(ESCAPE_TIME, escape_time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor<'a, S: Into<Cow<'a, str>>>(editor: Option<S>) -> TmuxCommand<'a> {
        Self::set(EDITOR, editor)
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty<'a>(exit_empty: Option<Switch>) -> TmuxCommand<'a> {
        Self::set_ext(EXIT_EMPTY, exit_empty.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached<'a>(exit_unattached: Option<Switch>) -> TmuxCommand<'a> {
        Self::set_ext(EXIT_UNATTACHED, exit_unattached.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys<'a>(extended_keys: Option<Switch>) -> TmuxCommand<'a> {
        Self::set_ext(EXTENDED_KEYS, extended_keys.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events<'a>(focus_events: Option<Switch>) -> TmuxCommand<'a> {
        Self::set_ext(FOCUS_EVENTS, focus_events.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file<'a, S: Into<Cow<'a, str>>>(history_file: Option<S>) -> TmuxCommand<'a> {
        Self::set(HISTORY_FILE, history_file)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit<'a>(message_limit: Option<usize>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_LIMIT, message_limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit<'a>(prompt_history_limit: Option<usize>) -> TmuxCommand<'a> {
        Self::set(
            PROMPT_HISTORY_LIMIT,
            prompt_history_limit.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn set_clipboard<'a>(set_clipboard: Option<SetClipboard>) -> TmuxCommand<'a> {
        Self::set_ext(SET_CLIPBOARD, set_clipboard.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features<'a, I, S>(terminal_features: Option<I>) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(TERMINAL_FEATURES, terminal_features)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides<'a, I, S>(terminal_overrides: Option<I>) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(TERMINAL_OVERRIDES, terminal_overrides)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys<'a, I, S>(user_keys: Option<Vec<S>>) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(USER_KEYS, user_keys)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet<'a>(quiet: Option<Switch>) -> TmuxCommand<'a> {
        Self::set_ext(QUIET, quiet.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy<'a>(detach_on_destroy: Option<Switch>) -> TmuxCommand<'a> {
        Self::set_ext(DETACH_ON_DESTROY, detach_on_destroy.map(|s| s.to_string()))
    }
}
