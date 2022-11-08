use crate::options::*;
use crate::{Error, TmuxOutput};
use std::str::FromStr;

// mb common
fn parse_value<T: FromStr>(value: Option<&str>) -> Option<T> {
    value.and_then(|data| data.parse().ok())
}

impl<'a> From<TmuxOutput> for TmuxServerOptionOutput {
    fn from(tmux_output: TmuxOutput) -> Self {
        Self(tmux_output.to_string())
    }
}

#[derive(Debug)]
pub enum ServerOptionOutput {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    Backspace(Option<String>),
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    BufferLimit(Option<usize>),
    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    CommandAlias(Option<Vec<String>>),
    // default-terminal terminal
    #[cfg(feature = "tmux_2_1")]
    DefaultTerminal(Option<String>),
    //copy_command
    //escape-time time
    #[cfg(feature = "tmux_1_2")]
    EscapeTime(Option<usize>),
    //editor
    //exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    ExitEmpty(Option<Switch>),
    //exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    ExitUnattached(Option<Switch>),
    //extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    ExtendedKeys(Option<Switch>),
    //focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    FocusEvents(Option<Switch>),
    //history-file path
    #[cfg(feature = "tmux_2_1")]
    HistoryFile(Option<String>),
    //message-limit number
    #[cfg(feature = "tmux_2_0")]
    MessageLimit(Option<usize>),
    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    SetClipboard(Option<SetClipboard>),
    // terminal-features[]
    //terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    TerminalOverrides(Option<Vec<String>>),
    //user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    UserKeys(Option<Vec<String>>),
    // quiet ?
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    Quiet(Option<Switch>),
    // detach-on-destroy ?
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    DetachOnDestroy(Option<Switch>),
    // user option
    UserOption(Option<String>),
}

impl ServerOptionOutput {
    //pub get(name: ServerOptionName) -> Self {
    //match name {
    //}
    //}

    // split string in 3 parts, name, index (if option is an array) and value
    // TODO: rename
    pub fn get_option(s: &str) -> (Option<&str>, Option<usize>, Option<&str>) {
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

    //pub fn get_option_value(s: &str) -> (Option<&str>, Option<Vec<&str>>) {
    //}

    //pub fn parse_short() -> Result<> {
    //}

    pub fn parse_vec(
        _name: &str,
        _lines: &mut std::str::Lines,
        i: Option<usize>,
        value: Option<String>,
    ) -> Option<Vec<String>> {
        let mut v = Vec::new();

        if let Some(value) = value {
            if let Some(i) = i {
                v.insert(i, value.to_string());
            }
        }

        //for line in lines {
        //let (name, i, value) = Self::get_option(line);

        //unimplemented!();
        //if name != name {
        //lines.prev();
        //break;
        //}

        //if let Some(value) = value {
        //if let Some(i) = i {
        //v.insert(i, value.to_string());
        //}
        //}
        //}

        Some(v)
    }

    // parse single option with given name value and index
    pub fn parse_option(
        lines: &mut std::str::Lines,
        name: &str,
        i: Option<usize>,
        value: Option<&str>,
    ) -> Result<Self, Error> {
        match name {
            #[cfg(feature = "tmux_3_1")]
            BACKSPACE => Ok(Self::Backspace(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            BUFFER_LIMIT => Ok(Self::BufferLimit(parse_value(value))),
            #[cfg(feature = "tmux_2_4")]
            COMMAND_ALIAS => Ok(Self::CommandAlias(Self::parse_vec(
                COMMAND_ALIAS,
                lines,
                i,
                value.map(|s| s.to_string()),
            ))),
            #[cfg(feature = "tmux_2_1")]
            DEFAULT_TERMINAL => Ok(Self::DefaultTerminal(parse_value(value))),
            #[cfg(feature = "tmux_1_2")]
            ESCAPE_TIME => Ok(Self::EscapeTime(parse_value(value))),
            #[cfg(feature = "tmux_2_7")]
            EXIT_EMPTY => Ok(Self::ExitEmpty(parse_value(value))),
            #[cfg(feature = "tmux_1_4")]
            EXIT_UNATTACHED => Ok(Self::ExitUnattached(parse_value(value))),
            #[cfg(feature = "tmux_3_2")]
            EXTENDED_KEYS => Ok(Self::ExtendedKeys(parse_value(value))),
            #[cfg(feature = "tmux_1_9")]
            FOCUS_EVENTS => Ok(Self::FocusEvents(parse_value(value))),
            #[cfg(feature = "tmux_2_1")]
            HISTORY_FILE => Ok(Self::HistoryFile(parse_value(value))),
            #[cfg(feature = "tmux_2_0")]
            MESSAGE_LIMIT => Ok(Self::MessageLimit(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            SET_CLIPBOARD => Ok(Self::SetClipboard(parse_value(value))),
            #[cfg(feature = "tmux_2_0")]
            TERMINAL_OVERRIDES => Ok(Self::TerminalOverrides(Self::parse_vec(
                TERMINAL_OVERRIDES,
                lines,
                i,
                value.map(|s| s.to_string()),
            ))),
            #[cfg(feature = "tmux_3_0")]
            USER_KEYS => Ok(Self::UserKeys(Self::parse_vec(lines))),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            QUIET => Ok(Self::Quiet(parse_value(value))),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            DETACH_ON_DESTROY => Ok(Self::DetachOnDestroy(parse_value(value))),
            _ => Err(Error::ParseStatusKeys),
        }
    }
}

// single line
// multiple lines
impl FromStr for ServerOptionOutput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        match lines.next() {
            Some(line) => {
                let (name, i, value) = Self::get_option(line);

                match name {
                    Some(name) => Self::parse_option(&mut lines, name, i, value),
                    None => Err(Error::ParseStatusKeys),
                }
            }
            None => Err(Error::ParseStatusKeys),
        }
    }
}

//#[test]
//fn server_option_test222() {
//let output = r#"command-alias[0] "split-pane=split-window"
//command-alias[1] "splitp=split-window"
//command-alias[2] "server-info=show-messages -JT"
//command-alias[3] "info=show-messages -JT"
//command-alias[4] "choose-window=choose-tree -w"
//command-alias[5] "choose-session=choose-tree -s"
//"#;
//let option = ServerOptionOutput::from_str(output);
//dbg!(option);

//let output = "default-terminal \"screen-256color\"";
//let option = ServerOptionOutput::from_str(output);
//dbg!(option);

//let output = "escape-time 500";
//let option = ServerOptionOutput::from_str(output);
//dbg!(option);
//}

#[derive(Debug)]
pub struct TmuxServerOptionOutput(String);

impl TmuxServerOptionOutput {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace(&self) -> Result<String, ()> {
        Ok(self.0.trim_end())
    }

    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit<'a>(&self) -> Result<usize, Error> {
        Ok(self.0.trim_end().parse()?)
    }

    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    pub fn command_alias(&self) -> Vec<String> {
        let mut v = Vec::new();
        for item in self.0.lines() {
            v.push(item.to_string())
        }
        v
    }

    // default-terminal terminal
    #[cfg(feature = "tmux_3_2")]
    pub fn copy_command(&self) -> String {
        self.0.trim_end().to_string()
    }

    // copy-command shell-command
    #[cfg(feature = "tmux_2_1")]
    pub fn default_terminal(&self) -> String {
        self.0.trim_end().to_string()
    }

    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    pub fn escape_time(&self) -> Result<usize, Error> {
        Ok(self.0.trim_end().parse()?)
    }

    // editor shell-command
    #[cfg(feature = "tmux_3_2")]
    pub fn editor(&self) -> String {
        self.0.trim_end()
    }

    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    pub fn exit_empty(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    pub fn exit_unattached(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    pub fn extended_keys(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    pub fn focus_events(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // history-file path
    #[cfg(feature = "tmux_2_1")]
    pub fn history_file(&self) -> String {
        self.0.trim_end().to_string()
    }

    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    pub fn message_limit(&self) -> Result<usize, Error> {
        Ok(self.0.trim_end().parse()?)
    }

    // prompt-history-limit number
    #[cfg(feature = "tmux_3_3")]
    pub fn prompt_history_limit(&self) -> Result<usize, Error> {
        self.0.trim_end().parse()
    }

    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    pub fn set_clipboard(&self) -> Result<SetClipboard, Error> {
        self.0.trim_end().parse()
    }

    // terminal-features[] string
    #[cfg(feature = "tmux_3_2")]
    pub fn terminal_features(&self) -> Vec<String> {
        let mut v = Vec::new();
        for item in self.0.lines() {
            v.push(item.to_string())
        }
        v
    }

    // terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    pub fn terminal_overrides(&self) -> Vec<String> {
        let mut v = Vec::new();
        for item in self.0.lines() {
            v.push(item.to_string())
        }
        v
    }

    // user-keys[] key
    //#[cfg(feature = "tmux_3_0")]
    //pub fn user_keys(&self) -> {}

    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn quiet(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    pub fn detach_on_destroy(&self) -> Result<Switch, Error> {
        self.0.trim_end().parse()
    }

    // user option
    //pub fn user_option<S: fmt::Display>(&self, name: S) -> TmuxCommand<'a> {
    //}
}

//#[test]
//fn server_option33() {
//let server_options_str = r#"buffer-limit 50
//command-alias[0] "split-pane=split-window"
//command-alias[1] "splitp=split-window"
//command-alias[2] "server-info=show-messages -JT"
//command-alias[3] "info=show-messages -JT"
//command-alias[4] "choose-window=choose-tree -w"
//command-alias[5] "choose-session=choose-tree -s"
//default-terminal "screen-256color"
//escape-time 500
//exit-empty on
//exit-unattached off
//focus-events off
//history-file ""
//message-limit 100
//set-clipboard external
//terminal-overrides[0] "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q"
//terminal-overrides[1] "screen*:XT"
//user-keys
//"#;
//for line in server_options_str.lines() {
//let option = TmuxServerOptionOutputFull::from_str(line).unwrap();
//dbg!(&option);
//}
//}
