use crate::options::session::activity::Activity;
use crate::options::SetOptionExt;
use crate::options::*;
use crate::{SetOption, TmuxCommand, TmuxCommands};
use std::borrow::Cow;
use std::fmt;

// TODO: all options exist in get/set?

pub struct SetGlobalSessionOption;

impl SetOptionExt for SetGlobalSessionOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().global().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().global().option(name).value(data),
            None => SetOption::new().global().option(name),
        };
        cmd.build()
    }
}

impl SetSessionOption for SetGlobalSessionOption {}

pub struct SetLocalSessionOption;

impl SetOptionExt for SetLocalSessionOption {
    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = match value {
            Some(data) => SetOption::new().option(name).value(data),
            None => SetOption::new().option(name),
        };
        cmd.build()
    }
}

impl SetSessionOption for SetLocalSessionOption {}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
pub trait SetSessionOption: SetOptionExt {
    fn set_array<'a, S: fmt::Display>(name: S, value: Option<Vec<String>>) -> TmuxCommands<'a> {
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
    fn activity_action<'a>(activity: Option<Activity>) -> TmuxCommand<'a> {
        Self::set(ACTIVITY_ACTION, activity.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn assume_paste_time<'a>(milliseconds: Option<usize>) -> TmuxCommand<'a> {
        Self::set(ASSUME_PASTE_TIME, milliseconds.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn base_index<'a>(index: Option<usize>) -> TmuxCommand<'a> {
        Self::set(BASE_INDEX, index.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux:
    /// ```text
    /// bell-action [any | none | current | other]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// bell-action [any | none | other]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn bell_action<'a>(action: Option<Action>) -> TmuxCommand<'a> {
        Self::set(BELL_ACTION, action.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn bell_on_alert<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(BELL_ON_ALERT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn buffer_limit<'a>(limit: Option<usize>) -> TmuxCommand<'a> {
        Self::set(BUFFER_LIMIT, limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_command<'a, S: Into<Cow<'a, str>>>(shell_command: Option<S>) -> TmuxCommand<'a> {
        Self::set(DEFAULT_COMMAND, shell_command)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_shell<'a, S: Into<Cow<'a, str>>>(path: Option<S>) -> TmuxCommand<'a> {
        Self::set(DEFAULT_SHELL, path)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn default_path<'a, S: Into<Cow<'a, str>>>(path: Option<S>) -> TmuxCommand<'a> {
        Self::set(DEFAULT_PATH, path)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn default_terminal<'a, S: Into<Cow<'a, str>>>(terminal: Option<S>) -> TmuxCommand<'a> {
        Self::set(DEFAULT_TERMINAL, terminal)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn default_size<'a>() -> TmuxCommand<'a> {
        Self::set(DEFAULT_SIZE)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn destroy_unattached<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(DESTROY_UNATTACHED, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```
    /// detach-on-destroy [on | off | no-detached]
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn detach_on_destroy<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(DETACH_ON_DESTROY, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn display_panes_active_colour<'a, S: Into<Cow<'a, str>>>(
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(DISPLAY_PANES_ACTIVE_COLOUR, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_colour<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(DISPLAY_PANES_COLOUR, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_time<'a>(time: Option<usize>) -> TmuxCommand<'a> {
        Self::set(DISPLAY_PANES_TIME, time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_time<'a>(time: Option<usize>) -> TmuxCommand<'a> {
        Self::set(DISPLAY_TIME, time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn history_limit<'a>(lines: Option<usize>) -> TmuxCommand<'a> {
        Self::set(HISTORY_LIMIT, lines.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn key_table<'a, S: Into<Cow<'a, str>>>(key_table: Option<S>) -> TmuxCommand<'a> {
        Self::set(KEY_TABLE, key_table)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_after_time<'a>(number: Option<usize>) -> TmuxCommand<'a> {
        Self::set(LOCK_AFTER_TIME, number.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_command<'a, S: Into<Cow<'a, str>>>(shell_command: Option<S>) -> TmuxCommand<'a> {
        Self::set(LOCK_COMMAND, shell_command)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn lock_server<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(LOCK_SERVER, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_attr<'a, S: Into<Cow<'a, str>>>(attributes: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_attr<'a, S: Into<Cow<'a, str>>>(attributes: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_COMMAND_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_COMMAND_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_COMMAND_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_command_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_COMMAND_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn message_limit<'a>(number: Option<usize>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_LIMIT, number.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(MESSAGE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_resize_pane<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MOUSE_RESIZE_PANE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_pane<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MOUSE_SELECT_PANE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_window<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MOUSE_SELECT_WINDOW, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn mouse<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MOUSE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn mouse_utf8<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(MOUSE_UTF8, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(PANE_ACTIVE_BORDER_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(PANE_ACTIVE_BORDER_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(PANE_BORDER_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(PANE_BORDER_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_active_border_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(PANE_ACTIVE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_border_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(PANE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn prefix<'a, S: Into<Cow<'a, str>>>(key: Option<S>) -> TmuxCommand<'a> {
        Self::set(PREFIX, key)
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn prefix2<'a, S: Into<Cow<'a, str>>>(key: Option<S>) -> TmuxCommand<'a> {
        Self::set(PREFIX2, key)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn renumber_windows<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(RENUMBER_WINDOWS, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn repeat_time<'a>(time: Option<usize>) -> TmuxCommand<'a> {
        Self::set(REPEAT_TIME, time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_remain_on_exit<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(SET_REMAIN_ON_EXIT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(SET_TITLES, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles_string<'a, S: Into<Cow<'a, str>>>(string: Option<S>) -> TmuxCommand<'a> {
        Self::set(SET_TITLES_STRING, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn silence_action<'a>(action: Option<Action>) -> TmuxCommand<'a> {
        Self::set(SILENCE_ACTION, action.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// status [off | on | 2 | 3 | 4 | 5]
    /// ```
    /// tmux 1.0:
    /// ```text
    /// status [off | on]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(STATUS, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_attr<'a, S: Into<Cow<'a, str>>>(attributes: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn status_format<'a, S: Into<Cow<'a, str>>>(format: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_FORMAT, format)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_interval<'a>(interval: Option<usize>) -> TmuxCommand<'a> {
        Self::set(STATUS_INTERVAL, interval.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_justify<'a>(status_justify: Option<StatusJustify>) -> TmuxCommand<'a> {
        Self::set(STATUS_JUSTIFY, status_justify.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_keys<'a>(status_keys: Option<StatusKeys>) -> TmuxCommand<'a> {
        Self::set(STATUS_KEYS, status_keys.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left<'a, S: Into<Cow<'a, str>>>(string: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_LEFT, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_attr<'a, S: Into<Cow<'a, str>>>(attributes: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_LEFT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_LEFT_BG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_LEFT_FG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left_length<'a>(length: Option<usize>) -> TmuxCommand<'a> {
        Self::set(STATUS_LEFT_LENGTH, length.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_left_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_LEFT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn status_position<'a>(status_position: Option<StatusPosition>) -> TmuxCommand<'a> {
        Self::set(STATUS_POSITION, status_position.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right<'a, S: Into<Cow<'a, str>>>(string: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_RIGHT, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_attr<'a, S: Into<Cow<'a, str>>>(attributes: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_RIGHT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_bg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_RIGHT_BG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_fg<'a, S: Into<Cow<'a, str>>>(colour: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_RIGHT_FG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right_length<'a>(length: Option<usize>) -> TmuxCommand<'a> {
        Self::set(STATUS_RIGHT_LENGTH, length.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_right_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_RIGHT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(STATUS_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn status_utf8<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(STATUS_UTF8, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn terminal_overrides<'a, S: Into<Cow<'a, str>>>(string: Option<S>) -> TmuxCommand<'a> {
        Self::set(TERMINAL_OVERRIDES, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn update_environment<'a, S: Into<Cow<'a, str>>>(variable: Option<S>) -> TmuxCommand<'a> {
        Self::set(UPDATE_ENVIRONMENT, variable)
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn user_keys<'a, S: Into<Cow<'a, str>>>(key: Option<S>) -> TmuxCommand<'a> {
        Self::set(USER_KEYS, key)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-activity [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```
    /// visual-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn visual_activity<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(VISUAL_ACTIVITY, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-bell [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```
    /// visual-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn visual_bell<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(VISUAL_BELL, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn visual_content<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(VISUAL_CONTENT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn visual_silence<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(VISUAL_SILENCE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn word_separators<'a, S: Into<Cow<'a, str>>>(string: Option<S>) -> TmuxCommand<'a> {
        Self::set(WORD_SEPARATORS, string)
    }

    //pub user_options: Option<HashMap<String, String>>
}

//#[test]
//fn parse_server_option() {
//use crate::options::get_server_option::{GetServerOption, TmuxServerOptionOutput};
//use crate::Tmux;

//#[cfg(feature = "tmux_3_1")]
//{
//let origin = "C-?";
//let output = Tmux::new()
//.command(GetServerOption::backspace())
//.output()
//.unwrap();
//let value = TmuxServerOptionOutput::from(output).backspace().unwrap();
//assert_eq!(origin, value);
//}

//#[cfg(feature = "tmux_1_5")]
//{
//let origin = 50;
//let output = Tmux::new()
//.command(GetServerOption::buffer_limit())
//.output()
//.unwrap();
//let value = TmuxServerOptionOutput::from(output).buffer_limit().unwrap();
//assert_eq!(origin, value);
//}
//}

//#[test]
//fn get_server_option_c() {
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
