use crate::options::*;
use crate::{SetOption, TmuxCommand, TmuxCommands};
use std::borrow::Cow;
use std::fmt;

// TODO: all options exist in get/set?

pub struct SetWindowOption;

pub struct SetGlobalWindowOption(SetWindowOption);

impl std::ops::Deref for SetGlobalWindowOption {
    type Target = SetWindowOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Setter {
    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(name: T, value: S) -> TmuxCommand<'a> {
        SetOption::new().option(name).value(value).build()
    }
}

impl Setter for SetWindowOption {
    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(name: T, value: S) -> TmuxCommand<'a> {
        SetOption::new().option(name).value(value).build()
    }
}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
impl SetWindowOption {
    pub fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        match value {
            Some(data) => Self::set_ext(name, Some(data)),
            None => Self::unset(name),
        }
    }

    // unset if value = None
    pub fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().option(name).unset().build()
    }

    // unset if value = None
    pub fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        //(self.setter)(name.into(), value.map(|s| s.into()))
        let cmd = match value {
            Some(data) => SetOption::new().option(name).value(data),
            None => SetOption::new().option(name),
        };
        cmd.build()
    }

    pub fn set_array<'a, S: fmt::Display>(name: S, value: Option<Vec<String>>) -> TmuxCommands<'a> {
        let mut cmds = TmuxCommands::new();
        if let Some(data) = value {
            for (i, item) in data.iter().enumerate() {
                cmds.push(Self::set(format!("{}[{}]", name, i), Some(item.to_owned())));
            }
        }
        cmds
    }

    //pub fn get<T: Into<Cow<'a, str>>>(&self, name: T) -> TmuxCommand<'a> {
    //(self.getter)(name.into())
    //}

    //pub fn gets<'a>(names: ServerOptionB) -> TmuxCommands<'a> {
    //let mut cmds = TmuxCommands::new();
    //for name in names.0 {
    //cmds.push(Self::get(name));
    //}
    //cmds
    //}

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn activity_action<'a>(activity: Option<Activity>) -> TmuxCommand<'a> {
        Self::set(ACTIVITY_ACTION, activity.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// aggressive-resize [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn aggressive_resize<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(AGGRESIVE_RESIZE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub fn allow_rename<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(ALLOW_RENAME, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub fn alternate_screen<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(ALTERNATE_SCREEN, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    pub fn automatic_rename<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(AUTOMATIC_RENAME, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn automatic_rename_format<'a>(switch: Option<String>) -> TmuxCommand<'a> {
        Self::set(AUTOMATIC_RENAME_FORMAT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_interval<'a>(interval: Option<usize>) -> TmuxCommand<'a> {
        Self::set(C0_CHANGE_INTERVAL, interval.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_trigger<'a>(trigger: Option<usize>) -> TmuxCommand<'a> {
        Self::set(C0_CHANGE_TRIGGER, trigger.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_colour<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(CLOCK_MODE_COLOUR, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_style<'a>(clock_mode_style: Option<ClockModeStyle>) -> TmuxCommand<'a> {
        Self::set(CLOCK_MODE_STYLE, clock_mode_style.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub fn force_height<'a>(height: Option<usize>) -> TmuxCommand<'a> {
        Self::set(FORCE_HEIGHT, height.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub fn force_width<'a>(width: Option<usize>) -> TmuxCommand<'a> {
        Self::set(FORCE_WIDTH, width.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub fn layout_history_limit<'a>(limit: Option<usize>) -> TmuxCommand<'a> {
        Self::set(LAYOUT_HISTORY_LIMIT, limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn main_pane_height<'a>(height: Option<usize>) -> TmuxCommand<'a> {
        Self::set(MAIN_PANE_HEIGHT, height.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn main_pane_width<'a>(width: Option<usize>) -> TmuxCommand<'a> {
        Self::set(MAIN_PANE_WIDTH, width.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(MODE_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(MODE_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(MODE_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn mode_keys<'a>(mode_keys: Option<StatusKeys>) -> TmuxCommand<'a> {
        Self::set(MODE_KEYS, mode_keys.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// mode-mouse [on | off]
    /// ```
    ///
    /// tmux 1.6:
    /// ```text
    /// mode-mouse [on | off | copy-mode]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn mode_mouse<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MODE_MOUSE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn mode_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(MODE_STYLE, style.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn monitor_activity<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MONITOR_ACTIVITY, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn monitor_content<'a>(match_string: Option<String>) -> TmuxCommand<'a> {
        Self::set(MONITOR_CONTENT, match_string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn monitor_bell<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MONITOR_BELL, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn monitor_silence<'a>(interval: Option<usize>) -> TmuxCommand<'a> {
        Self::set(MONITOR_SILENCE, interval.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_height<'a>(height: Option<usize>) -> TmuxCommand<'a> {
        Self::set(OTHER_PANE_HEIGHT, height.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_width<'a>(width: Option<usize>) -> TmuxCommand<'a> {
        Self::set(OTHER_PANE_WIDTH, width.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_active_border_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(PANE_ACTIVE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn pane_base_index<'a>(index: Option<usize>) -> TmuxCommand<'a> {
        Self::set(PANE_BASE_INDEX, index.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_format<'a>(format: Option<String>) -> TmuxCommand<'a> {
        Self::set(PANE_BORDER_FORMAT, format)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_status<'a>(pane_border_status: Option<PaneBorderStatus>) -> TmuxCommand<'a> {
        Self::set(
            PANE_BORDER_STATUS,
            pane_border_status.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub fn pane_border_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(PANE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub fn remain_on_exit<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(REMAIN_ON_EXIT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    pub fn synchronize_panes<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(SYNCHRONIZE_PANES, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn utf8<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(UTF8, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_active_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_ACTIVE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_BELL_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_BELL_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_BELL_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CONTENT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CONTENT_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CONTENT_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ACTIVITY_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-activity-bg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_bg<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ACTIVITY_BG, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_fg<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ACTIVITY_FG, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn window_status_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn window_status_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn window_status_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn window_status_current_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CURRENT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn window_status_current_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CURRENT_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn window_status_current_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CURRENT_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub fn window_status_alert_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ALERT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub fn window_status_alert_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ALEERT_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub fn window_status_alert_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ALERT_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_activity_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_ACTIVITY_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_bell_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_BELL_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn window_status_content_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CONTENT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_current_format<'a>(string: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CURRENT_FORMAT, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_attr<'a>(attributes: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_LAST_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_bg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_LAST_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_fg<'a>(colour: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_LAST_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_current_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_CURRENT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_format<'a>(string: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_FORMAT, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_last_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_LAST_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn window_status_separator<'a>(string: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_SEPARATOR, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STATUS_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub fn window_size<'a>(window_size: Option<WindowSize>) -> TmuxCommand<'a> {
        Self::set(WINDOW_SIZE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub fn word_separators<'a>(string: Option<String>) -> TmuxCommand<'a> {
        Self::set(WORD_SEPARATORS, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_style<'a>(style: Option<String>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn wrap_search<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(WRAP_SEARCH, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn xterm_keys<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(XTERM_KEYS, switch.map(|s| s.to_string()))
    }

    // XXX: user options?
    //pub user_options: Option<HashMap<String, String>>
}
