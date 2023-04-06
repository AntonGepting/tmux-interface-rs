use crate::{GetServerOption, GetServerOptionTr, GetUserOptions, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

#[derive(Debug)]
pub struct GetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetServerOptionsTr<'a> for GetServerOptions<'a> {
    type Getter = GetServerOption;

    fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }

    fn push_cmds(&mut self, options: TmuxCommands<'a>) {
        self.options.push_cmds(options);
    }

    fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}

impl<'a> GetUserOptions<'a> for GetServerOptions<'a> {
    type Getter = GetServerOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}

pub trait GetServerOptionsTr<'a> {
    type Getter: GetServerOptionTr;

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
    fn backspace(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::backspace());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::buffer_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::command_alias());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::copy_command());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::default_terminal());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::escape_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::editor());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::exit_empty());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::exit_unattached());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::extended_keys());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::focus_events());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::history_file());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::message_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::prompt_history_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn set_clipboard(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::set_clipboard());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::terminal_features());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::terminal_overrides());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::user_keys());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::quiet());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::detach_on_destroy());
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
