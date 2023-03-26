use crate::options::SetOptionExt;
use crate::options::*;
use crate::{Action, Activity, DetachOnDestroy, Status, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
pub trait SetSessionOption: SetOptionExt {
    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn activity_action<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        action: Option<Action>,
    ) -> TmuxCommand<'a> {
        Self::set(target, ACTIVITY_ACTION, action.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn assume_paste_time<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        milliseconds: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
            ASSUME_PASTE_TIME,
            milliseconds.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn base_index<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        index: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, BASE_INDEX, index.map(|s| s.to_string()))
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
    fn bell_action<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        action: Option<Action>,
    ) -> TmuxCommand<'a> {
        Self::set(target, BELL_ACTION, action.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn bell_on_alert<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, BELL_ON_ALERT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn buffer_limit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        limit: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, BUFFER_LIMIT, limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_command<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        shell_command: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DEFAULT_COMMAND, shell_command)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_shell<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        path: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DEFAULT_SHELL, path)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn default_path<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        path: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DEFAULT_PATH, path)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn default_terminal<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        terminal: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DEFAULT_TERMINAL, terminal)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn default_size<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        size: Option<(usize, usize)>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
            DEFAULT_SIZE,
            size.map(|s| format!("{}x{}", s.0, s.1)),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn destroy_unattached<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DESTROY_UNATTACHED, switch.map(|s| s.to_string()))
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
    fn detach_on_destroy<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        detach_on_destroy: Option<DetachOnDestroy>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
            DETACH_ON_DESTROY,
            detach_on_destroy.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn display_panes_active_colour<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DISPLAY_PANES_ACTIVE_COLOUR, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_colour<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DISPLAY_PANES_COLOUR, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_time<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        time: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DISPLAY_PANES_TIME, time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_time<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        time: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, DISPLAY_TIME, time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn history_limit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        lines: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, HISTORY_LIMIT, lines.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn key_table<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        key_table: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, KEY_TABLE, key_table)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_after_time<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        number: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, LOCK_AFTER_TIME, number.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_command<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        shell_command: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, LOCK_COMMAND, shell_command)
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn lock_server<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, LOCK_SERVER, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_attr<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        attributes: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_attr<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        attributes: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_COMMAND_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_COMMAND_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_COMMAND_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_command_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_COMMAND_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn message_limit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        number: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_LIMIT, number.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MESSAGE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_resize_pane<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MOUSE_RESIZE_PANE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_pane<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MOUSE_SELECT_PANE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_window<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MOUSE_SELECT_WINDOW, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn mouse<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MOUSE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn mouse_utf8<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, MOUSE_UTF8, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PANE_ACTIVE_BORDER_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PANE_ACTIVE_BORDER_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PANE_BORDER_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PANE_BORDER_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_active_border_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PANE_ACTIVE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_border_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PANE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn prefix<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        key: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PREFIX, key)
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn prefix2<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        key: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, PREFIX2, key)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn renumber_windows<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, RENUMBER_WINDOWS, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn repeat_time<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        time: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, REPEAT_TIME, time.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_remain_on_exit<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, SET_REMAIN_ON_EXIT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, SET_TITLES, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles_string<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        string: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, SET_TITLES_STRING, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn silence_action<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        action: Option<Action>,
    ) -> TmuxCommand<'a> {
        Self::set(target, SILENCE_ACTION, action.map(|s| s.to_string()))
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
    fn status<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        status: Option<Status>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS, status.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_attr<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        attributes: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn status_format<'a, T, I, S>(target: Option<T>, format: Option<I>) -> TmuxCommands<'a>
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(target, STATUS_FORMAT, format)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_interval<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        interval: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_INTERVAL, interval.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_justify<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        status_justify: Option<StatusJustify>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
            STATUS_JUSTIFY,
            status_justify.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_keys<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        status_keys: Option<StatusKeys>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_KEYS, status_keys.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        string: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_LEFT, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_attr<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        attributes: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_LEFT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_LEFT_BG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_LEFT_FG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left_length<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        length: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_LEFT_LENGTH, length.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_left_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_LEFT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn status_position<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        status_position: Option<StatusPosition>,
    ) -> TmuxCommand<'a> {
        Self::set(
            target,
            STATUS_POSITION,
            status_position.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        string: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_RIGHT, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_attr<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        attributes: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_RIGHT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_bg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_RIGHT_BG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_fg<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        colour: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_RIGHT_FG, colour.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right_length<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        length: Option<usize>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_RIGHT_LENGTH, length.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_right_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_RIGHT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_style<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn status_utf8<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, STATUS_UTF8, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn terminal_overrides<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        string: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, TERMINAL_OVERRIDES, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn update_environment<'a, T, I, S>(target: Option<T>, variable: Option<I>) -> TmuxCommands<'a>
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(target, UPDATE_ENVIRONMENT, variable)
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn user_keys<'a, T, I, S>(target: Option<T>, key: Option<I>) -> TmuxCommands<'a>
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_array(target, USER_KEYS, key)
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
    fn visual_activity<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        activity: Option<Activity>,
    ) -> TmuxCommand<'a> {
        Self::set(target, VISUAL_ACTIVITY, activity.map(|s| s.to_string()))
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
    fn visual_bell<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        activity: Option<Activity>,
    ) -> TmuxCommand<'a> {
        Self::set(target, VISUAL_BELL, activity.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn visual_content<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        switch: Option<Switch>,
    ) -> TmuxCommand<'a> {
        Self::set(target, VISUAL_CONTENT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn visual_silence<'a, T: Into<Cow<'a, str>>>(
        target: Option<T>,
        activity: Option<Activity>,
    ) -> TmuxCommand<'a> {
        Self::set(target, VISUAL_SILENCE, activity.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn word_separators<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<T>,
        string: Option<S>,
    ) -> TmuxCommand<'a> {
        Self::set(target, WORD_SEPARATORS, string)
    }

    //pub user_options: Option<HashMap<String, String>>
}
