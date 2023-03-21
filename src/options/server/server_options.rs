//#[cfg(feature = "tmux_2_0")]
//use super::create_insert_vec;
use super::*;
use crate::options::common::{array_insert, cow_parse, get_parts};
use crate::{Error, Switch};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// XXX: mb methods for all fields set get?
// NOTE: not allows selective get by bitmask
// FIXME: tmux v1.6 option attribute not exists, all options will be showed
// XXX: mb custom serde serialize/deserialize?
// XXX: ugly array implementation?
// XXX: String vs &str, send/recieve like 2 structures
// XXX: String vs &str optimization
// TODO: Vec variables solution for arrays
// TODO: check types
// TODO: command_alias and terminal_overrides both as String and as Vec<String> see tmux versions
#[derive(PartialEq, Clone, Debug)]
pub struct ServerOptions<'a> {
    /// `backspace key`
    #[cfg(feature = "tmux_3_1")]
    pub backspace: Option<Cow<'a, str>>,
    /// `buffer-limit number`
    #[cfg(feature = "tmux_1_5")]
    pub buffer_limit: Option<usize>,
    /// `command-alias[] name=value`
    #[cfg(feature = "tmux_2_4")]
    pub command_alias: Option<Vec<Cow<'a, str>>>,
    /// `copy-command shell-command`
    #[cfg(feature = "tmux_3_2")]
    pub copy_command: Option<Cow<'a, str>>,
    /// `default-terminal terminal`
    #[cfg(feature = "tmux_2_1")]
    pub default_terminal: Option<Cow<'a, str>>,
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
    pub terminal_features: Option<Vec<Cow<'a, str>>>,
    /// `terminal-overrides[] string`
    #[cfg(feature = "tmux_2_0")]
    pub terminal_overrides: Option<Vec<Cow<'a, str>>>,
    /// `user-keys[] key`
    #[cfg(feature = "tmux_3_0")]
    pub user_keys: Option<Vec<Cow<'a, str>>>,
    /// `quiet [on | off]`
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub quiet: Option<Switch>,
    /// `detach-on-destroy [on | off]`
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub detach_on_destroy: Option<Switch>,
    /// `@user-option-name value`
    pub user_options: HashMap<String, Option<Cow<'a, str>>>,
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

// tmux.h
#[cfg(feature = "tmux_2_1")]
const DEFAULT_TERMINAL_DEFAULT: &str = "screen";

// compat.h
#[cfg(feature = "tmux_3_2")]
const EDITOR_DEFAULT: &str = "/usr/bin/vi";

#[cfg(feature = "tmux_1_5")]
const BUFFER_LIMIT_DEFAULT: usize = 50;

/// ```text
/// tmux show-options -g -s
/// ```
///
/// ```text
/// backspace C-?
/// buffer-limit 50
/// command-alias[0] split-pane=split-window
/// command-alias[1] splitp=split-window
/// command-alias[2] "server-info=show-messages -JT"
/// command-alias[3] "info=show-messages -JT"
/// command-alias[4] "choose-window=choose-tree -w"
/// command-alias[5] "choose-session=choose-tree -s"
/// default-terminal screen-256color
/// escape-time 500
/// exit-empty on
/// exit-unattached off
/// focus-events off
/// history-file
/// message-limit 100
/// set-clipboard external
/// terminal-overrides[0] "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q"
/// terminal-overrides[1] screen*:XT
/// user-keys
/// ```
impl<'a> Default for ServerOptions<'a> {
    fn default() -> Self {
        let options = ServerOptions::new();
        #[cfg(feature = "tmux_3_1")]
        let options = options.backspace(None);
        #[cfg(feature = "tmux_1_5")]
        let options = options.buffer_limit(Some(BUFFER_LIMIT_DEFAULT));
        #[cfg(feature = "tmux_2_4")]
        let options = options.command_alias(Some(vec![
            "split-pane=split-window,",
            "splitp=split-window,",
            "server-info=show-messages -JT,",
            "info=show-messages -JT,",
            "choose-window=choose-tree -w,",
            "choose-session=choose-tree -s",
        ]));
        #[cfg(feature = "tmux_2_1")]
        let options = options.default_terminal(Some(DEFAULT_TERMINAL_DEFAULT));
        #[cfg(feature = "tmux_3_2")]
        let options = options.editor(Some(EDITOR_DEFAULT));
        #[cfg(feature = "tmux_3_2")]
        let options = options.copy_command(Some(""));
        #[cfg(feature = "tmux_1_2")]
        let options = options.escape_time(Some(500));
        #[cfg(feature = "tmux_2_7")]
        let options = options.exit_empty(Some(Switch::On));
        #[cfg(feature = "tmux_1_4")]
        let options = options.exit_unattached(Some(Switch::Off));
        #[cfg(feature = "tmux_3_2")]
        let options = options.extended_keys(Some(0));
        #[cfg(feature = "tmux_1_9")]
        let options = options.focus_events(Some(Switch::Off));
        #[cfg(feature = "tmux_2_1")]
        let options = options.history_file(Some(""));
        #[cfg(feature = "tmux_2_0")]
        let options = options.message_limit(Some(1000));
        #[cfg(feature = "tmux_3_3")]
        let options = options.prompt_history_limit(Some(100));
        #[cfg(feature = "tmux_1_5")]
        let options = options.set_clipboard(Some(SetClipboard::Off));
        #[cfg(feature = "tmux_2_0")]
        let options = options.terminal_overrides(Some(vec![""]));
        #[cfg(feature = "tmux_3_2")]
        let options = options.terminal_features(Some(vec![
            "xterm*:clipboard:ccolour:cstyle:focus:title,",
            "screen*:title,",
            "rxvt*:ignorefkeys",
        ]));
        #[cfg(feature = "tmux_3_0")]
        let options = options.user_keys(Some(""));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
        let options = options.quiet(None);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
        let options = options.detach_on_destroy(None);
        options
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
        Self {
            #[cfg(feature = "tmux_3_1")]
            backspace: None,
            #[cfg(feature = "tmux_1_5")]
            buffer_limit: None,
            #[cfg(feature = "tmux_2_4")]
            command_alias: None,
            #[cfg(feature = "tmux_2_1")]
            default_terminal: None,
            #[cfg(feature = "tmux_3_2")]
            copy_command: None,
            #[cfg(feature = "tmux_1_2")]
            escape_time: None,
            #[cfg(feature = "tmux_2_7")]
            exit_empty: None,
            #[cfg(feature = "tmux_1_4")]
            exit_unattached: None,
            #[cfg(feature = "tmux_3_2")]
            extended_keys: None,
            #[cfg(feature = "tmux_1_9")]
            focus_events: None,
            #[cfg(feature = "tmux_2_1")]
            history_file: None,
            #[cfg(feature = "tmux_2_0")]
            message_limit: None,
            #[cfg(feature = "tmux_1_5")]
            set_clipboard: None,
            #[cfg(feature = "tmux_2_0")]
            terminal_overrides: None,
            #[cfg(feature = "tmux_3_0")]
            user_keys: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            quiet: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            detach_on_destroy: None,
            user_options: HashMap::new(),
        }
    }

    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// backspace key
    /// ```
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace<S: Into<Cow<'a, str>>>(mut self, backspace: Option<S>) -> Self {
        self.backspace = backspace.map(|s| s.into());
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
        self.buffer_limit = buffer_limit;
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// command-alias[] name=value
    /// ```
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias<I, S>(mut self, command_alias: Option<I>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.command_alias =
            command_alias.and_then(|v| Some(v.into_iter().map(|s| s.into()).collect()));
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
        self.copy_command = copy_command.map(|s| s.into());
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
        self.default_terminal = default_terminal.map(|s| s.into());
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
        self.escape_time = escape_time;
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
        self.editor = editor.map(|s| s.into());
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
        self.exit_empty = exit_empty;
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
        self.exit_unattached = exit_unattached;
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// extended-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(mut self, extended_keys: Option<Switch>) -> Self {
        self.extended_keys = extended_keys;
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
        self.focus_events = focus_events;
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
        self.history_file = history_file.map(|s| s.into());
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
        self.message_limit = message_limit;
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
        self.prompt_history_limit = prompt_history_limit;
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
        self.set_clipboard = set_clipboard;
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// terminal-features[] string
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features<I, S>(mut self, terminal_features: Option<I>) -> Self
    where
        I: Iterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.terminal_features =
            terminal_features.and_then(|v| Some(v.into_iter().map(|s| s.into()).collect()));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// terminal-overrides[] string
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides<I, S>(mut self, terminal_overrides: Option<I>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.terminal_overrides =
            terminal_overrides.and_then(|v| Some(v.into_iter().map(|s| s.into()).collect()));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn user_keys<I, S>(mut self, user_keys: Option<Vec<String>>) -> Self
    where
        I: Iterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.user_keys = user_keys.and_then(|v| Some(v.into_iter().map(|s| s.into()).collect()));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// quiet [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn quiet(mut self, quiet: Option<bool>) -> Self {
        self.quiet = quiet;
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    fn detach_on_destroy(mut self, detach_on_destroy: Option<bool>) -> Self {
        self.detach_on_destroy = detach_on_destroy;
        self
    }

    // `@USER_OPTION`
    //fn user_options(mut self, user_options: HashMap<String, String>) -> Self {
    //unimplemented!()
    //}
}

impl<'a> FromStr for ServerOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut server_options = ServerOptions::new();

        for line in s.lines() {
            if let Some((name, i, value)) = get_parts(line) {
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
                    _ => {
                        // if user option (@user_option value)
                        if let Some(name) = name.strip_prefix('@') {
                            server_options
                                .user_options
                                .insert(name.to_string(), cow_parse(value));
                        }
                    }
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
//
//
