use crate::{
    Error, GetServerOptionTr, GetServerOptionValue, GetUserOption, ServerOptions, SetClipboard,
    SetServerOption, SetServerOptionTr, SetServerOptions, SetServerOptionsTr, SetUserOption,
    ShowOptions, Switch, Tmux, TmuxCommand, TmuxOutput,
};
use std::borrow::Cow;
use std::str::FromStr;
//oneline
//multiline

// Output:
// * random (got buffer, need recognize fields)
//      * Form
//          * long (`option_name value`)
//          * short (`value`), single option get/parse (otherwise there is no chance to assign)
//
// * expected (got buffer, need to extract some fields)
//      * Form
//          * long (`option_name value`)
//          * short (`value`), single option get/parse (otherwise there is no chance to assign)
//

// get
// * get single one
// * get all for object
//
// set
// * set single one
//      * set value
//      * toggle (on/off {default}/off) if no value specified

// FIXME: proper Error in return
//
//
// ServerOption::backspace<S: Into<Cow<'a, str>>>(: Option<S>) -> Result<Self::Backspace(String), Error>
//  GetServerOption::backspace()
//  ParseServerOption::from_str()
//
// ServerOption::set().backspace<S: Into<Cow<'a, str>>>(: Option<S>) -> Result<(), Error> {
//  SetServerOption::backspace()
//  Output
// }

// XXX: rename ServerOptionCtl?
// trait top level options, then server session window pane
pub struct ServerOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for ServerOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> ServerOptionsCtl<'a> {
    pub fn new(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get_all(&self) -> Result<ServerOptions<'a>, Error> {
        Self::get_all_ext(self.invoker)
    }

    pub fn get_all_ext(
        invoker: impl FnOnce(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<ServerOptions<'a>, Error> {
        let cmd = ShowOptions::new().server().build();
        let output = invoker(cmd)?.to_string();
        ServerOptions::from_str(&output)
    }

    pub fn set_all(&self, server_options: ServerOptions<'a>) -> Result<TmuxOutput, Error> {
        Self::set_all_ext(self.invoker(), server_options)
    }

    pub fn set_all_ext(
        invoker: impl FnOnce(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
        server_options: ServerOptions<'a>,
    ) -> Result<TmuxOutput, Error> {
        let cmds = SetServerOptions::new();

        #[cfg(feature = "tmux_3_1")]
        let cmds = cmds.backspace(server_options.backspace);

        #[cfg(feature = "tmux_1_5")]
        let cmds = cmds.buffer_limit(server_options.buffer_limit);

        #[cfg(feature = "tmux_2_4")]
        let cmds = cmds.command_alias(server_options.command_alias);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.copy_command(server_options.copy_command);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.default_terminal(server_options.default_terminal);

        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.escape_time(server_options.escape_time);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.editor(server_options.editor);

        #[cfg(feature = "tmux_2_7")]
        let cmds = cmds.exit_empty(server_options.exit_empty);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.exit_unattached(server_options.exit_unattached);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.extended_keys(server_options.extended_keys);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.focus_events(server_options.focus_events);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.history_file(server_options.history_file);

        #[cfg(feature = "tmux_2_0")]
        let cmds = cmds.message_limit(server_options.message_limit);

        #[cfg(feature = "tmux_3_3")]
        let cmds = cmds.prompt_history_limit(server_options.prompt_history_limit);

        #[cfg(feature = "tmux_1_5")]
        let cmds = cmds.set_clipboard(server_options.set_clipboard);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.terminal_features(server_options.terminal_features);

        #[cfg(feature = "tmux_2_0")]
        let cmds = cmds.terminal_overrides(server_options.terminal_overrides);

        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.user_keys(server_options.user_keys);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let cmds = cmds.quiet(server_options.quiet);

        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
        let cmds = cmds.detach_on_destroy(server_options.detach_on_destroy);

        // `@USER_OPTION`

        let cmd = TmuxCommand::with_cmds(cmds.build());

        invoker(cmd)
    }

    // get and parse single line option
    pub fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error> {
        Ok((self.invoker)(cmd)?.to_string().trim().parse::<T>().ok())
    }

    pub fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        (self.invoker)(cmd)
    }

    // FIXME: full array support
    // Tmux binary
    //
    // 1. multiple binary call
    // tmux set -s command-alias[0] value0
    // tmux set -s command-alias[1] value1
    // tmux set -s command-alias[2] value2
    //
    // 2. single binary call
    // tmux set -s command-alias[0] value0 ; set -s command-alias[1] ; set -s command-alias[2]
    //
    // Control Mode
    //
    // 1. multiple control mode commands
    // set -s command-alias[0] value0
    // set -s command-alias[1] value1
    // set -s command-alias[2] value2
    //
    // 2. single control mode command
    // set -s command-alias[0] value0 ; set -s command-alias[1] ; set -s command-alias[2]
    //
    pub fn get_array(&self, get_option_cmd: TmuxCommand<'a>) -> Result<Option<Vec<String>>, Error> {
        let output = (self.invoker)(get_option_cmd)?;
        let v: Vec<String> = output
            .to_string()
            .lines()
            .map(|s| s.trim().into())
            .collect();
        let result = match v.is_empty() {
            true => None,
            false => Some(v),
        };
        Ok(result)
    }

    // pub fn set_array(&self, values: Vec<String>) -> Result<TmuxOutput, Error> {
    // let cmd = TmuxCommand::new();
    // let output = (self.invoker)(cmd);
    //
    // for (i, value) in values.iter().enumerate() {}
    //
    // output
    // }
}

impl<'a> ServerOptionsCtl<'a> {
    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn get_backspace(&self) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::backspace())
    }

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn set_backspace(&self, key: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::backspace(key))
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn get_buffer_limit(&self) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::buffer_limit())
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_buffer_limit(&self, buffer_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::buffer_limit(buffer_limit))
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn get_command_alias(&self) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::command_alias())
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn set_command_alias<I, S>(&self, command_alias: Option<I>) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::command_alias(
            command_alias,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_copy_command(&self) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::copy_command())
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_copy_command(&self, copy_command: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::copy_command(copy_command))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn get_default_terminal(&self) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::default_terminal())
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn set_default_terminal(
        &self,
        default_terminal: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::default_terminal(default_terminal))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn get_escape_time(&self) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::escape_time())
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn set_escape_time(&self, escape_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::escape_time(escape_time))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_editor(&self) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::editor())
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_editor<S>(&self, editor: Option<S>) -> Result<TmuxOutput, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        self.set(SetServerOption::editor(editor))
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn get_exit_empty(&self) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::exit_empty())
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn set_exit_empty(&self, exit_empty: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::exit_empty(exit_empty))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn get_exit_unattached(&self) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::exit_unattached())
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn set_exit_unattached(
        &self,
        exit_unattached: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::exit_unattached(exit_unattached))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_extended_keys(&self) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::extended_keys())
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_extended_keys(&self, extended_keys: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::extended_keys(extended_keys))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn get_focus_events(&self) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::focus_events())
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn set_focus_events(&self, focus_events: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::focus_events(focus_events))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn get_history_file(&self) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::history_file())
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn set_history_file(&self, history_file: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::history_file(history_file))
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn get_message_limit(&self) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::message_limit())
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn set_message_limit(&self, message_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::message_limit(message_limit))
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn get_prompt_history_limit(&self) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::prompt_history_limit())
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn set_prompt_history_limit(
        &self,
        prompt_history_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::prompt_history_limit(prompt_history_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn get_set_clipboard(&self) -> Result<Option<SetClipboard>, Error> {
        self.get(GetServerOptionValue::set_clipboard())
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_set_clipboard(
        &self,
        set_clipboard: Option<SetClipboard>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::set_clipboard(set_clipboard))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_terminal_features(&self) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::terminal_features())
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_terminal_features<I, S>(
        &self,
        terminal_features: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::terminal_features(
            terminal_features,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn get_terminal_overrides(&self) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::terminal_overrides())
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn set_terminal_overrides<I, S>(
        &self,
        terminal_overrides: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::terminal_overrides(
            terminal_overrides,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn get_user_keys(&self) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::user_keys())
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_user_keys<I, S>(&self, user_keys: Option<I>) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::user_keys(
            user_keys,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn get_quiet(&self) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::quiet())
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn set_quiet(&self, quiet: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::quiet(quiet))
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn get_detach_on_destroy(&self) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::detach_on_destroy())
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn set_detach_on_destroy(
        &self,
        detach_on_destroy: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::detach_on_destroy(detach_on_destroy))
    }

    // XXX: get/set multiple user options

    /// # Manual
    ///
    /// tmux:
    /// ```text
    /// @user-option-name value
    /// ```
    pub fn get_user_option<S: Into<Cow<'a, str>>>(&self, name: S) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::user_option(name))
    }

    /// # Manual
    ///
    /// tmux:
    /// ```text
    /// @user-option-name value
    /// ```
    pub fn set_user_option<S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        &self,
        name: S,
        value: Option<T>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::user_option(name, value))
    }
}
