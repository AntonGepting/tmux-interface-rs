//#[cfg(feature = "tmux_2_0")]
//use super::create_insert_vec;
use super::*;
use crate::{Error, ShowOptions, Switch, Tmux, TmuxCommand, TmuxCommands};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct OptionsArray<'a>(Vec<Cow<'a, str>>);

impl<'a> OptionsArray<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push<S: Into<Cow<'a, str>>>(&mut self, item: S) {
        self.0.push(item.into());
    }

    pub fn insert<S: Into<Cow<'a, str>>>(&mut self, i: usize, item: S) {
        self.0.insert(i, item.into());
    }

    pub fn to_string_ext<S: fmt::Display>(self, name: S) -> String {
        let mut s = String::new();
        for (i, item) in self.0.iter().enumerate() {
            s += &format!("{}[{}] {}\n", name, i, item);
        }
        s
    }
}

#[test]
fn options_array() {
    let mut a = OptionsArray::new();
    a.push("asdf1");
    a.push("asdf2");
    let result = a.to_string_ext(TERMINAL_OVERRIDES);
    dbg!(result);
}

//#[derive(Default, PartialEq, Clone, Debug)]
//pub struct TmuxOption<N, T> {
//pub name: N,
//pub value: T,
//}

//// array name needed
//impl<'a> fmt::Display for ServerOptions<'a> {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//let mut v = Vec::new();
//for item in self.0 {
//v.push
//}
//}
//}

//impl<'a> FromStr for ServerOptions<'a> {
//type Err = Error;

//fn from_str(s: &str) -> Result<Self, Self::Err> {
////for option in s.lines() {
////let (name, i, value) = get_option(option);
////}
//}
//}

// XXX: mb custom serde serialize/deserialize?
// XXX: ugly array implementation?
// XXX: String vs &str, send/recieve like 2 structures
// XXX: String vs &str optimization
// TODO: Vec variables solution for arrays
// TODO: check types
// TODO: command_alias and terminal_overrides both as String and as Vec<String> see tmux versions
#[derive(Default, PartialEq, Clone, Debug)]
pub struct ServerOptions<'a> {
    /// `backspace key`
    #[cfg(feature = "tmux_3_1")]
    pub backspace: Option<Cow<'a, str>>,
    /// `buffer-limit number`
    #[cfg(feature = "tmux_1_5")]
    pub buffer_limit: Option<usize>,
    /// `command-alias[] name=value`
    #[cfg(feature = "tmux_2_4")]
    pub command_alias: Option<Vec<String>>,
    /// `default-terminal terminal`
    #[cfg(feature = "tmux_2_1")]
    pub default_terminal: Option<Cow<'a, str>>,
    /// `copy-command shell-command`
    #[cfg(feature = "tmux_3_2")]
    pub copy_command: Option<Cow<'a, str>>,
    /// `escape-time time`
    #[cfg(feature = "tmux_1_2")]
    pub escape_time: Option<usize>,
    /// `editor shell-command`
    #[cfg(feature = "tmux_3_2")]
    pub editor: Option<Cow<'a, str>>,
    /// `exit-empty [on | off]`
    #[cfg(feature = "tmux_2_7")]
    pub exit_empty: Option<Switch>,
    /// `exit-unattached [on | off]`
    #[cfg(feature = "tmux_1_4")]
    pub exit_unattached: Option<Switch>,
    /// `extended-keys [on | off]`
    #[cfg(feature = "tmux_3_2")]
    pub extended_keys: Option<Switch>,
    /// `focus-events [on | off]`
    #[cfg(feature = "tmux_1_9")]
    pub focus_events: Option<Switch>,
    /// `history-file path`
    #[cfg(feature = "tmux_2_1")]
    pub history_file: Option<Cow<'a, str>>,
    /// `message-limit number`
    #[cfg(feature = "tmux_2_0")]
    pub message_limit: Option<usize>,
    /// `prompt-history-limit number`
    #[cfg(feature = "tmux_3_3")]
    pub prompt_history_limit: Option<usize>,
    /// `set-clipboard [on | external | off]`
    #[cfg(feature = "tmux_1_5")]
    pub set_clipboard: Option<SetClipboard>,
    /// `terminal-features[] string`
    #[cfg(feature = "tmux_3_2")]
    pub terminal_features: Option<Vec<String>>,
    /// `terminal-overrides[] string`
    #[cfg(feature = "tmux_2_0")]
    pub terminal_overrides: Option<Vec<String>>,
    /// `user-keys[] key`
    #[cfg(feature = "tmux_3_0")]
    pub user_keys: Option<Vec<String>>,
    /// `quiet [on | off]`
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub quiet: Option<Switch>,
    /// `detach-on-destroy [on | off]`
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub detach_on_destroy: Option<Switch>,
    // `@USER_OPTION`
    //pub user_options: Option<HashMap<String, String>>
}

fn option_to_string<S: fmt::Display>(v: &mut Vec<String>, name: &str, value: &Option<S>) {
    if let Some(data) = value {
        v.push(format!("{} {}", name, data))
    }
}

fn option_array_to_string<S: fmt::Display>(
    v: &mut Vec<String>,
    name: &str,
    value: &Option<Vec<S>>,
) {
    if let Some(data) = value {
        for item in data {
            v.push(format!("{} {}", name, item))
        }
    }
}

impl<'a> fmt::Display for ServerOptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        #[cfg(feature = "tmux_3_1")]
        option_to_string(&mut v, BACKSPACE, &self.backspace);
        #[cfg(feature = "tmux_1_5")]
        option_to_string(&mut v, BUFFER_LIMIT, &self.buffer_limit);
        #[cfg(feature = "tmux_2_4")]
        option_array_to_string(&mut v, COMMAND_ALIAS, &self.command_alias);
        #[cfg(feature = "tmux_2_1")]
        option_to_string(&mut v, DEFAULT_TERMINAL, &self.default_terminal);
        #[cfg(feature = "tmux_3_2")]
        option_to_string(&mut v, COPY_COMMAND, &self.copy_command);
        #[cfg(feature = "tmux_1_2")]
        option_to_string(&mut v, ESCAPE_TIME, &self.escape_time);
        #[cfg(feature = "tmux_2_7")]
        option_to_string(&mut v, EXIT_EMPTY, &self.exit_empty);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, EXIT_UNATTACHED, &self.exit_unattached);
        #[cfg(feature = "tmux_3_2")]
        option_to_string(&mut v, EXTENDED_KEYS, &self.extended_keys);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, FOCUS_EVENTS, &self.focus_events);
        #[cfg(feature = "tmux_2_1")]
        option_to_string(&mut v, HISTORY_FILE, &self.history_file);
        #[cfg(feature = "tmux_2_0")]
        option_to_string(&mut v, MESSAGE_LIMIT, &self.message_limit);
        #[cfg(feature = "tmux_1_5")]
        option_to_string(&mut v, SET_CLIPBOARD, &self.set_clipboard);
        #[cfg(feature = "tmux_2_0")]
        option_array_to_string(&mut v, TERMINAL_OVERRIDES, &self.command_alias);
        #[cfg(feature = "tmux_3_0")]
        option_array_to_string(&mut v, USER_KEYS, &self.user_keys);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, QUIET, &self.quiet);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
        option_to_string(&mut v, DETACH_ON_DESTROY, &self.detach_on_destroy);
        let s = v.join("\n");
        write!(f, "{}", s)
    }
}

// TODO not needed at all? methods for main struct
// XXX: Cow?
impl<'a> ServerOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `backspace key`
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace<S: Into<Cow<'a, str>>>(mut self, backspace: S) -> Self {
        self.backspace = Some(backspace.into());
        self
    }

    /// `buffer-limit number`
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(mut self, buffer_limit: usize) -> Self {
        self.buffer_limit = Some(buffer_limit);
        self
    }

    /// `command-alias[] name=value`
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(mut self, command_alias: Vec<&str>) -> Self {
        self.command_alias = Some(command_alias.iter().map(|s| (*s).to_string()).collect());
        self
    }

    /// `default-terminal terminal`
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal<S: Into<Cow<'a, str>>>(mut self, default_terminal: S) -> Self {
        self.default_terminal = Some(default_terminal.into());
        self
    }

    /// `copy-command shell-command`
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command<S: Into<Cow<'a, str>>>(mut self, copy_command: S) -> Self {
        self.copy_command = Some(copy_command.into());
        self
    }

    /// `escape-time time`
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(mut self, escape_time: usize) -> Self {
        self.escape_time = Some(escape_time);
        self
    }

    /// `editor shell-command`
    #[cfg(feature = "tmux_3_2")]
    pub fn editor<S: Into<Cow<'a, str>>>(mut self, editor: S) -> Self {
        self.editor = Some(editor.into());
        self
    }

    /// `exit-empty [on | off]`
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(mut self, exit_empty: Switch) -> Self {
        self.exit_empty = Some(exit_empty);
        self
    }

    /// `exit-unattached [on | off]`
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(mut self, exit_unattached: Switch) -> Self {
        self.exit_unattached = Some(exit_unattached);
        self
    }

    /// `extended-keys [on | off]`
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(mut self, extended_keys: Switch) -> Self {
        self.extended_keys = Some(extended_keys);
        self
    }

    /// `focus-events [on | off]`
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(mut self, focus_events: Switch) -> Self {
        self.focus_events = Some(focus_events);
        self
    }

    /// `history-file path`
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file<S: Into<Cow<'a, str>>>(mut self, history_file: S) -> Self {
        self.history_file = Some(history_file.into());
        self
    }

    /// `message-limit number`
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(mut self, message_limit: usize) -> Self {
        self.message_limit = Some(message_limit);
        self
    }

    // `prompt-history-limit number`

    /// `set-clipboard [on | external | off]`
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(mut self, set_clipboard: SetClipboard) -> Self {
        self.set_clipboard = Some(set_clipboard);
        self
    }

    // `terminal-features[] string`

    /// `terminal-overrides[] string`
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(mut self, terminal_overrides: Vec<&str>) -> Self {
        self.terminal_overrides = Some(
            terminal_overrides
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        );
        self
    }

    /// `user-keys[] key`
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(mut self, user_keys: Vec<String>) -> Self {
        self.user_keys = Some(user_keys);
        self
    }

    // `quiet [on | off]`

    // `detach-on-destroy [on | off]`

    // `@USER_OPTION`
    //pub user_options: Option<HashMap<String, String>>
}

//pub struct ServerOptionsController {
//pub getter:
//pub setter:
//pub get_global:
//}

pub struct GetServerOptions<'a> {
    pub getter: &'a dyn Fn() -> TmuxCommand<'a>,
}

impl<'a> GetServerOptions<'a> {
    pub fn default_get_server_options() -> TmuxCommand<'a> {
        ShowOptions::new().server().build()
    }

    pub fn default_get_server_global_options() -> TmuxCommand<'a> {
        ShowOptions::new().server().global().build()
    }

    pub fn new() -> Self {
        Self {
            getter: &Self::default_get_server_options,
        }
    }

    pub fn new_global() -> Self {
        Self {
            getter: &Self::default_get_server_global_options,
        }
    }
}

pub struct SetServerOptions<'a> {
    pub setter: SetServerOption<'a>,
}

impl<'a> SetServerOptions<'a> {
    pub fn new() -> Self {
        Self {
            setter: SetServerOption::new(),
        }
    }

    // allows selective set by bitmask
    //pub fn set(self) -> Result<(), Error> {
    //self.set_ext(SetServerOption::default())
    //}
}

impl<'a> ServerOptions<'a> {
    //#[cfg(feature = "tmux_1_5")]
    //pub fn buffer_limit(&self) -> TmuxCommand<'a> {
    //if let Some(buffer_limit) = self.buffer_limit {
    //SetServerOption::new().buffer_limit()
    //}
    //}

    pub fn set(self) -> TmuxCommands<'a> {
        let mut cmds = TmuxCommands::new();

        #[cfg(feature = "tmux_3_1")]
        cmds.push(SetServerOption::new().backspace(self.backspace));

        #[cfg(feature = "tmux_1_5")]
        cmds.push(SetServerOption::new().buffer_limit(self.buffer_limit));

        #[cfg(feature = "tmux_2_4")]
        cmds.push_cmds(SetServerOption::new().command_alias(self.command_alias));

        #[cfg(feature = "tmux_2_1")]
        cmds.push(SetServerOption::new().default_terminal(self.default_terminal));

        #[cfg(feature = "tmux_3_2")]
        cmds.push(SetServerOption::new().copy_command(self.copy_command));

        #[cfg(feature = "tmux_1_2")]
        cmds.push(SetServerOption::new().escape_time(self.escape_time));

        #[cfg(feature = "tmux_3_2")]
        cmds.push(SetServerOption::new().editor(self.editor));

        // NOTE: unwrap Option then wrap again (reason: toggle if None, set if Some)
        #[cfg(feature = "tmux_2_7")]
        cmds.push(SetServerOption::new().exit_empty(self.exit_empty));

        #[cfg(feature = "tmux_1_4")]
        cmds.push(SetServerOption::new().exit_unattached(self.exit_unattached));

        #[cfg(feature = "tmux_3_2")]
        cmds.push(SetServerOption::new().extended_keys(self.extended_keys));

        #[cfg(feature = "tmux_1_9")]
        cmds.push(SetServerOption::new().focus_events(self.focus_events));

        #[cfg(feature = "tmux_2_1")]
        cmds.push(SetServerOption::new().history_file(self.history_file));

        #[cfg(feature = "tmux_2_0")]
        cmds.push(SetServerOption::new().message_limit(self.message_limit));

        #[cfg(feature = "tmux_3_3")]
        cmds.push(SetServerOption::new().prompt_history_limit(self.prompt_history_limit));

        #[cfg(feature = "tmux_1_5")]
        cmds.push(SetServerOption::new().set_clipboard(self.set_clipboard));

        #[cfg(feature = "tmux_3_2")]
        cmds.push(SetServerOption::new().terminal_features(self.terminal_features));

        #[cfg(feature = "tmux_2_0")]
        cmds.push_cmds(SetServerOption::new().terminal_overrides(self.terminal_overrides));

        #[cfg(feature = "tmux_3_0")]
        cmds.push(SetServerOption::new().user_keys(self.user_keys));

        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        cmds.push(SetServerOption::new().quiet(self.quiet));

        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
        cmds.push(SetServerOption::new().detach_on_destroy(self.detach_on_destroy));

        // `@USER_OPTION`

        cmds
    }
}

#[test]
fn server_options23() {
    let s = Tmux::new().commands(
        ServerOptions::new()
            .message_limit(10)
            .buffer_limit(50)
            .set(),
    );
    dbg!(s.build().to_string());
}

pub struct SetServerOptionsByOne<'a> {
    pub cmds: TmuxCommands<'a>,
}

impl<'a> SetServerOptionsByOne<'a> {
    pub fn new() -> Self {
        Self {
            cmds: TmuxCommands::new(),
        }
    }

    // backspace key
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace<S: Into<Cow<'a, str>>>(mut self, backspace: S) -> Self {
        self.cmds.push(SetServerOption::new().backspace(backspace));
        self
    }

    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(mut self, buffer_limit: Option<usize>) -> Self {
        self.cmds
            .push(SetServerOption::new().buffer_limit(buffer_limit));
        self
    }

    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(mut self, command_alias: Option<Vec<String>>) -> Self {
        self.cmds
            .push_cmds(SetServerOption::new().command_alias(command_alias));
        self
    }

    // default-terminal terminal
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command<S: Into<Cow<'a, str>>>(mut self, copy_command: Option<S>) -> Self {
        self.cmds
            .push(SetServerOption::new().copy_command(copy_command));
        self
    }

    // copy-command shell-command
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal<S: Into<Cow<'a, str>>>(mut self, default_terminal: Option<S>) -> Self {
        self.cmds
            .push(SetServerOption::new().default_terminal(default_terminal));
        self
    }

    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(mut self, escape_time: Option<usize>) -> Self {
        self.cmds
            .push(SetServerOption::new().escape_time(escape_time));
        self
    }

    // editor shell-command
    #[cfg(feature = "tmux_3_2")]
    pub fn editor<S: Into<Cow<'a, str>>>(mut self, editor: Option<S>) -> Self {
        self.cmds.push(SetServerOption::new().editor(editor));
        self
    }

    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(mut self, exit_empty: Option<Switch>) -> Self {
        self.cmds
            .push(SetServerOption::new().exit_empty(exit_empty));
        self
    }

    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(mut self, exit_unattached: Option<Switch>) -> Self {
        self.cmds
            .push(SetServerOption::new().exit_unattached(exit_unattached));
        self
    }

    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(mut self, extended_keys: Option<Switch>) -> Self {
        self.cmds
            .push(SetServerOption::new().extended_keys(extended_keys));
        self
    }

    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(mut self, focus_events: Option<Switch>) -> Self {
        self.cmds
            .push(SetServerOption::new().focus_events(focus_events));
        self
    }

    // history-file path
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file<S: Into<Cow<'a, str>>>(mut self, history_file: Option<S>) -> Self {
        self.cmds
            .push(SetServerOption::new().history_file(history_file));
        self
    }

    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(mut self, message_limit: Option<usize>) -> Self {
        self.cmds
            .push(SetServerOption::new().message_limit(message_limit));
        self
    }

    // prompt-history-limit number
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(mut self, prompt_history_limit: Option<usize>) -> Self {
        self.cmds
            .push(SetServerOption::new().prompt_history_limit(prompt_history_limit));
        self
    }

    // set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(mut self, set_clipboard: Option<SetClipboard>) -> Self {
        self.cmds
            .push(SetServerOption::new().set_clipboard(set_clipboard));
        self
    }

    // terminal-features[] string
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(mut self, terminal_features: Option<Vec<String>>) -> Self {
        self.cmds
            .push_cmds(SetServerOption::new().terminal_features(terminal_features));
        self
    }

    // terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(mut self, terminal_overrides: Option<Vec<String>>) -> Self {
        self.cmds
            .push_cmds(SetServerOption::new().terminal_overrides(terminal_overrides));
        self
    }

    // user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys(mut self, user_keys: Option<Vec<String>>) -> Self {
        self.cmds
            .push_cmds(SetServerOption::new().user_keys(user_keys));
        self
    }

    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(mut self, quiet: Option<Switch>) -> Self {
        self.cmds.push(SetServerOption::new().quiet(quiet));
        self
    }

    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(mut self, detach_on_destroy: Option<Switch>) -> Self {
        self.cmds
            .push(SetServerOption::new().detach_on_destroy(detach_on_destroy));
        self
    }

    // user option
    pub fn user_option<S: fmt::Display>(mut self, name: S, value: Option<String>) -> Self {
        self.cmds
            .push(SetServerOption::new().user_option(name, value));
        self
    }
}

#[test]
fn set_server_options_by_one() {
    let set = SetServerOptionsByOne::new()
        .buffer_limit(50)
        .set_clipboard(Switch::On);

    //let get = Get::new().buffer_limit(&mut Option<buffer_limit>)
}

// XXX: bitmask is overkill now, mb later use for multiple select
// NOTE: not allows selective get by bitmask
// FIXME: tmux v1.6 option attribute not exists, all options will be showed
//#[cfg(feature = "tmux_1_7")]
//pub fn get(bitflags: usize) -> Result<Self, Error> {
//let selected_option = SERVER_OPTIONS
//.iter()
//.filter(|t| bitflags == t.3)
//.map(|t| t.0.to_string())
//.collect::<Vec<String>>()
//.join(" ");
//ShowOptions::new()
//.server()
//.option(&selected_option)
//.build()
////.into_tmux_bin_command()
////.output()?
//.to_string()
//.parse()
//}

// XXX: add selective multiple vars and single methods
//pub fn get(bitflags: usize) -> Result<Self, Error> {
//let mut tmux = TmuxInterface::new();
//for selected_option in SERVER_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
//let show_options = ShowOptionsBuilder::<TargetPane>::new()
//.server()
//.option(&selected_option.0)
//.build();
//let s = tmux.show_options(Some(&show_options))?;
//s.parse::<ServerOptions>();
//}
//Err(Error::new(""))
//}

//pub fn set_array(name: &str, value: Option<Vec<String>>) {
//if let Some(data) = value {
//for (i, item) in data.iter().enumerate() {
//let array_name = format!("{}[{}]", name, i);
////Self::set(&array_name, Some(item));
//Tmux::new()
//.command(SetOption::new().server().option(array_name).value(item))
//.output()
//.unwrap();
//}
//}
//}

// XXX: mb methods for all fields set get?

//pub fn parse_single_option(s: &str) -> Self {
//}
//}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?

const SEPARATOR: &str = " ";

// split string in 3 parts, name, index (if option is an array) and value
fn get_option(s: &str) -> (Option<&str>, Option<usize>, Option<&str>) {
    let v: Vec<&str> = s.trim().splitn(2, SEPARATOR).collect();
    let value = v.get(1).copied();
    let mut index: Option<usize> = None;
    let mut name = v.get(0).copied();

    if let Some(name_array) = name {
        let v: Vec<&str> = name_array.split(|c| c == '[' || c == ']').collect();
        name = v.get(0).copied();
        index = v.get(1).and_then(|i| i.parse().ok());
    }

    (name, index, value)
}

fn array_insert<T: FromStr>(v: &mut Option<Vec<T>>, index: Option<usize>, value: Option<T>) {
    index.and_then(|i| value.and_then(|data| Some(v.get_or_insert(Vec::new()).insert(i, data))));
}

fn cow_parse<'a>(value: Option<&str>) -> Option<Cow<'a, str>> {
    value.and_then(|s| Some(Cow::Owned(s.into())))
}

impl<'a> FromStr for ServerOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut server_options = ServerOptions::default();

        for option in s.lines() {
            let (name, i, value) = get_option(option);
            //dbg!((s, option, name, i, value));
            if let Some(name) = name {
                //let option = option.parse::<ServerOption>()?;
                match name {
                    #[cfg(feature = "tmux_3_1")]
                    BACKSPACE => server_options.backspace = cow_parse(value),
                    #[cfg(feature = "tmux_1_5")]
                    BUFFER_LIMIT => {
                        server_options.buffer_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_4")]
                    COMMAND_ALIAS => array_insert(
                        &mut server_options.command_alias,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
                    #[cfg(feature = "tmux_2_1")]
                    DEFAULT_TERMINAL => server_options.default_terminal = cow_parse(value),
                    #[cfg(feature = "tmux_3_2")]
                    COPY_COMMAND => server_options.copy_command = cow_parse(value),
                    #[cfg(feature = "tmux_1_2")]
                    ESCAPE_TIME => server_options.escape_time = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_3_2")]
                    EDITOR => server_options.editor = cow_parse(value),
                    #[cfg(feature = "tmux_2_7")]
                    EXIT_EMPTY => server_options.exit_empty = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_4")]
                    EXIT_UNATTACHED => {
                        server_options.exit_unattached = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_2")]
                    EXTENDED_KEYS => {
                        server_options.extended_keys = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    FOCUS_EVENTS => {
                        server_options.focus_events = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_1")]
                    HISTORY_FILE => server_options.history_file = cow_parse(value),
                    #[cfg(feature = "tmux_2_0")]
                    MESSAGE_LIMIT => {
                        server_options.message_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_3")]
                    PROMPT_HISTORY_LIMIT => {
                        server_options.prompt_history_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_5")]
                    SET_CLIPBOARD => {
                        server_options.set_clipboard = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_2")]
                    TERMINAL_FEATURES => array_insert(
                        &mut server_options.terminal_features,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
                    #[cfg(feature = "tmux_2_0")]
                    TERMINAL_OVERRIDES => array_insert(
                        &mut server_options.terminal_overrides,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
                    #[cfg(feature = "tmux_3_0")]
                    USER_KEYS => array_insert(
                        &mut server_options.user_keys,
                        i,
                        value.and_then(|s| s.parse().ok()),
                    ),
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
                    QUIET => server_options.quiet = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
                    DETACH_ON_DESTROY => {
                        server_options.detach_on_destroy = value.and_then(|s| s.parse().ok())
                    }
                    _ => return Err(Error::Tmux(format!("unknown option: {}", name))),
                }
            }
        }
        Ok(server_options)
    }
}

// universal insert in vec
//pub fn insert_vec<T: >(server_options: &mut ServerOptions, i: usize, value: Option<T>) {
//match value {
//Some((i, data)) => server_options
//.command_alias
//.get_or_insert(Vec::new())
//.insert(i, data),
//None => (),
//};
//}
