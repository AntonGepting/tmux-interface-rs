use crate::{GetServerOption, GetServerOptionTrait, GetUserOptions, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

#[derive(Debug)]
pub struct GetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetServerOptionsTrait<'a> for GetServerOptions<'a> {
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

pub trait GetServerOptionsTrait<'a> {
    type Getter: GetServerOptionTrait;

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
    fn backspace<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::backspace(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::buffer_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::command_alias(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::copy_command(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::default_terminal(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::escape_time(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::editor(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::exit_empty(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::exit_unattached(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::extended_keys(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::focus_events(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::history_file(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::message_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::prompt_history_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn set_clipboard<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::set_clipboard(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::terminal_features(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::terminal_overrides(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::user_keys(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::quiet(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy<S: Into<Cow<'a, str>>>(mut self, target: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Getter::detach_on_destroy(target));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
