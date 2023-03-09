use crate::options::*;
use crate::Error;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// hashmap for options -> String, String
//  - string only
//      buffer_limit(&self) -> Option<String>
//      buffer_limit_mut(&self) -> Option<&mut String> // usize could be better
//
// struct for options -> <name>, <T>
//  + types supported

// XXX: typealias name value
#[derive(Debug, Default)]
pub struct TmuxOptionsMap(HashMap<String, Option<String>>);

impl fmt::Display for TmuxOptionsMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        for (key, value) in &self.0 {
            match value {
                Some(value) => v.push(format!("{} {}", &key, &value)),
                None => v.push(format!("{}", &key)),
            }
        }
        let output = v.join("\n");
        write!(f, "{}", output)
    }
}

impl TmuxOptionsMap {
    // XXX: mb result with an error if not in list
    pub fn get(&self, name: &str) -> Option<&Option<String>> {
        self.0.get(name)
    }

    // XXX: result
    pub fn set<T: std::fmt::Display>(&mut self, name: &str, value: Option<T>) {
        self.0
            .get_mut(name)
            .and_then(|v| Some(*v = value.and_then(|s| Some(s.to_string()))));
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Option<String>> {
        self.0.get_mut(name)
    }

    //pub fn get<F: FromStr>(&self, name: &str) -> Result<F, Error> {
    //self.0
    //.get(name)
    //.and_then(|value| value.as_ref().and_then(|data| data.parse().ok()))
    //.ok_or(Error::ParseVersion)
    //}

    pub fn get_array_item(&self, name: &str, index: usize) -> Option<&Option<String>> {
        self.get(&format!("{}[{}]", name, index))
    }

    pub fn insert(&mut self, name: &str, value: Option<String>) -> Option<Option<String>> {
        self.0.insert(name.to_string(), value)
    }

    // NOTE:
    // * 1. Variant - empty array:
    //      ```text
    //      option_name
    //      ```
    //
    // * 2. Variant - filled array:
    //      ```text
    //      option_name[0] value
    //      option_name[1] value
    //      ...
    //      ```
    pub fn get_array(&self, name: &str) -> Option<Vec<&Option<String>>> {
        let mut output = None;

        // check only name without array index
        if self.get(name).is_none() {
            let mut v = Vec::new();
            let mut i = 0;
            // while item names with an index exist, save them
            while let Some(item) = self.get_array_item(name, i) {
                v.push(item);
                i += 1;
            }
            if i > 0 {
                output = Some(v);
            }
        }

        output
    }

    pub fn insert_array_item(
        &mut self,
        name: &str,
        index: usize,
        value: Option<String>,
    ) -> Option<Option<String>> {
        self.insert(&format!("{}[{}]", name, index), value)
    }
}

impl FromStr for TmuxOptionsMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();

        for line in s.lines() {
            let v: Vec<&str> = line.splitn(2, " ").collect();
            if let Some(name) = v.get(0) {
                let value = v.get(1);
                // XXX: .to_lowercase()
                map.insert(name.to_string(), value.map(|s| s.to_string()));
            }
        }

        Ok(Self(map))
    }
}

#[test]
fn tmux_options_map() {
    // FIXME: conditionals
    //#[cfg(feature = "tmux_2_6")]
    //use crate::SetClipboard;
    //use crate::{ServerOptions, Switch};

    //let mut options = ServerOptions::new();
    //let options = options.buffer_limit(50);
    //#[cfg(feature = "tmux_2_1")]
    //let options = options.default_terminal("\"screen-256color\"");
    //#[cfg(feature = "tmux_2_7")]
    //let options = options.exit_empty(Switch::On);
    //#[cfg(feature = "tmux_2_4")]
    //let options = options.command_alias(vec![
    //"\"split-pane=split-window\"",
    //"\"splitp=split-window\"",
    //]);
    //let server_options_default = options;

    //// test int, string, enum, vec
    //let server_options_str = r#"buffer-limit 50
    //default-terminal "screen-256color"
    //exit-empty on
    //command-alias[0] "split-pane=split-window"
    //command-alias[1] "splitp=split-window"
    //"#;
    //let server_options = server_options_str.parse::<ServerOptions>().unwrap();
    //assert_eq!(server_options_default, server_options);

    //let mut options = ServerOptions::new();
    //let options = options.buffer_limit(50);
    //#[cfg(feature = "tmux_2_4")]
    //let options = options.command_alias(vec![
    //"\"split-pane=split-window\"",
    //"\"splitp=split-window\"",
    //"\"server-info=show-messages -JT\"",
    //"\"info=show-messages -JT\"",
    //"\"choose-window=choose-tree -w\"",
    //"\"choose-session=choose-tree -s\"",
    //]);
    //#[cfg(feature = "tmux_2_1")]
    //let options = options.default_terminal("\"screen-256color\"");
    //let options = options.escape_time(500);
    //#[cfg(feature = "tmux_2_7")]
    //let options = options.exit_empty(Switch::On);
    //let options = options.exit_unattached(Switch::Off);
    //#[cfg(feature = "tmux_1_9")]
    //let options = options.focus_events(Switch::Off);
    //#[cfg(feature = "tmux_2_1")]
    //let options = options.history_file("\"\"");
    //#[cfg(feature = "tmux_2_0")]
    //let options = options.message_limit(100);
    //#[cfg(feature = "tmux_2_6")]
    //let options = options.set_clipboard(SetClipboard::External);
    //#[cfg(feature = "tmux_2_0")]
    //let options = options.terminal_overrides(vec!["\"xterm*:XT:Ms=\\\\E]52;%p1%s;%p2%s\\\\007:Cs=\\\\E]12;%p1%s\\\\007:Cr=\\\\E]112\\\\007:Ss=\\\\E[%p1%d q:Se=\\\\E[2 q\"",
    //"\"screen*:XT\""]);
    ////#[cfg(feature = "tmux_3_0")]
    ////builder.user_keys = None;

    //let server_options_default = options;

    let server_options_str = r#"buffer-limit 50
command-alias[0] "split-pane=split-window"
command-alias[1] "splitp=split-window"
command-alias[2] "server-info=show-messages -JT"
command-alias[3] "info=show-messages -JT"
command-alias[4] "choose-window=choose-tree -w"
command-alias[5] "choose-session=choose-tree -s"
default-terminal "screen-256color"
escape-time 500
exit-empty on
exit-unattached off
focus-events off
history-file ""
message-limit 100
set-clipboard external
terminal-overrides[0] "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q"
terminal-overrides[1] "screen*:XT"
user-keys
"#;
    let map = server_options_str.parse::<TmuxOptionsMap>().unwrap();
    dbg!(&map);

    dbg!(&map.to_string());

    //let value: Option<String> = map.get("default-terminal").unwrap();
    //dbg!(&value);
    //let value: Option<String> = map.get("escape-time").unwrap();
    //dbg!(&value);
    //let value: Switch = map.get("focus-events").unwrap();
    //dbg!(&value);
}

#[test]
fn tmux_options_map_get() {
    use crate::{GetServerOptions, GetServerOptionsTrait, Tmux};

    //let cmd = GetServerOption::escape_time();
    //let cmd = GetServerOptions::new().build();
    let cmd = GetServerOptions::new()
        .escape_time()
        .exit_unattached()
        .command_alias()
        .build();
    let tmux = Tmux::with_commands(cmd);
    dbg!(&tmux);
    let output = tmux.output().unwrap().to_string();
    dbg!(&output);

    let mut map = output.parse::<TmuxOptionsMap>().unwrap();
    dbg!(&map);

    let value = map.get("escape-time");
    dbg!(&value);

    map.set("escape-time", Some(1));
    dbg!(&map);
}

// Build command - getter setter builder with custom invoker
// Convert into Map - Result as TmuxOptionsMap as ServerOptionsMap
//
// map.escape_time() -> Option<Value>
// map.escape_time_mut() -> Option<&mut Value> (not exists, problem create, but if not set?)
//
// server_options_struct.get/set

//pub struct TmuxServerOptionsMap;

//impl TmuxServerOptionsMap {
//pub fn new(invoker: &'a dyn Fn(&TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
//ServerOption { invoker }
//}

//pub fn load() {}

//pub fn save() {}
//}
