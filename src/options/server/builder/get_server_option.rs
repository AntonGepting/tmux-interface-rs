use crate::options::*;
use crate::{GetOptionExt, ShowOptions, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct GetServerOption;

impl GetOptionExt for GetServerOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().server().option(name.into()).build()
    }
}

impl GetServerOptionTrait for GetServerOption {}

impl GetUserOption for GetServerOption {}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
pub trait GetServerOptionTrait: GetOptionExt + GetUserOption {
    //pub fn get<T: Into<Cow<'a, str>>>(&self, name: T) -> TmuxCommand<'a> {
    //(self.getter)(name.into())
    //}

    //pub fn gets<'a>(names: ServerOptionB) -> TmuxCommands<'a> {
    //let mut cmds = TmuxCommands::new();
    //for name in names.0 {
    //cmds.push(Self::get(name));
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
    fn backspace<'a>() -> TmuxCommand<'a> {
        Self::get(BACKSPACE)
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit<'a>() -> TmuxCommand<'a> {
        Self::get(BUFFER_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias<'a>() -> TmuxCommand<'a> {
        Self::get(COMMAND_ALIAS)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command<'a>() -> TmuxCommand<'a> {
        Self::get(COPY_COMMAND)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_TERMINAL)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time<'a>() -> TmuxCommand<'a> {
        Self::get(ESCAPE_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor<'a>() -> TmuxCommand<'a> {
        Self::get(EDITOR)
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty<'a>() -> TmuxCommand<'a> {
        Self::get(EXIT_EMPTY)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached<'a>() -> TmuxCommand<'a> {
        Self::get(EXIT_UNATTACHED)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys<'a>() -> TmuxCommand<'a> {
        Self::get(EXTENDED_KEYS)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events<'a>() -> TmuxCommand<'a> {
        Self::get(FOCUS_EVENTS)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file<'a>() -> TmuxCommand<'a> {
        Self::get(HISTORY_FILE)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit<'a>() -> TmuxCommand<'a> {
        Self::get(PROMPT_HISTORY_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    ///set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn set_clipboard<'a>() -> TmuxCommand<'a> {
        Self::get(SET_CLIPBOARD)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features<'a>() -> TmuxCommand<'a> {
        Self::get(TERMINAL_FEATURES)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides<'a>() -> TmuxCommand<'a> {
        Self::get(TERMINAL_OVERRIDES)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys<'a>() -> TmuxCommand<'a> {
        Self::get(USER_KEYS)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet<'a>() -> TmuxCommand<'a> {
        Self::get(QUIET)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy<'a>() -> TmuxCommand<'a> {
        Self::get(DETACH_ON_DESTROY)
    }
}
