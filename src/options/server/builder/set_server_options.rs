use crate::{SetClipboard, SetServerOption, Switch, TmuxCommands};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
pub struct SetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetServerOptions<'a> {
    pub fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace<S: Into<Cow<'a, str>>>(mut self, backspace: S) -> Self {
        self.options.push(SetServerOption::backspace(backspace));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(mut self, buffer_limit: Option<usize>) -> Self {
        self.options
            .push(SetServerOption::buffer_limit(buffer_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(mut self, command_alias: Option<Vec<String>>) -> Self {
        self.options
            .push_cmds(SetServerOption::command_alias(command_alias));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command<S: Into<Cow<'a, str>>>(mut self, copy_command: Option<S>) -> Self {
        self.options
            .push(SetServerOption::copy_command(copy_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal<S: Into<Cow<'a, str>>>(mut self, default_terminal: Option<S>) -> Self {
        self.options
            .push(SetServerOption::default_terminal(default_terminal));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(mut self, escape_time: Option<usize>) -> Self {
        self.options.push(SetServerOption::escape_time(escape_time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn editor<S: Into<Cow<'a, str>>>(mut self, editor: Option<S>) -> Self {
        self.options.push(SetServerOption::editor(editor));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(mut self, exit_empty: Option<Switch>) -> Self {
        self.options.push(SetServerOption::exit_empty(exit_empty));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(mut self, exit_unattached: Option<Switch>) -> Self {
        self.options
            .push(SetServerOption::exit_unattached(exit_unattached));
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(mut self, extended_keys: Option<Switch>) -> Self {
        self.options
            .push(SetServerOption::extended_keys(extended_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(mut self, focus_events: Option<Switch>) -> Self {
        self.options
            .push(SetServerOption::focus_events(focus_events));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file<S: Into<Cow<'a, str>>>(mut self, history_file: Option<S>) -> Self {
        self.options
            .push(SetServerOption::history_file(history_file));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(mut self, message_limit: Option<usize>) -> Self {
        self.options
            .push(SetServerOption::message_limit(message_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(mut self, prompt_history_limit: Option<usize>) -> Self {
        self.options
            .push(SetServerOption::prompt_history_limit(prompt_history_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(mut self, set_clipboard: Option<SetClipboard>) -> Self {
        self.options
            .push(SetServerOption::set_clipboard(set_clipboard));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(mut self, terminal_features: Option<Vec<String>>) -> Self {
        self.options
            .push_cmds(SetServerOption::terminal_features(terminal_features));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(mut self, terminal_overrides: Option<Vec<String>>) -> Self {
        self.options
            .push_cmds(SetServerOption::terminal_overrides(terminal_overrides));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(mut self, user_keys: Option<Vec<String>>) -> Self {
        self.options
            .push_cmds(SetServerOption::user_keys(user_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(mut self, quiet: Option<Switch>) -> Self {
        self.options.push(SetServerOption::quiet(quiet));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(mut self, detach_on_destroy: Option<Switch>) -> Self {
        self.options
            .push(SetServerOption::detach_on_destroy(detach_on_destroy));
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<S: fmt::Display>(mut self, name: S, value: Option<String>) -> Self {
        self.options.push(SetServerOption::user_option(name, value));
        self
    }

    pub fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}

#[test]
fn set_server_options_by_one() {
    let set = SetServerOptions::new()
        .buffer_limit(Some(50))
        .message_limit(Some(100))
        .set_clipboard(Some(SetClipboard::On));

    dbg!(set);
    //let get = Get::new().buffer_limit(&mut Option<buffer_limit>)
}
