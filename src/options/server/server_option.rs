//use std::str::FromStr;
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
// ServerOption::backspace() -> Result<Self::Backspace(String), Error>
//  GetServerOption::backspace()
//  ParseServerOption::from_str()
//
// ServerOption::set().backspace() -> Result<(), Error> {
//  SetServerOption::backspace()
//  Output
// }

//fn array_name(name: &str, index: usize) -> String {
//format!("{}[{}]", name, index)
//}

//impl<'a> From<ServerOptionName> for Cow<'a, str> {
//fn from(name: ServerOptionName) -> Self {
//Cow::from(&name)
//}
//}

//impl<'a> From<&ServerOptionName> for Cow<'a, str> {
//fn from(name: &ServerOptionName) -> Self {
//match name {
//#[cfg(feature = "tmux_3_1")]
//ServerOptionName::Backspace => BACKSPACE.into(),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::BufferLimit => BUFFER_LIMIT.into(),
//#[cfg(feature = "tmux_2_4")]
////ServerOptionName::CommandAlias(i) => array_name(COMMAND_ALIAS, *i).into(),
//ServerOptionName::CommandAlias => COMMAND_ALIAS.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::CopyCommand => COPY_COMMAND.into(),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::DefaultTerminal => DEFAULT_TERMINAL.into(),
//#[cfg(feature = "tmux_1_2")]
//ServerOptionName::EscapeTime => ESCAPE_TIME.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::Editor => EDITOR.into(),
//#[cfg(feature = "tmux_2_7")]
//ServerOptionName::ExitEmpty => EXIT_EMPTY.into(),
//#[cfg(feature = "tmux_1_4")]
//ServerOptionName::ExitUnattached => EXIT_UNATTACHED.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::ExtendedKeys => EXTENDED_KEYS.into(),
//#[cfg(feature = "tmux_1_9")]
//ServerOptionName::FocusEvents => FOCUS_EVENTS.into(),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::HistoryFile => HISTORY_FILE.into(),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::MessageLimit => MESSAGE_LIMIT.into(),
//#[cfg(feature = "tmux_3_3")]
//ServerOptionName::PromptHistoryLimit => PROMPT_HISTORY_LIMIT.into(),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::SetClipboard => SET_CLIPBOARD.into(),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::TerminalFeatures(i) => array_name(TERMINAL_FEATURES, *i).into(),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::TerminalOverrides(i) => array_name(TERMINAL_OVERRIDES, *i).into(),
//#[cfg(feature = "tmux_3_0")]
//ServerOptionName::UserKeys(i) => array_name(USER_KEYS, *i).into(),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//ServerOptionName::Quiet => QUIET.into(),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//ServerOptionName::DetachOnDestroy => DETACH_ON_DESTROY.into(),
//ServerOptionName::UserOption(user_option_name) => {
//format!("{}{}", USER_OPTION_MARKER, user_option_name).into()
//}
//}
//}
//}

use crate::{Error, GetServerOption, GetServerOptionTrait, TmuxCommand, TmuxOutput};

pub struct ServerOption<'a> {
    pub invoker: &'a dyn Fn(&TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> ServerOption<'a> {
    pub fn new(invoker: &'a dyn Fn(&TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        ServerOption { invoker }
    }

    //pub fn get(&self, option: usize) {
    ////"1".parse().ok();
    ////match option {}
    //}

    /// `backspace key`
    #[cfg(feature = "tmux_3_1")]
    pub fn backspace(&self) -> Option<Cow<'a, str>> {
        let cmd = GetServerOption::backspace();
        let output = (self.invoker)(&cmd).ok()?;
        //ServerOption::from_str(&output)
    }

    /// `buffer-limit number`
    #[cfg(feature = "tmux_1_5")]
    pub fn buffer_limit(&self) -> Option<usize> {
        let cmd = GetServerOption::buffer_limit();
        let output = (self.invoker)(&cmd).ok()?;
        //ServerOption::from_str(&output)
        dbg!(&output);
        output.to_string().parse().ok()
    }
}

#[test]
fn server_option123() {
    use crate::Tmux;

    //let c = ServerOption::new(&|cmd| Tmux::with_command(cmd).output());
    //let buffer_limit = c.buffer_limit();
    //dbg!(buffer_limit);
}

//impl<'a> FromStr for ServerOption<'a> {
//type Err = Error;

//fn from_str(s: &str) -> Result<Self, Self::Err> {
//for line in s.lines() {
//let option: SingleLineServerOption = line.parse()?;

//match option {
//#[cfg(feature = "tmux_3_1")]
//SingleLineServerOption::Backspace(value) => {
//server_options.backspace = value.map(|s| s.into())
//}
//#[cfg(feature = "tmux_1_5")]
//SingleLineServerOption::BufferLimit(value) => server_options.buffer_limit = value,
//#[cfg(feature = "tmux_2_4")]
//SingleLineServerOption::CommandAlias(value) => {
//array_insert(&mut server_options.command_alias, value)
//}
//#[cfg(feature = "tmux_2_1")]
//SingleLineServerOption::DefaultTerminal(value) => {}
//}
//}
//}
//}

//impl fmt::Display for ServerOptionName {
//fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
//let s: Cow<'a, str> = self.into();
//write!(f, "{}", s)
//}
//}

//pub struct OptionsController<'a> {
//pub setter: &'a dyn Fn(&str) -> String,
//pub getter: &'a dyn Fn(&str) -> String,
//}

//impl<'a> OptionsController<'a> {
//pub fn new() -> Self {
//Self {
//setter: &Self::default_getter,
//getter: &Self::default_getter,
//}
//}

//fn default_getter(name: &str) -> String {
//Tmux::new()
//.command(SetOption::new().server().option(name))
//.output()
//.unwrap()
//.to_string()
//}

// get from output containing name value pair (request: `backspace`, response: `backspace C-?`)
//pub fn get_name_value(tmux: Option<Tmux>, name: ServerOptionName) -> Result<ServerOption, Error> {
//let show_options = ShowOptions::new().server().option(name);
//let tmux = match tmux {
//Some(tmux) => tmux.command(show_options),
//None => Tmux::new().command(show_options),
//};
//let output = tmux.output()?.to_string();
////parse(output, name)
//}

//// get from output containing only value (request: `backspace`, response: `C-?`)
//pub fn get_value(tmux: Option<Tmux>, name: ServerOptionName) -> Result<ServerOption, Error> {
//unimplemented!()
////let show_options = ShowOptions::new().server().value().option(name);
////let tmux = match tmux {
////Some(tmux) => tmux.command(show_options),
////None => Tmux::new().command(show_options),
////};
//// FIXME
////tmux.output()?.to_string().parse()
//}

//pub fn setter(&mut self, setter: &'a dyn Fn(&str) -> String) -> &mut Self {
//self.setter = setter;
//self
//}

//pub fn getter(&mut self, getter: &'a dyn Fn(&str) -> String) -> &mut Self {
//self.getter = getter;
//self
//}
//}

//impl<'a> ServerOptionController<'a> {
//pub fn new() -> Self {}
//}

//impl<T: fmt::Display> TmuxServerOption2<T> {
//pub fn buffer_limit() {}

//pub fn get(&self, name: ServerOptionName) -> Result<ServerOption, Error> {
//Tmux::new()
//.command(ShowOptions::new().server().option(name))
//.output()?
//.to_string()
//.parse()
//}

//pub fn set(&self) -> Result<(), Error> {
////Tmux::new()
////.command(SetOption::new().server().option(name).value())
////.output()?;
//Ok(())
//}
//}

//#[test]
//fn get_server_option() {
//use crate::Tmux;

//let cmd = Tmux::new()
//.command(GetServerOption::get(BUFFER_LIMIT))
//.output()
//.unwrap();
//let cmd = Tmux::new()
//.command(GetServerOption::buffer_limit())
//.command(GetServerOption::set_clipboard())
//.output()
//.unwrap();
//dbg!(&cmd);
//let cmd = TmuxServerOptionOutput::from(cmd).buffer_limit();
//dbg!(&cmd);

//let cmd = Tmux::new()
//.command(GetServerOption::command_alias())
//.output()
//.unwrap();
//let cmd = TmuxServerOptionOutput::from(cmd).command_alias();
//dbg!(&cmd);

//let cmds = SetServerOption::command_alias(Some(vec!["asdf".to_string(), "a".to_string()]));
//dbg!(&cmds);
//}

// mb common
//fn parse_value<T: FromStr>(value: Option<&str>) -> Option<T> {
//value.and_then(|data| data.parse().ok())
//}

// single line
// multiple lines
//impl FromStr for TmuxServerOptionsOutputStore {
//type Err = Error;

//fn from_str(s: &str) -> Result<Self, Self::Err> {
//let (name, i, value) = Self::get_option(s);

//match name {
//Some(name) => Self::parse_option(name, i, value),
//None => Err(Error::ParseStatusKeys),
//}
//}
//}

// TODO: multiline
//fn default_show_server_option<'a, S: Into<Cow<'a, str>>>(name: S) -> Result<String, Error> {
//let output = Tmux::new()
//.command(ShowOptions::new().server().option(name).value())
//.output()?;
//Ok(output.to_string())
//}

//fn default_set_option<'a, S: Into<Cow<'a, str>>>(name: S, value: S) -> Result<TmuxOutput, Error> {
//Tmux::new()
//.command(SetOption::new().server().option(name).value(value))
//.output()
//}

//pub struct ArrayItem<T> {
//pub index: usize,
//pub data: T,
//}

//impl<T: fmt::Display> fmt::Display for ArrayItem<T> {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
////let mut s = String::new();

////if let Some(i) = self.index {
//let mut s = format!("[{}]", self.index);
////}

////if let Some(data) = self.data {
//s = format!("{} {}", s, self.data);
////}

//write!(f, "{}", s)
//}
//}

//impl<T: fmt::Display> ArrayItem<T> {
////
//pub fn create_name_with_index<'a>(&self, name: &'a str) -> Cow<'a, str> {
////match self.index {
////Some(i) => format!("{}[{}]", name, i).into(),
////None => name.into(),
////}
//format!("{}[{}]", name, self.index).into()
//}

//pub fn to_string_with_name(&self, name: &str) -> String {
//let mut s = name.to_string();
////if let Some(i) = self.index {
//s = format!("{}[{}]", s, self.index);
////}
////if let Some(data) = self.data {
//s = format!("{} {}", s, self.data);
////}
//s
//}
//}

// mb common
// if option value is not set, print only name
//fn print_option<S: ToString>(name: &str, value: &Option<S>) -> String {
//match value {
//Some(value) => format!("{} {}", name, value.to_string()),
//None => format!("{}", name),
//}
//}

// if option value is not set, print only name
//fn print_array_option<T: fmt::Display>(name: &str, value: &Option<(usize, T)>) -> String {
//match value {
//Some((i, data)) => format!("{}[{}] {}", name, i, data),
//None => format!("{}", name),
//}
//}

//fn print_option_array_item(name: &str, array: Vec<String>, index: usize) -> String {
//let item = array.get(index);
//match item {
//Some(data) => format!("{}[{}] {}", name, index, data),
//None => format!("{}[{}]", name, index),
//}
//}
//fn get_name_value<S: ToString>(name: &str, value: Option<S>) -> {

//}

//fn parse_array_value<T: FromStr>(index: Option<usize>, value: Option<&str>) -> Option<(usize, T)> {
//match index {
//Some(i) => value
//.and_then(|data| data.parse().ok())
//.map(|data| (i, data)),
//None => None,
//}
//}

//impl fmt::Display for ServerOption {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//let s = match self {
//#[cfg(feature = "tmux_3_1")]
//Self::Backspace(value) => print_option(BACKSPACE, value),
//#[cfg(feature = "tmux_1_5")]
//Self::BufferLimit(value) => print_option(BUFFER_LIMIT, value),
//#[cfg(feature = "tmux_2_4")]
//// FIXME
//Self::CommandAlias(value) => print_array_option(COMMAND_ALIAS, value),
//#[cfg(feature = "tmux_2_1")]
//Self::DefaultTerminal(value) => print_option(DEFAULT_TERMINAL, value),
//#[cfg(feature = "tmux_1_2")]
//Self::EscapeTime(value) => print_option(ESCAPE_TIME, value),
//#[cfg(feature = "tmux_2_7")]
//Self::ExitEmpty(value) => print_option(EXIT_EMPTY, value),
//#[cfg(feature = "tmux_1_4")]
//Self::ExitUnattached(value) => print_option(EXIT_UNATTACHED, value),
//#[cfg(feature = "tmux_3_2")]
//Self::ExtendedKeys(value) => print_option(EXTENDED_KEYS, value),
//#[cfg(feature = "tmux_1_9")]
//Self::FocusEvents(value) => print_option(FOCUS_EVENTS, value),
//#[cfg(feature = "tmux_2_1")]
//Self::HistoryFile(value) => print_option(HISTORY_FILE, value),
//#[cfg(feature = "tmux_2_0")]
//Self::MessageLimit(value) => print_option(MESSAGE_LIMIT, value),
//#[cfg(feature = "tmux_1_5")]
//Self::SetClipboard(value) => print_option(SET_CLIPBOARD, value),
//#[cfg(feature = "tmux_2_0")]
//// FIXME
//Self::TerminalOverrides(value) => print_array_option(TERMINAL_OVERRIDES, value),
//#[cfg(feature = "tmux_3_0")]
//Self::UserKeys(value) => print_array_option(USER_KEYS, value),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//Self::Quiet(value) => print_option(QUIET, value),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//Self::DetachOnDestroy(value) => print_option(DETACH_ON_DESTROY, value),
//_ => String::new(),
//};
//write!(f, "{}", s)
//}
//}

//fn get_array_item_name<'a, T: fmt::Display>(
//name: &'a str,
//value: &Option<ArrayItem<T>>,
//) -> Cow<'a, str> {
//match value {
//Some(index) => format!("{}[{}]", name, index).into(),
//None => name.into(),
//}
//}

//impl ServerOption {
//pub fn get_by_name(name: &str) {

//}

//pub fn set_ext(cb: Option<&dyn Fn(&str, &str) -> String>) -> Result<Self, Error> {
//let s = match cb {
//Some(cb) => cb("", ""),
//None => Tmux::new()
//.command(SetOption::new().server().option().value())
//.output()?
//.to_string(),
//};
//s.parse()
//}

//pub fn get_ext<T: FromStr>(
//cb: Option<&dyn Fn(&str) -> String>,
//name: &str,
//) -> Result<T, Error> {
//let s = match cb {
//Some(cb) => cb(name),
//None => Tmux::new()
//.command(ShowOptions::new().server().option(name))
//.output()?
//.to_string(),
//};
//s.parse()
//}

// using both name and value
//pub fn get_full<T: FromStr>(name: &str) -> Result<T, Error> {
//let s = Tmux::new()
//.command(ShowOptions::new().server().value().option(name))
//.output()?
//.to_string();
//parse_option(s,)
//}

// using only tmux returned value (without option name)
//pub fn get_short(name: &str) -> Result<Self, Error> {
//let s = Tmux::new()
//.command(ShowOptions::new().server().value().option(name))
//.output()?
//.to_string();
//Self::parse_option(name, None, Some(&s))
//}

//pub fn set23<'a, S: fmt::Display, T: Into<Cow<'a, str>>>(
//name: T,
//value: Option<S>,
//) -> Result<TmuxOutput, Error> {
//let name = name;
//let value = match value {
//Some(value) => value.to_string(),
//None => "".to_string(),
//};
//Tmux::new()
//.command(SetOption::new().server().value(value).option(name))
//.output()
//}

//pub fn get2<T: FromStr>(name: ServerOptionName) -> Result<T, Error> {
//let output = Tmux::new()
//.command(ShowOptions::new().server().option(name.to_string()))
//.output()?;
//ServerOption::from_str(output)
//}

//pub fn set<S: fmt::Display>(
//self,
//name: ServerOptionName,
//value: Option<S>,
//cb: &dyn Fn(&str, Option<&str>) -> Result<TmuxOutput, Error>,
//) -> Result<TmuxOutput, Error> {
//match name {
//#[cfg(feature = "tmux_3_1")]
//ServerOptionName::Backspace => Self::set23(BACKSPACE, value),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::BufferLimit => Self::set23(BUFFER_LIMIT, value),
////#[cfg(feature = "tmux_2_4")]
////ServerOptionName::CommandAlias(i) => Self::set23(array_name(COMMAND_ALIAS, i), value),
//#[cfg(feature = "tmux_2_4")]
//ServerOptionName::CommandAlias => Self::set23(COMMAND_ALIAS, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::CopyCommand => Self::set23(COPY_COMMAND, value),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::DefaultTerminal => Self::set23(DEFAULT_TERMINAL, value),
//#[cfg(feature = "tmux_1_2")]
//ServerOptionName::EscapeTime => Self::set23(ESCAPE_TIME, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::Editor => Self::set23(EDITOR, value),
//#[cfg(feature = "tmux_2_7")]
//ServerOptionName::ExitEmpty => Self::set23(EXIT_EMPTY, value),
//#[cfg(feature = "tmux_1_4")]
//ServerOptionName::ExitUnattached => Self::set23(EXIT_UNATTACHED, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::ExtendedKeys => Self::set23(EXTENDED_KEYS, value),
//#[cfg(feature = "tmux_1_9")]
//ServerOptionName::FocusEvents => Self::set23(FOCUS_EVENTS, value),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::HistoryFile => Self::set23(HISTORY_FILE, value),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::MessageLimit => Self::set23(MESSAGE_LIMIT, value),
//#[cfg(feature = "tmux_3_3")]
//ServerOptionName::PromptHistoryLimit => Self::set23(PROMPT_HISTORY_LIMIT, value),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::SetClipboard => Self::set23(SET_CLIPBOARD, value),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::TerminalFeatures(i) => {
//Self::set23(array_name(TERMINAL_FEATURES, i), value)
//}
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::TerminalOverrides(i) => {
//Self::set23(array_name(TERMINAL_OVERRIDES, i), value)
//}
//#[cfg(feature = "tmux_3_0")]
//ServerOptionName::UserKeys(i) => Self::set23(array_name(USER_KEYS, i), value),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//ServerOptionName::Quiet => Self::set23(QUIET, value),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//ServerOptionName::DetachOnDestroy => Self::set23(DETACH_ON_DESTROY, value),
//ServerOptionName::UserOption(name) => Self::set23(name.to_string(), value),
//}
//}

//pub fn parse_only_value(
//name: ServerOptionName,
//index: Option<usize>,
//value: Option<&str>,
//) -> Result<Self, Error> {
//match name {
//#[cfg(feature = "tmux_3_1")]
//ServerOptionName::Backspace => Ok(Self::Backspace(parse_value(value))),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::BufferLimit => Ok(Self::BufferLimit(parse_value(value))),
//// FIXME
////#[cfg(feature = "tmux_2_4")]
////ServerOptionName::CommandAlias(_) => {
////Ok(Self::CommandAlias(parse_array_value(index, value)))
////}
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::DefaultTerminal => Ok(Self::DefaultTerminal(parse_value(value))),
//#[cfg(feature = "tmux_1_2")]
//ServerOptionName::EscapeTime => Ok(Self::EscapeTime(parse_value(value))),
//#[cfg(feature = "tmux_2_7")]
//ServerOptionName::ExitEmpty => Ok(Self::ExitEmpty(parse_value(value))),
//#[cfg(feature = "tmux_1_4")]
//ServerOptionName::ExitUnattached => Ok(Self::ExitUnattached(parse_value(value))),
//#[cfg(feature = "tmux_3_2")]
//ServerOptionName::ExtendedKeys => Ok(Self::ExtendedKeys(parse_value(value))),
//#[cfg(feature = "tmux_1_9")]
//ServerOptionName::FocusEvents => Ok(Self::FocusEvents(parse_value(value))),
//#[cfg(feature = "tmux_2_1")]
//ServerOptionName::HistoryFile => Ok(Self::HistoryFile(parse_value(value))),
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::MessageLimit => Ok(Self::MessageLimit(parse_value(value))),
//#[cfg(feature = "tmux_1_5")]
//ServerOptionName::SetClipboard => Ok(Self::SetClipboard(parse_value(value))),
//// FIXME
//#[cfg(feature = "tmux_2_0")]
//ServerOptionName::TerminalOverrides(_) => {
//Ok(Self::TerminalOverrides(parse_array_value(index, value)))
//}
//#[cfg(feature = "tmux_3_0")]
//ServerOptionName::UserKeys(_) => Ok(Self::UserKeys(parse_array_value(index, value))),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
//ServerOptionName::Quiet => Ok(Self::Quiet(parse_value(value))),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//ServerOptionName::DetachOnDestroy => Ok(Self::DetachOnDestroy(parse_value(value))),
//_ => Err(Error::ParseStatusKeys),
//}
//}

// recognize option name, stor where? wrong way, storage equal for all
//pub fn parse_option2<T: FromStr>(
//name: &str,
//index: Option<usize>,
//value: Option<&str>,
//output: &mut Option<T>,
//) {
//match name {
//#[cfg(feature = "tmux_3_1")]
//BACKSPACE => *output = parse_value(value),
//#[cfg(feature = "tmux_1_5")]
//BUFFER_LIMIT => *output = parse_value(value),
////#[cfg(feature = "tmux_2_4")]
////COMMAND_ALIAS => *output = parse_array_value(index, value),
//#[cfg(feature = "tmux_2_1")]
//DEFAULT_TERMINAL => *output = parse_value(value),
//_ => {}
//}
//}
