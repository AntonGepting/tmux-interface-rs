use crate::{
    SetClipboard, SetServerOption, SetServerOptionTrait, SetUserOptions, Switch, TmuxCommand,
    TmuxCommands,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct SetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetServerOptionsTrait<'a> for SetServerOptions<'a> {
    type Setter = SetServerOption;

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

impl<'a> SetUserOptions<'a> for SetServerOptions<'a> {
    type Setter = SetServerOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}

pub trait SetServerOptionsTrait<'a> {
    type Setter: SetServerOptionTrait;

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
    fn backspace<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        backspace: Option<S>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::backspace(target, backspace));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    fn buffer_limit<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        buffer_limit: Option<usize>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::buffer_limit(target, buffer_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    fn command_alias<T, S>(mut self, target: Option<T>, command_alias: Option<Vec<S>>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>> + Clone,
        S: Into<Cow<'a, str>>,
    {
        self.push_cmds(Self::Setter::command_alias(target, command_alias));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn copy_command<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        copy_command: Option<S>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::copy_command(target, copy_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn default_terminal<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        default_terminal: Option<S>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_terminal(target, default_terminal));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn escape_time<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        escape_time: Option<usize>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::escape_time(target, escape_time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn editor<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        editor: Option<S>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::editor(target, editor));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    fn exit_empty<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        exit_empty: Option<Switch>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::exit_empty(target, exit_empty));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn exit_unattached<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        exit_unattached: Option<Switch>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::exit_unattached(target, exit_unattached));
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn extended_keys<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        extended_keys: Option<Switch>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::extended_keys(target, extended_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn focus_events<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        focus_events: Option<Switch>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::focus_events(target, focus_events));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn history_file<T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        history_file: Option<S>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::history_file(target, history_file));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn message_limit<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        message_limit: Option<usize>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_limit(target, message_limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    fn prompt_history_limit<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        prompt_history_limit: Option<usize>,
    ) -> Self
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
    fn set_clipboard<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        set_clipboard: Option<SetClipboard>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::set_clipboard(target, set_clipboard));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn terminal_features<T, S, I>(mut self, target: Option<T>, terminal_features: Option<I>) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::terminal_features(target, terminal_features));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn terminal_overrides<T, S>(
        mut self,
        target: Option<T>,
        terminal_overrides: Option<Vec<S>>,
    ) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>> + Clone,
        S: Into<Cow<'a, str>>,
    {
        self.push_cmds(Self::Setter::terminal_overrides(target, terminal_overrides));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn user_keys<T, S>(mut self, target: Option<T>, user_keys: Option<Vec<S>>) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::user_keys(target, user_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet<T: Into<Cow<'a, str>>>(mut self, target: Option<T>, quiet: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::quiet(target, quiet));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy<T: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        detach_on_destroy: Option<Switch>,
    ) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::detach_on_destroy(target, detach_on_destroy));
        self
    }

    // ### Manual
    //
    // ```text
    // user option
    // ```
    //fn user_option<S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
    //mut self,
    //name: S,
    //value: Option<T>,
    //) -> Self
    //where
    //Self: Sized,
    //{
    //self.push(Self::Setter::user_option(name, value));
    //self
    //}

    fn build(self) -> TmuxCommands<'a>;
}

//#[test]
//fn set_server_options_by_one() {
//let set = SetServerOptions::new()
//.buffer_limit(Some(50))
//.message_limit(Some(100))
//.set_clipboard(Some(SetClipboard::On))
//.user_option("a", Some("b"));

//dbg!(set);
////let get = Get::new().buffer_limit(&mut Option<buffer_limit>)
//}
