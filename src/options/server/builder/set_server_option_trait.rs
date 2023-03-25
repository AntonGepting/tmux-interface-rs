use crate::options::*;
use crate::{SetOptionExt, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

//impl SetOptionExt for SetServerOption {
//fn unset<'a, T: Into<Cow<'a, str>>>(target: Option<T>, name: T) -> TmuxCommand<'a> {
//SetOption::new().server().option(name).unset(target, ).build()
//}

//fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(target: Option<T>,
//name: T,
//value: Option<S>,
//) -> TmuxCommand<'a> {
//Self::set_ext(name, value)
//}

//// unset if value = None
//fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(target: Option<T>,
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
    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    fn backspace<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        backspace: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, BACKSPACE, backspace)
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        buffer_limit: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, BUFFER_LIMIT, buffer_limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias<'a, T: Into<Cow<'a, str>>, I, S>(
        target: Option<T>,
        command_alias: Option<I>,
    ) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>> + Clone,
    {
        Self::set_array(target, COMMAND_ALIAS, command_alias)
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        i: usize,
        command_alias: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, format!("{}[{}]", COMMAND_ALIAS, i), command_alias)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        copy_command: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, COPY_COMMAND, copy_command)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        default_terminal: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DEFAULT_TERMINAL, default_terminal)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        escape_time: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, ESCAPE_TIME, escape_time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        editor: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, EDITOR, editor)
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        exit_empty: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, EXIT_EMPTY, exit_empty.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        exit_unattached: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
            EXIT_UNATTACHED,
            exit_unattached.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys<'a, T>(target: Option<T>, extended_keys: Option<Switch>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::set(target, EXTENDED_KEYS, extended_keys.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        focus_events: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, FOCUS_EVENTS, focus_events.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        history_file: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, HISTORY_FILE, history_file)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        message_limit: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_LIMIT, message_limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        prompt_history_limit: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
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
    fn set_clipboard<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        set_clipboard: Option<SetClipboard>,
    ) -> TmuxCommand<'a> {
        Self::set(target, SET_CLIPBOARD, set_clipboard.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features<'a, T, I, S>(
        target: Option<T>,
        terminal_features: Option<I>,
    ) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>> + Clone,
    {
        Self::set_array(target, TERMINAL_FEATURES, terminal_features)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides<'a, T, I, S>(
        target: Option<T>,
        terminal_overrides: Option<I>,
    ) -> TmuxCommands<'a>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>> + Clone,
    {
        Self::set_array(target, TERMINAL_OVERRIDES, terminal_overrides)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys<'a, T, I, S>(target: Option<T>, user_keys: Option<I>) -> TmuxCommands<'a>
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(target, USER_KEYS, user_keys)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        quiet: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set_ext(target, QUIET, quiet.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        detach_on_destroy: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set_ext(
            target,
            DETACH_ON_DESTROY,
            detach_on_destroy.map(|s| s.to_string()),
        )
    }
}
