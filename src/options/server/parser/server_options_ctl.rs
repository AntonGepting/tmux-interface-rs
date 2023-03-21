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
// ServerOption::backspace<S: Into<Cow<'a, str>>>(target: Option<S>) -> Result<Self::Backspace(String), Error>
//  GetServerOption::backspace()
//  ParseServerOption::from_str()
//
// ServerOption::set().backspace<S: Into<Cow<'a, str>>>(target: Option<S>) -> Result<(), Error> {
//  SetServerOption::backspace()
//  Output
// }

use crate::{
    Error, GetServerOptionTrait, GetServerOptionValue, ServerOptions, SetClipboard,
    SetServerOption, SetServerOptionTrait, SetServerOptions, SetServerOptionsTrait, ShowOptions,
    Switch, Tmux, TmuxCommand, TmuxCommands, TmuxOutput,
};

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
    pub invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    pub target: Option<Cow<'a, str>>,
}

impl<'a> Default for ServerOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: |cmd| Tmux::with_command(cmd).output(),
            target: None,
        }
    }
}

impl<'a> ServerOptionsCtl<'a> {
    pub fn new(
        invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
        target: Option<Cow<'a, str>>,
    ) -> Self {
        Self { invoker, target }
    }

    //pub fn get<S: Into<Cow<'a, str>>>(&self, target: Option<S>) -> Result<Self, Error> {
    //let mut cmd = ShowOptions::new().server().build();
    //let output = self.invoker(&mut cmd);
    //dbg!(&output);
    //ServerOptions::from_str(&output)
    //}

    pub fn get_all(&self) -> Result<ServerOptions<'a>, Error> {
        let cmd = ShowOptions::new().server().build();
        let output = (self.invoker)(cmd)?.to_string();
        ServerOptions::from_str(&output)
    }

    pub fn get_all_ext(
        invoke: &dyn Fn(&mut TmuxCommand<'a>) -> String,
    ) -> Result<ServerOptions<'a>, Error> {
        let mut cmd = ShowOptions::new().server().build();
        let output = invoke(&mut cmd);
        ServerOptions::from_str(&output)
    }

    pub fn set_all_ext(
        self,
        invoke: &dyn Fn(&TmuxCommands<'a>) -> Result<String, Error>,
        server_options: ServerOptions<'a>,
    ) -> Result<String, Error> {
        let cmds = SetServerOptions::new();

        #[cfg(feature = "tmux_3_1")]
        let cmds = cmds.backspace(self.target.clone(), server_options.backspace);

        #[cfg(feature = "tmux_1_5")]
        let cmds = cmds.buffer_limit(self.target.clone(), server_options.buffer_limit);

        #[cfg(feature = "tmux_2_4")]
        let cmds = cmds.command_alias(self.target.clone(), server_options.command_alias);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.copy_command(self.target.clone(), server_options.copy_command);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.default_terminal(self.target.clone(), server_options.default_terminal);

        #[cfg(feature = "tmux_1_2")]
        let cmds = cmds.escape_time(self.target.clone(), server_options.escape_time);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.editor(self.target.clone(), server_options.editor);

        #[cfg(feature = "tmux_2_7")]
        let cmds = cmds.exit_empty(self.target.clone(), server_options.exit_empty);

        #[cfg(feature = "tmux_1_4")]
        let cmds = cmds.exit_unattached(self.target.clone(), server_options.exit_unattached);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.extended_keys(self.target.clone(), server_options.extended_keys);

        #[cfg(feature = "tmux_1_9")]
        let cmds = cmds.focus_events(self.target.clone(), server_options.focus_events);

        #[cfg(feature = "tmux_2_1")]
        let cmds = cmds.history_file(self.target.clone(), server_options.history_file);

        #[cfg(feature = "tmux_2_0")]
        let cmds = cmds.message_limit(self.target.clone(), server_options.message_limit);

        #[cfg(feature = "tmux_3_3")]
        let cmds =
            cmds.prompt_history_limit(self.target.clone(), server_options.prompt_history_limit);

        #[cfg(feature = "tmux_1_5")]
        let cmds = cmds.set_clipboard(self.target.clone(), server_options.set_clipboard);

        #[cfg(feature = "tmux_3_2")]
        let cmds = cmds.terminal_features(self.target.clone(), server_options.terminal_features);

        #[cfg(feature = "tmux_2_0")]
        let cmds = cmds.terminal_overrides(self.target.clone(), server_options.terminal_overrides);

        #[cfg(feature = "tmux_3_0")]
        let cmds = cmds.user_keys(self.target.clone(), server_options.user_keys);

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let cmds = cmds.quiet(self.target.clone(), server_options.quiet);

        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
        let cmds = cmds.detach_on_destroy(self.target.clone(), server_options.detach_on_destroy);

        // `@USER_OPTION`

        let cmds = cmds.build();

        invoke(&cmds)
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
    pub fn get_backspace<S: Into<Cow<'a, str>>>(
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::backspace(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn set_backspace<T: Into<Cow<'a, str>>>(
        target: Option<T>,
        key: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::backspace(target, key))
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn get_buffer_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::buffer_limit(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// buffer-limit number
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_buffer_limit<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        buffer_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::buffer_limit(target, buffer_limit))
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn get_command_alias<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::command_alias(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn set_command_alias<S, I, T>(
        &self,
        target: Option<S>,
        command_alias: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        S: Into<Cow<'a, str>> + Copy,
        I: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::command_alias(
            target,
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
    pub fn get_copy_command<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::copy_command(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// copy-command shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_copy_command<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        copy_command: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::copy_command(target, copy_command))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn get_default_terminal<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::default_terminal(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn set_default_terminal<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        default_terminal: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::default_terminal(target, default_terminal))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn get_escape_time<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::escape_time(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// escape-time time
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn set_escape_time<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        escape_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::escape_time(target, escape_time))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_editor<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::editor(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// editor shell-command
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_editor<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        editor: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::editor(target, editor))
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn get_exit_empty<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::exit_empty(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.7:
    /// ```text
    /// exit-empty [on | off]
    /// ```
    #[cfg(feature = "tmux_2_7")]
    pub fn set_exit_empty<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        exit_empty: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::exit_empty(target, exit_empty))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn get_exit_unattached<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::exit_unattached(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// exit-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn set_exit_unattached<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        exit_unattached: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::exit_unattached(target, exit_unattached))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_extended_keys<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::extended_keys(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_extended_keys<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        extended_keys: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::extended_keys(target, extended_keys))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn get_focus_events<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::focus_events(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// focus-events [on | off]
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn set_focus_events<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        focus_events: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::focus_events(target, focus_events))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn get_history_file<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<String>, Error> {
        self.get(GetServerOptionValue::history_file(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// history-file path
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn set_history_file<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        history_file: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::history_file(target, history_file))
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn get_message_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::message_limit(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn set_message_limit<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        message_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::message_limit(target, message_limit))
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn get_prompt_history_limit<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<usize>, Error> {
        self.get(GetServerOptionValue::prompt_history_limit(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.3:
    /// ```text
    /// prompt-history-limit number
    /// ```
    #[cfg(feature = "tmux_3_3")]
    pub fn set_prompt_history_limit<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        prompt_history_limit: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::prompt_history_limit(
            target,
            prompt_history_limit,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn get_set_clipboard<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<SetClipboard>, Error> {
        self.get(GetServerOptionValue::set_clipboard(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.5:
    /// ```text
    /// set-clipboard [on | external | off]
    /// ```
    #[cfg(feature = "tmux_1_5")]
    pub fn set_set_clipboard<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        set_clipboard: Option<SetClipboard>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::set_clipboard(target, set_clipboard))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn get_terminal_features<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::terminal_features(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn set_terminal_features<I, T>(
        &self,
        target: Option<S>,
        terminal_features: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::terminal_features(
            target,
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
    pub fn get_terminal_overrides<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::terminal_overrides(target))
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn set_terminal_overrides<I, T, U>(
        &self,
        target: Option<U>,
        terminal_overrides: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>> + Copy,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::terminal_overrides(
            target,
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
    pub fn get_user_keys<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Vec<String>>, Error> {
        self.get_array(GetServerOptionValue::user_keys(target))
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn set_user_keys<U, I, T>(
        &self,
        target: Option<U>,
        user_keys: Option<I>,
    ) -> Result<TmuxOutput, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.set(TmuxCommand::with_cmds(SetServerOption::user_keys(
            target, user_keys,
        )))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn get_quiet<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::quiet(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn set_quiet<T: Into<Cow<'a, str>>>(
        &self,
        target: Option<T>,
        quiet: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::quiet(target, quiet))
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn get_detach_on_destroy<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
    ) -> Result<Option<Switch>, Error> {
        self.get(GetServerOptionValue::detach_on_destroy(target))
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn set_detach_on_destroy<S: Into<Cow<'a, str>>>(
        &self,
        target: Option<S>,
        detach_on_destroy: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(SetServerOption::detach_on_destroy(
            target,
            detach_on_destroy,
        ))
    }
}
