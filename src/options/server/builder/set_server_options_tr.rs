use crate::{SetClipboard, SetServerOptionTr, Switch, TmuxCommand, TmuxCommands};

use std::borrow::Cow;

pub trait SetServerOptionsTr<'a> {
    type Setter: SetServerOptionTr;

    fn new() -> Self;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn push_cmds(&mut self, options: TmuxCommands<'a>);

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    fn backspace<S: Into<Cow<'a, str>>>(mut self, backspace: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::backspace(backspace));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit(mut self, buffer_limit: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::buffer_limit(buffer_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias<S>(mut self, command_alias: Option<Vec<S>>) -> Self
    where
        Self: Sized,
        S: Into<Cow<'a, str>>,
    {
        self.push_cmds(Self::Setter::command_alias(command_alias));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command<S: Into<Cow<'a, str>>>(mut self, copy_command: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::copy_command(copy_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal<S: Into<Cow<'a, str>>>(mut self, default_terminal: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_terminal(default_terminal));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time(mut self, escape_time: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::escape_time(escape_time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor<S: Into<Cow<'a, str>>>(mut self, editor: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::editor(editor));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty(mut self, exit_empty: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::exit_empty(exit_empty));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached(mut self, exit_unattached: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::exit_unattached(exit_unattached));
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys(mut self, extended_keys: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::extended_keys(extended_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events(mut self, focus_events: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::focus_events(focus_events));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file<S: Into<Cow<'a, str>>>(mut self, history_file: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::history_file(history_file));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit(mut self, message_limit: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_limit(message_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit(mut self, prompt_history_limit: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(SetServerOption::prompt_history_limit(
            target,
            prompt_history_limit,
        ));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn set_clipboard(mut self, set_clipboard: Option<SetClipboard>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::set_clipboard(set_clipboard));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features<S, I>(mut self, terminal_features: Option<I>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::terminal_features(terminal_features));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides<S>(mut self, terminal_overrides: Option<Vec<S>>) -> Self
    where
        Self: Sized,
        S: Into<Cow<'a, str>>,
    {
        self.push_cmds(Self::Setter::terminal_overrides(terminal_overrides));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys<S>(mut self, user_keys: Option<Vec<S>>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::user_keys(user_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet<T: Into<Cow<'a, str>>>(mut self, quiet: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::quiet(quiet));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy<T: Into<Cow<'a, str>>>(mut self, detach_on_destroy: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::detach_on_destroy(detach_on_destroy));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
