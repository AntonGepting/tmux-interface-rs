pub mod constants;
pub mod status_keys;
pub mod switch;
pub mod terminal_features;
pub mod user_option;

pub use constants::*;
pub use status_keys::StatusKeys;
pub use switch::Switch;
pub use terminal_features::*;
pub use user_option::*;

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?

const SEPARATOR: &str = " ";

use std::borrow::Cow;
use std::fmt;

pub fn option_to_string<S: fmt::Display>(v: &mut Vec<String>, name: &str, value: &Option<S>) {
    if let Some(data) = value {
        v.push(format!("{} {}", name, data))
    }
}

pub fn option_array_to_string<S: fmt::Display>(
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

pub fn array_insert<'a>(
    v: &mut Option<Vec<Cow<'a, str>>>,
    i: Option<usize>,
    value: Option<String>,
) {
    if let Some(i) = i {
        match value {
            Some(data) => v.get_or_insert(Vec::new()).insert(i, data.into()),
            None => *v = None,
        }
    }
}

pub fn cow_parse<'a>(value: Option<&str>) -> Option<Cow<'a, str>> {
    value.and_then(|s| Some(Cow::Owned(s.into())))
}

// split string in 3 parts, name, index (if option is an array) and value
// TODO: rename
pub fn get_parts(s: &str) -> Option<(&str, Option<usize>, Option<&str>)> {
    let v: Vec<&str> = s.trim().splitn(2, SEPARATOR).collect();
    let value = v.get(1).copied();
    match v.get(0) {
        Some(name) => {
            let v: Vec<&str> = name.split(|c| c == '[' || c == ']').collect();
            match v.get(0) {
                Some(name) => {
                    let index = v.get(1).and_then(|i| i.parse().ok());
                    Some((name, index, value))
                }
                None => None,
            }
        }
        None => None,
    }
}

#[cfg(test)]
#[path = "."]
mod common_tests {
    pub mod status_keys_tests;
    pub mod switch_tests;
}
