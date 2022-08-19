use super::get_server_option::GetServerOption;
use crate::TmuxCommands;
use std::fmt;

#[derive(Debug)]
pub struct GetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetServerOptions<'a> {
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
    pub fn backspace(mut self) -> Self {
        self.options.push(GetServerOption::backspace());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(mut self) -> Self {
        self.options.push(GetServerOption::buffer_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(mut self) -> Self {
        self.options.push(GetServerOption::command_alias());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command(mut self) -> Self {
        self.options.push(GetServerOption::copy_command());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal(mut self) -> Self {
        self.options.push(GetServerOption::default_terminal());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(mut self) -> Self {
        self.options.push(GetServerOption::escape_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn editor(mut self) -> Self {
        self.options.push(GetServerOption::editor());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(mut self) -> Self {
        self.options.push(GetServerOption::exit_empty());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(mut self) -> Self {
        self.options.push(GetServerOption::exit_unattached());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(mut self) -> Self {
        self.options.push(GetServerOption::extended_keys());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(mut self) -> Self {
        self.options.push(GetServerOption::focus_events());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file(mut self) -> Self {
        self.options.push(GetServerOption::history_file());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(mut self) -> Self {
        self.options.push(GetServerOption::message_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(mut self) -> Self {
        self.options.push(GetServerOption::prompt_history_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(mut self) -> Self {
        self.options.push(GetServerOption::set_clipboard());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(mut self) -> Self {
        self.options.push(GetServerOption::terminal_features());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(mut self) -> Self {
        self.options.push(GetServerOption::terminal_overrides());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(mut self) -> Self {
        self.options.push(GetServerOption::user_keys());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(mut self) -> Self {
        self.options.push(GetServerOption::quiet());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(mut self) -> Self {
        self.options.push(GetServerOption::detach_on_destroy());
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<T: fmt::Display>(mut self, name: T) -> Self {
        self.options.push(GetServerOption::user_option(name));
        self
    }

    pub fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}
