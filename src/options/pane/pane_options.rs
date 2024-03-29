use super::constants::*;
use crate::options::common::{cow_parse, get_parts, option_to_string};
use crate::{Error, Switch};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "tmux_3_0")]
use crate::RemainOnExit;

// TODO: check types
#[derive(PartialEq, Default, Clone, Debug)]
pub struct PaneOptions<'a> {
    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub allow_rename: Option<Switch>,

    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub alternate_screen: Option<Switch>,

    /// tmux ^3.2:
    /// ```text
    /// remain-on-exit [on | off | failed]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub remain_on_exit: Option<RemainOnExit>,

    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub window_active_style: Option<Cow<'a, str>>,

    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub window_style: Option<Cow<'a, str>>,

    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub synchronize_panes: Option<Switch>,

    #[cfg(feature = "tmux_3_0")]
    pub user_options: HashMap<String, Option<Cow<'a, str>>>,
}

impl<'a> PaneOptions<'a> {
    // allows selective set by bitmask
    // NOTE: in tmux_2_6 not exists pane
    // pub fn set(&self, bitflags: usize) -> Result<(), Error> {
}

impl<'a> fmt::Display for PaneOptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();

        #[cfg(feature = "tmux_3_0")]
        option_to_string(&mut v, ALLOW_RENAME, &self.allow_rename);
        #[cfg(feature = "tmux_3_0")]
        option_to_string(&mut v, ALTERNATE_SCREEN, &self.alternate_screen);
        #[cfg(feature = "tmux_3_0")]
        option_to_string(&mut v, REMAIN_ON_EXIT, &self.remain_on_exit);
        #[cfg(feature = "tmux_3_0")]
        option_to_string(&mut v, WINDOW_ACTIVE_STYLE, &self.window_active_style);
        #[cfg(feature = "tmux_3_0")]
        option_to_string(&mut v, WINDOW_STYLE, &self.window_style);
        #[cfg(feature = "tmux_3_2")]
        option_to_string(&mut v, SYNCHRONIZE_PANES, &self.synchronize_panes);
        // #[cfg(feature = "tmux_3_0")]
        // option_to_string(&mut v, , &self.user_options);
        let s = v.join("\n");
        write!(f, "{}", s)
    }
}

impl<'a> PaneOptions<'a> {
    pub fn new() -> Self {
        let pane_options = PaneOptions::default();
        #[cfg(feature = "tmux_3_0")]
        let pane_options = pane_options.allow_rename(Some(ALLOW_RENAME_DEFAULT));
        #[cfg(feature = "tmux_3_0")]
        let pane_options = pane_options.alternate_screen(Some(ALTERNATE_SCREEN_DEFAULT));
        #[cfg(feature = "tmux_3_0")]
        let pane_options = pane_options.remain_on_exit(Some(REMAIN_ON_EXIT_DEFAULT));
        #[cfg(feature = "tmux_3_0")]
        let pane_options = pane_options.window_active_style(Some(WINDOW_ACTIVE_STYLE_DEFAULT));
        #[cfg(feature = "tmux_3_0")]
        let pane_options = pane_options.window_style(Some(WINDOW_STYLE_DEFAULT));
        #[cfg(feature = "tmux_3_2")]
        let pane_options = pane_options.synchronize_panes(Some(SYNCHRONIZE_PANES_DEFAULT));
        // #[cfg(feature = "tmux_3_0")]
        // let pane_options = pane_options.user_options();
        pane_options
    }

    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn allow_rename(mut self, allow_rename: Option<Switch>) -> Self {
        self.allow_rename = allow_rename;
        self
    }

    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn alternate_screen(mut self, alternate_screen: Option<Switch>) -> Self {
        self.alternate_screen = alternate_screen;
        self
    }

    /// tmux ^3.2:
    /// ```text
    /// remain-on-exit [on | off | failed]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn remain_on_exit(mut self, remain_on_exit: Option<RemainOnExit>) -> Self {
        self.remain_on_exit = remain_on_exit;
        self
    }

    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_active_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_active_style: Option<S>,
    ) -> Self {
        self.window_active_style = window_active_style.map(|s| s.into());
        self
    }

    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_style<S: Into<Cow<'a, str>>>(mut self, window_style: Option<S>) -> Self {
        self.window_style = window_style.map(|s| s.into());
        self
    }

    /// tmux ^3.0:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn synchronize_panes(mut self, synchronize_panes: Option<Switch>) -> Self {
        self.synchronize_panes = synchronize_panes;
        self
    }
}
impl<'a> FromStr for PaneOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pane_options = PaneOptions::default();

        for line in s.lines() {
            if let Some((name, _, value)) = get_parts(line) {
                match name {
                    #[cfg(feature = "tmux_3_0")]
                    ALLOW_RENAME => pane_options.allow_rename = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_3_0")]
                    ALTERNATE_SCREEN => {
                        pane_options.alternate_screen = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_0")]
                    REMAIN_ON_EXIT => {
                        pane_options.remain_on_exit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_3_0")]
                    WINDOW_ACTIVE_STYLE => pane_options.window_active_style = cow_parse(value),
                    #[cfg(feature = "tmux_3_0")]
                    WINDOW_STYLE => pane_options.window_style = cow_parse(value),
                    #[cfg(feature = "tmux_3_2")]
                    SYNCHRONIZE_PANES => {
                        pane_options.synchronize_panes = value.and_then(|s| s.parse().ok())
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
