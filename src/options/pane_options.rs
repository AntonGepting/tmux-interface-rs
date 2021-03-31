use crate::{Error, Switch};
use crate::{SetOption, ShowOptions};
use std::fmt;
use std::str::FromStr;

// XXX: conditionals?
// NOTE: total num: 5 (usize)
pub const ALLOW_RENAME: usize = 1 << 0;
pub const ALTERNATE_SCREEN: usize = 1 << 1;
pub const REMAIN_ON_EXIT: usize = 1 << 2;
pub const WINDOW_ACTIVE_STYLE: usize = 1 << 3;
pub const WINDOW_STYLE: usize = 1 << 4;

pub const PANE_OPTIONS_NONE: usize = 0;
////pub const PANE_OPTIONS_DEFAULT: usize = ;
pub const PANE_OPTIONS_ALL: usize =
    ALLOW_RENAME | ALTERNATE_SCREEN | REMAIN_ON_EXIT | WINDOW_ACTIVE_STYLE | WINDOW_STYLE;

pub const PANE_OPTIONS_NUM: usize = 5;

// TODO: waiting for const generics stabilization https://github.com/rust-lang/rust/issues/44580
pub const PANE_OPTIONS: [(
    &str,
    fn(p: &mut PaneOptions, i: Option<usize>, s: &str),
    fn(p: &PaneOptions) -> Option<String>,
    usize,
); PANE_OPTIONS_NUM] = [
    #[cfg(feature = "tmux_3_0")]
    (
        "allow-rename",
        |p, _, s| p.allow_rename = s.parse().ok(),
        |p| p.allow_rename.as_ref().map(|v| v.to_string()),
        ALLOW_RENAME,
    ),
    #[cfg(feature = "tmux_3_0")]
    (
        "alternate-screen",
        |p, _, s| p.alternate_screen = s.parse().ok(),
        |p| p.alternate_screen.as_ref().map(|v| v.to_string()),
        ALTERNATE_SCREEN,
    ),
    #[cfg(feature = "tmux_3_0")]
    (
        "remain-on-exit",
        |p, _, s| p.remain_on_exit = s.parse().ok(),
        |p| p.remain_on_exit.as_ref().map(|v| v.to_string()),
        REMAIN_ON_EXIT,
    ),
    #[cfg(feature = "tmux_3_0")]
    (
        "window-active-style",
        |p, _, s| p.window_active_style = Some(s.to_string()),
        |p| {
            p.window_active_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_ACTIVE_STYLE,
    ),
    #[cfg(feature = "tmux_3_0")]
    (
        "window-style",
        |p, _, s| p.window_style = Some(s.to_string()),
        |p| {
            p.window_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_STYLE,
    ),
];

// TODO: check types
// 5 Available pane options are:
#[derive(Default, PartialEq, Clone, Debug)]
pub struct PaneOptions {
    //allow-rename [on | off]
    #[cfg(feature = "tmux_3_0")]
    pub allow_rename: Option<Switch>,
    //alternate-screen [on | off]
    #[cfg(feature = "tmux_3_0")]
    pub alternate_screen: Option<Switch>,
    //remain-on-exit [on | off]
    #[cfg(feature = "tmux_3_0")]
    pub remain_on_exit: Option<Switch>,
    //window-active-style style
    #[cfg(feature = "tmux_3_0")]
    pub window_active_style: Option<String>,
    //window-style style
    #[cfg(feature = "tmux_3_0")]
    pub window_style: Option<String>,
    //pub user_options: Option<HashMap<String, String>>
}

impl PaneOptions {
    pub fn get_all() -> Result<Self, Error> {
        let s = ShowOptions::new().global().output()?.to_string();
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
        let s = ShowOptions::new()
            .pane()
            .option(&selected_option)
            .output()?
            .to_string();
        s.parse()
    }

    // allows selective set by bitmask
    // NOTE: in tmux_2_6 not exists pane
    pub fn set(&self, bitflags: usize) -> Result<(), Error> {
        for selected_option in PANE_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                SetOption::new()
                    .pane()
                    .option(selected_option.0)
                    .value(&selected_value)
                    .output()?;
            }
        }
        Ok(())
    }
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
impl FromStr for PaneOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut pane_options: PaneOptions = Default::default();
        let mut v: Vec<&str>;
        let mut arr: Vec<&str>;
        for option in options.lines() {
            v = option.trim().splitn(2, ' ').collect();
            arr = v[0].split(|c| c == '[' || c == ']').collect();
            for pane_var in PANE_OPTIONS.iter() {
                if pane_var.0 == arr[0] {
                    pane_var.1(
                        &mut pane_options,
                        arr.get(1).and_then(|i| i.parse::<usize>().ok()),
                        v.get(1).unwrap_or(&""),
                    )
                }
            }
        }
        Ok(pane_options)
    }
}

impl fmt::Display for PaneOptions {
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

// XXX: mb &Switch
#[derive(Default, Debug)]
pub struct PaneOptionsBuilder<'a> {
    #[cfg(feature = "tmux_3_0")]
    pub allow_rename: Option<Switch>,
    #[cfg(feature = "tmux_3_0")]
    pub alternate_screen: Option<Switch>,
    #[cfg(feature = "tmux_3_0")]
    pub remain_on_exit: Option<Switch>,
    #[cfg(feature = "tmux_3_0")]
    pub window_active_style: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub window_style: Option<&'a str>,
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
    pub fn remain_on_exit(&mut self, remain_on_exit: Switch) -> &mut Self {
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
        }
    }
}
