use crate::{Error, Switch};
use crate::{SetOptionBuilder, ShowOptionsBuilder, TargetPane, TmuxInterface};
use std::fmt;
use std::str::FromStr;

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
    fn(p: &mut PaneOptions, s: &str),
    fn(p: &PaneOptions) -> Option<String>,
    usize,
); PANE_OPTIONS_NUM] = [
    (
        "allow-rename",
        |p, s| p.allow_rename = s.parse().ok(),
        |p| p.allow_rename.as_ref().map(|v| v.to_string()),
        ALLOW_RENAME,
    ),
    (
        "alternate-screen",
        |p, s| p.alternate_screen = s.parse().ok(),
        |p| p.alternate_screen.as_ref().map(|v| v.to_string()),
        ALTERNATE_SCREEN,
    ),
    (
        "remain-on-exit",
        |p, s| p.remain_on_exit = s.parse().ok(),
        |p| p.remain_on_exit.as_ref().map(|v| v.to_string()),
        REMAIN_ON_EXIT,
    ),
    (
        "window-active-style",
        |p, s| p.window_active_style = Some(s.to_string()),
        |p| {
            p.window_active_style
                .as_ref()
                .map(|v| format!("\"{}\"", v.to_string()))
        },
        WINDOW_ACTIVE_STYLE,
    ),
    (
        "window-style",
        |p, s| p.window_style = Some(s.to_string()),
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
    pub allow_rename: Option<Switch>,
    //alternate-screen [on | off]
    pub alternate_screen: Option<Switch>,
    //remain-on-exit [on | off]
    pub remain_on_exit: Option<Switch>,
    //window-active-style style
    pub window_active_style: Option<String>,
    //window-style style
    pub window_style: Option<String>,
}

impl PaneOptions {
    pub fn get_all() -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let show_options = ShowOptionsBuilder::<TargetPane>::new().global().build();
        let s = tmux.show_options(Some(&show_options))?;
        s.parse()
    }

    // NOTE: in tmux_2_6 not exists pane
    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    pub fn get(bitflags: usize) -> Result<Self, Error> {
        let mut tmux = TmuxInterface::new();
        let selected_option = PANE_OPTIONS
            .iter()
            .filter(|t| bitflags == t.3)
            .map(|t| format!("{}", t.0))
            .collect::<Vec<String>>()
            .join(" ");
        let show_options = ShowOptionsBuilder::<TargetPane>::new()
            .pane()
            .option(&selected_option)
            .build();
        let s = tmux.show_options(Some(&show_options))?;
        s.parse()
    }

    // allows selective set by bitmask
    // NOTE: in tmux_2_6 not exists pane
    pub fn set(&self, bitflags: usize) -> Result<(), Error> {
        let mut tmux = TmuxInterface::new();
        for selected_option in PANE_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
            if let Some(selected_value) = selected_option.2(&self) {
                let set_option = SetOptionBuilder::<TargetPane>::new().pane().build();
                tmux.set_option(Some(&set_option), selected_option.0, &selected_value)?;
            }
        }
        Ok(())
    }
}

impl FromStr for PaneOptions {
    type Err = Error;

    fn from_str(options: &str) -> Result<Self, Self::Err> {
        let mut pane_options: PaneOptions = Default::default();
        let mut v: Vec<&str>;
        for option in options.lines() {
            v = option.trim().split(' ').collect();
            for pane_var in PANE_OPTIONS.iter() {
                if pane_var.0 == v[0] {
                    pane_var.1(&mut pane_options, v[1])
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
        write!(f, "{}", "")
    }
}

// XXX: mb &Switch
#[derive(Default, Debug)]
pub struct PaneOptionsBuilder<'a> {
    pub allow_rename: Option<Switch>,
    pub alternate_screen: Option<Switch>,
    pub remain_on_exit: Option<Switch>,
    pub window_active_style: Option<&'a str>,
    pub window_style: Option<&'a str>,
}

impl<'a> PaneOptionsBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn allow_rename(&mut self, allow_rename: Switch) -> &mut Self {
        self.allow_rename = Some(allow_rename);
        self
    }

    pub fn alternate_screen(&mut self, alternate_screen: Switch) -> &mut Self {
        self.alternate_screen = Some(alternate_screen);
        self
    }

    pub fn remain_on_exit(&mut self, remain_on_exit: Switch) -> &mut Self {
        self.remain_on_exit = Some(remain_on_exit);
        self
    }

    pub fn window_active_style(&mut self, window_active_style: &'a str) -> &mut Self {
        self.window_active_style = Some(window_active_style);
        self
    }

    pub fn window_style(&mut self, window_style: &'a str) -> &mut Self {
        self.window_style = Some(window_style);
        self
    }

    // XXX: clone rly needed?
    pub fn build(&self) -> PaneOptions {
        PaneOptions {
            allow_rename: self.allow_rename.clone(),
            alternate_screen: self.alternate_screen.clone(),
            remain_on_exit: self.remain_on_exit.clone(),
            window_active_style: self.window_active_style.map(|s| s.to_string()),
            window_style: self.window_style.map(|s| s.to_string()),
        }
    }
}
