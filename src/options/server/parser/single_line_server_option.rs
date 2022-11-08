use crate::options::*;
use crate::Error;
use std::str::FromStr;

//pub struct TmuxServerOptionOutputFull;

// variants possible:
// * option_name value
// * option_name
// * option_name[i] value
//

fn parse_array_value<T: FromStr>(index: Option<usize>, value: Option<&str>) -> Option<(usize, T)> {
    match index {
        Some(i) => value
            .and_then(|data| data.parse().ok())
            .map(|data| (i, data)),
        None => None,
    }
}

// XXX: generic for user types
//fn parse_hashmap_value<T: FromStr>(value: Option<&str>) -> Option<T> {
//value.and_then(|data| data.parse().ok())
//}

// mb common
fn parse_value<T: FromStr>(value: Option<&str>) -> Option<T> {
    value.and_then(|data| data.parse().ok())
}

#[derive(Debug)]
pub enum SingleLineServerOption {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    Backspace(Option<String>),
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    BufferLimit(Option<usize>),
    // command-alias[] name=value
    //CommandAlias(Option<Vec<String>>),
    //CommandAlias(Option<ArrayItem<String>>),
    #[cfg(feature = "tmux_2_4")]
    CommandAlias(Option<(usize, String)>),
    // copy_command
    #[cfg(feature = "tmux_3_2")]
    CopyCommand(Option<String>),
    // default-terminal terminal
    #[cfg(feature = "tmux_2_1")]
    DefaultTerminal(Option<String>),
    // escape-time time
    #[cfg(feature = "tmux_1_2")]
    EscapeTime(Option<usize>),
    // editor
    #[cfg(feature = "tmux_3_2")]
    Editor(Option<String>),
    // exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    ExitEmpty(Option<Switch>),
    // exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    ExitUnattached(Option<Switch>),
    // extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    ExtendedKeys(Option<Switch>),
    // focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    FocusEvents(Option<Switch>),
    // history-file path
    #[cfg(feature = "tmux_2_1")]
    HistoryFile(Option<String>),
    // message-limit number
    #[cfg(feature = "tmux_2_0")]
    MessageLimit(Option<usize>),
    // `prompt-history-limit number`
    #[cfg(feature = "tmux_3_3")]
    PromptHistoryLimit(Option<usize>),
    // set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    SetClipboard(Option<SetClipboard>),
    // terminal-features[]
    #[cfg(feature = "tmux_3_2")]
    TerminalFeatures(Option<(usize, String)>),
    // terminal-overrides[] string
    // TerminalOverrides(Option<ArrayItem<String>>),
    #[cfg(feature = "tmux_2_0")]
    TerminalOverrides(Option<(usize, String)>),
    // user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    UserKeys(Option<(usize, String)>),
    // quiet [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    Quiet(Option<Switch>),
    // detach-on-destroy [on | off]
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    DetachOnDestroy(Option<Switch>),
    // user option
    UserOption(Option<(String, String)>),
}

impl SingleLineServerOption {
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

    // parse single option with given name value and index
    pub fn parse_option(
        name: &str,
        index: Option<usize>,
        value: Option<&str>,
    ) -> Result<Self, Error> {
        match name {
            #[cfg(feature = "tmux_3_1")]
            BACKSPACE => Ok(Self::Backspace(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            BUFFER_LIMIT => Ok(Self::BufferLimit(parse_value(value))),
            #[cfg(feature = "tmux_2_4")]
            COMMAND_ALIAS => Ok(Self::CommandAlias(parse_array_value(index, value))),
            #[cfg(feature = "tmux_3_2")]
            COPY_COMMAND => Ok(Self::CopyCommand(parse_value(value))),
            #[cfg(feature = "tmux_2_1")]
            DEFAULT_TERMINAL => Ok(Self::DefaultTerminal(parse_value(value))),
            #[cfg(feature = "tmux_1_2")]
            ESCAPE_TIME => Ok(Self::EscapeTime(parse_value(value))),
            #[cfg(feature = "tmux_3_2")]
            EDITOR => Ok(Self::Editor(parse_value(value))),
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
            #[cfg(feature = "tmux_3_3")]
            PROMPT_HISTORY_LIMIT => Ok(Self::PromptHistoryLimit(parse_value(value))),
            #[cfg(feature = "tmux_1_5")]
            SET_CLIPBOARD => Ok(Self::SetClipboard(parse_value(value))),
            #[cfg(feature = "tmux_3_2")]
            TERMINAL_FEATURES => Ok(Self::TerminalFeatures(parse_array_value(index, value))),
            #[cfg(feature = "tmux_2_0")]
            TERMINAL_OVERRIDES => Ok(Self::TerminalOverrides(parse_array_value(index, value))),
            #[cfg(feature = "tmux_3_0")]
            USER_KEYS => Ok(Self::UserKeys(parse_array_value(index, value))),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            QUIET => Ok(Self::Quiet(parse_value(value))),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            DETACH_ON_DESTROY => Ok(Self::DetachOnDestroy(parse_value(value))),
            _ => {
                //let name = get_user_option_name(name)?;
                //Ok(Self::UserOption(Some((
                //name.to_string(),
                //parse_value(value),
                //))))
                Err(Error::ParseStatusKeys)
            }
        }
    }

    //fn get_user_option(name: &str, value: Option<&str>) -> Result<Self, Error> {
    //let name = get_user_option_name(name)?;
    //Ok(Self::UserOption(Some((
    //name.to_string(),
    //parse_value(value).ok_or(Error::ParseStatusKeys)?,
    //))))
    //}
}

//struct UserOptionName(&str);

//impl UserOptionName {
//pub fn is_ok(s: &str) -> bool {
//let first_char = s.chars().nth(0);
//match first_char {
//Some('@') => true,
//_ => false,
//}
//}
//}

//impl FromStr for UserOptionName {
//type Err = Error;

//fn from_str(s: &str) -> Result<Self, Self::Err> {
//let at = s.chars().nth(0);
//match at {
//Some('@') => {
//if let Some(tmp) = s.get(1..) {
//return Ok(tmp);
//}
//Err(Error::ParseStatusKeys)
//}
//_ => Err(Error::ParseStatusKeys),
//}
//}
//}

//pub fn get_user_option_name(s: &str) -> Result<&str, Error> {
//let at = s.chars().nth(0);
//match at {
//Some('@') => {
//if let Some(tmp) = s.get(1..) {
//dbg!(tmp);
//return Ok(tmp);
//}
//Err(Error::ParseStatusKeys)
//}
//_ => Err(Error::ParseStatusKeys),
//}
//}

// single line
// multiple lines
impl FromStr for SingleLineServerOption {
    type Err = Error;

    //fn from_str(s: &str) -> Result<Self, Self::Err> {
    //let lines = s.lines();
    //for line in lines {
    //let line = lines.next();
    //}

    //Err(Error::ParseStatusKeys)
    //}

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, i, value) = Self::get_option(s);

        match name {
            Some(name) => Self::parse_option(name, i, value),
            None => Err(Error::ParseStatusKeys),
        }
    }
}
