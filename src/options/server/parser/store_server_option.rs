
// custom storage (custom_struct and pointers to it's fields)
pub enum TmuxServerOptionOutputStore<'a> {
    // backspace key
    #[cfg(feature = "tmux_3_1")]
    Backspace(&'a mut Option<String>),
    // buffer-limit number
    #[cfg(feature = "tmux_1_5")]
    BufferLimit(&'a mut Option<usize>),
    // command-alias[] name=value
    #[cfg(feature = "tmux_2_4")]
    CommandAlias(&'a mut Option<Vec<String>>),
    // default-terminal terminal
    #[cfg(feature = "tmux_2_1")]
    DefaultTerminal(&'a mut Option<String>),
    //copy_command
    //escape-time time
    #[cfg(feature = "tmux_1_2")]
    EscapeTime(&'a mut Option<usize>),
    //editor
    //exit-empty [on | off]
    #[cfg(feature = "tmux_2_7")]
    ExitEmpty(&'a mut Option<Switch>),
    //exit-unattached [on | off]
    #[cfg(feature = "tmux_1_4")]
    ExitUnattached(&'a mut Option<Switch>),
    //extended-keys [on | off]
    #[cfg(feature = "tmux_3_2")]
    ExtendedKeys(&'a mut Option<Switch>),
    //focus-events [on | off]
    #[cfg(feature = "tmux_1_9")]
    FocusEvents(&'a mut Option<Switch>),
    //history-file path
    #[cfg(feature = "tmux_2_1")]
    HistoryFile(&'a mut Option<String>),
    //message-limit number
    #[cfg(feature = "tmux_2_0")]
    MessageLimit(&'a mut Option<usize>),
    //set-clipboard [on | external | off]
    #[cfg(feature = "tmux_1_5")]
    SetClipboard(&'a mut Option<SetClipboard>),
    // terminal-features[]
    //terminal-overrides[] string
    #[cfg(feature = "tmux_2_0")]
    //TerminalOverrides(Option<ArrayItem<String>>),
    TerminalOverrides(&'a mut Option<Vec<String>>),
    //user-keys[] key
    #[cfg(feature = "tmux_3_0")]
    UserKeys(&'a mut Option<Vec<String>>),
    // quiet ?
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    Quiet(&'a mut Option<Switch>),
    // detach-on-destroy ?
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    DetachOnDestroy(&'a mut Option<Switch>),
    // user option
    UserOption(&'a mut Option<String>),
}

//short, long

impl<'a> TmuxServerOptionOutputStore<'a> {
    pub fn get_name_value(s: &str) -> (Option<&str>, Option<&str>) {
        let v: Vec<&str> = s.trim().splitn(2, SEPARATOR).collect();
        let value = v.get(1).copied();
        let mut name = v.get(0).copied();

        if let Some(array_name) = name {
            let v: Vec<&str> = array_name.splitn(2, '[').collect();
            name = v.get(0).copied();
        }

        (name, value)
    }

    pub fn from_str_ext(name: &str, s: &str, option: &mut TmuxServerOptionOutputStore<'a>) {
        match (name, option) {
            #[cfg(feature = "tmux_3_1")]
            (BACKSPACE, Self::Backspace(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_5")]
            (BUFFER_LIMIT, Self::BufferLimit(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_4")]
            (COMMAND_ALIAS, Self::CommandAlias(value)) => {
                value.get_or_insert(Vec::new()).push(s.to_string())
            }
            #[cfg(feature = "tmux_2_1")]
            (DEFAULT_TERMINAL, Self::DefaultTerminal(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_2")]
            (ESCAPE_TIME, Self::EscapeTime(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_7")]
            (EXIT_EMPTY, Self::ExitEmpty(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_4")]
            (EXIT_UNATTACHED, Self::ExitUnattached(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_3_2")]
            (EXTENDED_KEY, Self::ExtendedKeys(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_9")]
            (FOCUS_EVENTS, Self::FocusEvents(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_1")]
            (HISTORY_FILE, Self::HistoryFile(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_0")]
            (MESSAGE_LIMIT, Self::MessageLimit(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_5")]
            (SET_CLIPBOARD, Self::SetClipboard(value)) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_0")]
            (TERMINAL_OVERRIDES, Self::TerminalOverrides(value)) => {
                value.get_or_insert(Vec::new()).push(s.to_string())
            }
            #[cfg(feature = "tmux_3_0")]
            (USER_KEY, Self::UserKeys(value)) => {
                value.get_or_insert(Vec::new()).push(s.to_string())
            }
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            (QUIET, Self::Quiet(value)) => **value = s.parse().ok(),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            (DETACH_ON_DESTROY, Self::DetachOnDestroy(value)) => **value = s.parse().ok(),
            _ => (),
        }
    }

    // only string and option awaiting given
    pub fn from_str_ext_short(s: &str, option: &mut TmuxServerOptionOutputStore<'a>) {
        match option {
            #[cfg(feature = "tmux_3_1")]
            Self::Backspace(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_5")]
            Self::BufferLimit(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_4")]
            Self::CommandAlias(value) => value.get_or_insert(Vec::new()).push(s.to_string()),
            #[cfg(feature = "tmux_2_1")]
            Self::DefaultTerminal(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_2")]
            Self::EscapeTime(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_7")]
            Self::ExitEmpty(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_4")]
            Self::ExitUnattached(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_3_2")]
            Self::ExtendedKeys(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_9")]
            Self::FocusEvents(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_1")]
            Self::HistoryFile(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_0")]
            Self::MessageLimit(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_1_5")]
            Self::SetClipboard(value) => **value = s.parse().ok(),
            #[cfg(feature = "tmux_2_0")]
            Self::TerminalOverrides(value) => value.get_or_insert(Vec::new()).push(s.to_string()),
            #[cfg(feature = "tmux_3_0")]
            Self::UserKeys(value) => value.get_or_insert(Vec::new()).push(s.to_string()),
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
            Self::Quiet(value) => **value = s.parse().ok(),
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
            Self::DetachOnDestroy(value) => **value = s.parse().ok(),
            _ => (),
        }
    }
}

#[test]
fn server_option_enum_ptr() {
    //let output_short = "split-pane=split-window";
    let output_short = "500";
    let mut value = None;
    TmuxServerOptionOutputStore::from_str_ext(
        "escape-time",
        output_short,
        //&mut TmuxServerOptionOutputStore::CommandAlias(&mut value),
        &mut TmuxServerOptionOutputStore::EscapeTime(&mut value),
    );
    dbg!(&value);

    //for line in output.lines() {
    //TmuxServerOptionOutputStore::from_str_ext(line)
    //}
}

pub struct TmuxServerOptionsOutputStore<'a>(Vec<TmuxServerOptionOutputStore<'a>>);

impl<'a> TmuxServerOptionsOutputStore<'a> {
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

    // parse single option with given name value and index
    //pub fn parse_option(
    //name: &str,
    //index: Option<usize>,
    //value: Option<&str>,
    //) -> Result<Self, Error> {
    //match name {
    //#[cfg(feature = "tmux_3_1")]
    //BACKSPACE => Ok(Self::Backspace(parse_value(value))),
    //#[cfg(feature = "tmux_1_5")]
    //BUFFER_LIMIT => Ok(Self::BufferLimit(parse_value(value))),
    //#[cfg(feature = "tmux_2_4")]
    //COMMAND_ALIAS => Ok(Self::CommandAlias(parse_array_value(index, value))),
    //#[cfg(feature = "tmux_2_1")]
    //DEFAULT_TERMINAL => Ok(Self::DefaultTerminal(parse_value(value))),
    //#[cfg(feature = "tmux_1_2")]
    //ESCAPE_TIME => Ok(Self::EscapeTime(parse_value(value))),
    //#[cfg(feature = "tmux_2_7")]
    //EXIT_EMPTY => Ok(Self::ExitEmpty(parse_value(value))),
    //#[cfg(feature = "tmux_1_4")]
    //EXIT_UNATTACHED => Ok(Self::ExitUnattached(parse_value(value))),
    //#[cfg(feature = "tmux_3_2")]
    //EXTENDED_KEYS => Ok(Self::ExtendedKeys(parse_value(value))),
    //#[cfg(feature = "tmux_1_9")]
    //FOCUS_EVENTS => Ok(Self::FocusEvents(parse_value(value))),
    //#[cfg(feature = "tmux_2_1")]
    //HISTORY_FILE => Ok(Self::HistoryFile(parse_value(value))),
    //#[cfg(feature = "tmux_2_0")]
    //MESSAGE_LIMIT => Ok(Self::MessageLimit(parse_value(value))),
    //#[cfg(feature = "tmux_1_5")]
    //SET_CLIPBOARD => Ok(Self::SetClipboard(parse_value(value))),
    //#[cfg(feature = "tmux_2_0")]
    //TERMINAL_OVERRIDES => Ok(Self::TerminalOverrides(parse_array_value(index, value))),
    //#[cfg(feature = "tmux_3_0")]
    //USER_KEYS => Ok(Self::UserKeys(parse_array_value(index, value))),
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //QUIET => Ok(Self::Quiet(parse_value(value))),
    //#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    //DETACH_ON_DESTROY => Ok(Self::DetachOnDestroy(parse_value(value))),
    //_ => Err(Error::ParseStatusKeys),
    //}
    //}
}


