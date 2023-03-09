use super::constants::*;
use crate::{Error, Switch};
use crate::{SetOption, ShowOptions, Tmux};
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "tmux_3_0")]
use crate::RemainOnExit;

// TODO: waiting for const generics stabilization https://github.com/rust-lang/rust/issues/44580

// TODO: check types
// 5 Available pane options are:
#[derive(Default, PartialEq, Clone, Debug)]
pub struct PaneOptions<'a> {
    //allow-rename [on | off]
    #[cfg(feature = "tmux_3_0")]
    pub allow_rename: Option<Switch>,
    //alternate-screen [on | off]
    #[cfg(feature = "tmux_3_0")]
    pub alternate_screen: Option<Switch>,
    // tmux ^3.2: remain-on-exit [on | off | failed]
    //remain-on-exit [on | off]
    #[cfg(feature = "tmux_3_0")]
    pub remain_on_exit: Option<RemainOnExit>,
    //window-active-style style
    #[cfg(feature = "tmux_3_0")]
    pub window_active_style: Option<Cow<'a, str>>,
    //window-style style
    #[cfg(feature = "tmux_3_0")]
    pub window_style: Option<Cow<'a, str>>,
    //pub user_options: Option<HashMap<String, String>>
    //synchronize-panes [on | off]
    #[cfg(feature = "tmux_3_2")]
    pub synchronize_panes: Option<Switch>,
}

impl<'a> PaneOptions<'a> {
    pub fn get_all() -> Result<Self, Error> {
        let s = Tmux::with_command(ShowOptions::new().global())
            .output()?
            .to_string();
        s.parse()
    }

    // NOTE: in tmux_2_6 not exists pane
    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let selected_option = PANE_OPTIONS
            .iter()
            .filter(|t| bitflags == t.3)
            .map(|t| format!("{}", t.0))
            .collect::<Vec<String>>()
            .join(" ");
        let s = Tmux::with_command(ShowOptions::new().pane().option(&selected_option))
            .output()?
            .to_string();
        s.parse()
    }

    // allows selective set by bitmask
    // NOTE: in tmux_2_6 not exists pane
    pub fn set(&self, bitflags: usize) -> Result<(), Error> {
        for selected_option in PANE_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                Tmux::with_command(
                    SetOption::new()
                        .pane()
                        .option(selected_option.0)
                        .value(&selected_value),
                )
                .output()?;
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for PaneOptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // pane option
        for var in PANE_OPTIONS.iter() {
            // if is set some - extract
            if let Some(ref v) = var.2(self) {
                write!(f, "{} {}\n", var.0, v)?;
            }
        }
        Ok(())
    }
}

impl<'a> PaneOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn allow_rename(&mut self, allow_rename: Switch) -> &mut Self {
        self.allow_rename = Some(allow_rename);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn alternate_screen(&mut self, alternate_screen: Switch) -> &mut Self {
        self.alternate_screen = Some(alternate_screen);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn remain_on_exit(&mut self, remain_on_exit: RemainOnExit) -> &mut Self {
        self.remain_on_exit = Some(remain_on_exit);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn window_active_style(&mut self, window_active_style: &'a str) -> &mut Self {
        self.window_active_style = Some(window_active_style);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn window_style(&mut self, window_style: &'a str) -> &mut Self {
        self.window_style = Some(window_style);
        self
    }

    #[cfg(feature = "tmux_3_2")]
    pub fn synchronize_panes(&mut self, synchronize_panes: Switch) -> &mut Self {
        self.synchronize_panes = Some(synchronize_panes);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> PaneOptions {
        PaneOptions {
            #[cfg(feature = "tmux_3_0")]
            allow_rename: self.allow_rename.clone(),
            #[cfg(feature = "tmux_3_0")]
            alternate_screen: self.alternate_screen.clone(),
            #[cfg(feature = "tmux_3_0")]
            remain_on_exit: self.remain_on_exit.clone(),
            #[cfg(feature = "tmux_3_0")]
            window_active_style: self.window_active_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_3_0")]
            window_style: self.window_style.map(|s| s.to_string()),
            #[cfg(feature = "tmux_3_2")]
            synchronize_panes: self.synchronize_panes.clone(),
        }
    }
}

//const SEPARATOR: &str = " ";

fn array_insert<'a>(v: &mut Option<Vec<Cow<'a, str>>>, i: Option<usize>, value: Option<String>) {
    if let Some(i) = i {
        match value {
            Some(data) => v.get_or_insert(Vec::new()).insert(i, data.into()),
            None => *v = None,
        }
    }
}

fn cow_parse<'a>(value: Option<&str>) -> Option<Cow<'a, str>> {
    value.and_then(|s| Some(Cow::Owned(s.into())))
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
// split string in 3 parts, name, index (if option is an array) and value
// TODO: rename
pub fn get_parts(s: &str) -> Option<(&str, Option<usize>, Option<&str>)> {
    let v: Vec<&str> = s.trim().splitn(2, SEPARATOR).collect();
    let value = v.get(1).copied();
    match v.get(0) {
        Some(name) => {
            let v: Vec<&str> = name.split(|c| c == '[').collect();
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

impl<'a> FromStr for ServerOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pane_options = PaneOptions::new();

        for line in s.lines() {
            if let Some((name, i, value)) = get_parts(line) {
                match name {
                    #[cfg(feature = "tmux_3_0")]
                    ALLOW_RENAME => pane_options.allow_rename = value.map(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_3_0")]
                    ALTERNATE_SCREEN => {
                        pane_options.alternate_screen = value.map(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_0")]
                    REMAIN_ON_EXIT => pane_options.remain_on_exit = value.map(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_3_0")]
                    WINDOW_ACTIVE_STYLE => {
                        pane_options.window_active_style = value.map(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_0")]
                    WINDOW_STYLE => pane_options.window_style = value.map(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_3_2")]
                    SYNCHRONIZE_PANES => {
                        pane_options.synchronize_panes = value.map(|s| s.parse().ok())
                    }
                    _ => {
                        // if user option (@user_option value)
                        if let Some(name) = name.strip_prefix('@') {
                            pane_options
                                .user_options
                                .insert(name.to_string(), cow_parse(value));
                        }
                    }
                }
            }
        }

        Ok(pane_options)
    }
}
