use crate::options::*;
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?

pub struct GetSessionOption;

pub struct GetGlobalSessionOption(GetSessionOption);

impl std::ops::Deref for GetGlobalSessionOption {
    type Target = GetSessionOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Getter for GetGlobalSessionOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .global()
            .option(name.into())
            .value()
            .build()
    }
}

pub trait Getter {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().option(name).value().build()
    }
}

impl Getter for GetSessionOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().option(name.into()).value().build()
    }
}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
impl GetSessionOption {
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
    pub fn activity_action<'a>() -> TmuxCommand<'a> {
        Self::get(ACTIVITY_ACTION)
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    pub fn assume_paste_time<'a>() -> TmuxCommand<'a> {
        Self::get(ASSUME_PASTE_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn base_index<'a>() -> TmuxCommand<'a> {
        Self::get(BASE_INDEX)
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
    pub fn bell_action<'a>() -> TmuxCommand<'a> {
        Self::get(BELL_ACTION)
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub fn bell_on_alert<'a>() -> TmuxCommand<'a> {
        Self::get(BELL_ON_ALERT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub fn buffer_limit<'a>() -> TmuxCommand<'a> {
        Self::get(BUFFER_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn default_command<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_COMMAND)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn default_shell<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_SHELL)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn default_path<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_PATH)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn default_terminal<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_TERMINAL)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub fn default_size<'a>() -> TmuxCommand<'a> {
        Self::get(DEFAULT_SIZE)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn destroy_unattached<'a>() -> TmuxCommand<'a> {
        Self::get(DESTROY_UNATTACHED)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// detach-on-destroy [on | off | no-detached]
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// detach-on-destroy [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn detach_on_destroy<'a>() -> TmuxCommand<'a> {
        Self::get(DETACH_ON_DESTROY)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn display_panes_active_colour<'a>() -> TmuxCommand<'a> {
        Self::get(DISPLAY_PANES_ACTIVE_COLOUR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_colour<'a>() -> TmuxCommand<'a> {
        Self::get(DISPLAY_PANES_COLOUR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_time<'a>() -> TmuxCommand<'a> {
        Self::get(DISPLAY_PANES_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_time<'a>() -> TmuxCommand<'a> {
        Self::get(DISPLAY_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn history_limit<'a>() -> TmuxCommand<'a> {
        Self::get(HISTORY_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    pub fn key_table<'a>() -> TmuxCommand<'a> {
        Self::get(KEY_TABLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn lock_after_time<'a>() -> TmuxCommand<'a> {
        Self::get(LOCK_AFTER_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn lock_command<'a>() -> TmuxCommand<'a> {
        Self::get(LOCK_COMMAND)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub fn lock_server<'a>() -> TmuxCommand<'a> {
        Self::get(LOCK_SERVER)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_attr<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_bg<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_attr<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_COMMAND_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_bg<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_COMMAND_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_fg<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_COMMAND_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_fg<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn message_command_style<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_COMMAND_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn message_limit<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn message_style<'a>() -> TmuxCommand<'a> {
        Self::get(MESSAGE_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_resize_pane<'a>() -> TmuxCommand<'a> {
        Self::get(MOUSE_RESIZE_PANE)
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_pane<'a>() -> TmuxCommand<'a> {
        Self::get(MOUSE_SELECT_PANE)
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_window<'a>() -> TmuxCommand<'a> {
        Self::get(MOUSE_SELECT_WINDOW)
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse<'a>() -> TmuxCommand<'a> {
        Self::get(MOUSE)
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub fn mouse_utf8<'a>() -> TmuxCommand<'a> {
        Self::get(MOUSE_UTF8)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_bg<'a>() -> TmuxCommand<'a> {
        Self::get(PANE_ACTIVE_BORDER_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_fg<'a>() -> TmuxCommand<'a> {
        Self::get(PANE_ACTIVE_BORDER_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_bg<'a>() -> TmuxCommand<'a> {
        Self::get(PANE_BORDER_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_fg<'a>() -> TmuxCommand<'a> {
        Self::get(PANE_BORDER_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_active_border_style<'a>() -> TmuxCommand<'a> {
        Self::get(PANE_ACTIVE_BORDER_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_border_style<'a>() -> TmuxCommand<'a> {
        Self::get(PANE_BORDER_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn prefix<'a>() -> TmuxCommand<'a> {
        Self::get(PREFIX)
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn prefix2<'a>() -> TmuxCommand<'a> {
        Self::get(PREFIX2)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn renumber_windows<'a>() -> TmuxCommand<'a> {
        Self::get(RENUMBER_WINDOWS)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn repeat_time<'a>() -> TmuxCommand<'a> {
        Self::get(REPEAT_TIME)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn set_remain_on_exit<'a>() -> TmuxCommand<'a> {
        Self::get(SET_REMAIN_ON_EXIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles<'a>() -> TmuxCommand<'a> {
        Self::get(SET_TITLES)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles_string<'a>() -> TmuxCommand<'a> {
        Self::get(SET_TITLES_STRING)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn silence_action<'a>() -> TmuxCommand<'a> {
        Self::get(SILENCE_ACTION)
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
    pub fn status<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_attr<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_bg<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_fg<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_FG)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub fn status_format<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_FORMAT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_interval<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_INTERVAL)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_justify<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_JUSTIFY)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_keys<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_KEYS)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_left<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_LEFT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_attr<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_LEFT_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_bg<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_LEFT_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_fg<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_LEFT_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_left_length<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_LEFT_LENGTH)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_left_style<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_LEFT_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn status_position<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_POSITION)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_right<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_RIGHT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_attr<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_RIGHT_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_bg<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_RIGHT_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_fg<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_RIGHT_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_right_length<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_RIGHT_LENGTH)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_style<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_RIGHT_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_style<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn status_utf8<'a>() -> TmuxCommand<'a> {
        Self::get(STATUS_UTF8)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn terminal_overrides<'a>() -> TmuxCommand<'a> {
        Self::get(TERMINAL_OVERRIDES)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn update_environment<'a>() -> TmuxCommand<'a> {
        Self::get(UPDATE_ENVIRONMENT)
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    pub fn user_keys<'a>() -> TmuxCommand<'a> {
        Self::get(USER_KEYS)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-activity [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```text
    /// visual-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn visual_activity<'a>() -> TmuxCommand<'a> {
        Self::get(VISUAL_ACTIVITY)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// visual-bell [on | off | both]
    /// ```
    ///
    /// tmux 1.0:
    /// ```text
    /// visual-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn visual_bell<'a>() -> TmuxCommand<'a> {
        Self::get(VISUAL_BELL)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn visual_content<'a>() -> TmuxCommand<'a> {
        Self::get(VISUAL_CONTENT)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn visual_silence<'a>() -> TmuxCommand<'a> {
        Self::get(VISUAL_SILENCE)
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn word_separators<'a>() -> TmuxCommand<'a> {
        Self::get(WORD_SEPARATORS)
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

#[test]
fn get_session_option() {
    use crate::Tmux;

    //let cmd = Tmux::new()
    //.command(GetSessionOption::get(BUFFER_LIMIT))
    //.output()
    //.unwrap();

    let cmd = Tmux::new()
        .command(GetSessionOption::activity_action())
        .command(GetSessionOption::base_index())
        .command(GetGlobalSessionOption::activity_action())
        .command(GetGlobalSessionOption::base_index());

    dbg!(&cmd);

    let output = cmd.output().unwrap();

    dbg!(&output);
}
