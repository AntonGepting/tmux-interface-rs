use super::set_session_option::SetSessionOption;
use super::{Action, Activity, StatusJustify};
use crate::{StatusKeys, StatusPosition, Switch, TmuxCommands};
use std::borrow::Cow;

#[derive(Debug)]
pub struct SetSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetSessionOptions<'a> {
    pub fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn activity_action(mut self, activity: Option<Activity>) -> Self {
        self.options
            .push(SetSessionOption::activity_action(activity));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    pub fn assume_paste_time(mut self, milliseconds: Option<usize>) -> Self {
        self.options
            .push(SetSessionOption::assume_paste_time(milliseconds));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn base_index(mut self, index: Option<usize>) -> Self {
        self.options.push(SetSessionOption::base_index(index));
        self
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
    pub fn bell_action(mut self, action: Option<Action>) -> Self {
        self.options.push(SetSessionOption::bell_action(action));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub fn bell_on_alert(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::BELL_ON_ALERT(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub fn buffer_limit(mut self, limit: Option<usize>) -> Self {
        self.options.push(SetSessionOption::buffer_limit(limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn default_command<S: Into<Cow<'a, str>>>(mut self, shell_command: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::default_command(shell_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn default_shell<S: Into<Cow<'a, str>>>(mut self, path: Option<S>) -> Self {
        self.options.push(SetSessionOption::default_shell(path));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn default_path<S: Into<Cow<'a, str>>>(mut self, path: Option<S>) -> Self {
        self.options.push(SetSessionOption::default_path(path));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn default_terminal<S: Into<Cow<'a, str>>>(mut self, terminal: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::default_terminal(terminal));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub fn default_size(mut self) -> Self {
        self.options.push(SetSessionOption::default_size());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn destroy_unattached(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::destroy_unattached(switch));
        self
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
    pub fn detach_on_destroy(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::detach_on_destroy(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn display_panes_active_colour<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::display_panes_active_colour(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_colour<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::display_panes_colour(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_time(mut self, time: Option<usize>) -> Self {
        self.options
            .push(SetSessionOption::display_panes_time(time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_time(mut self, time: Option<usize>) -> Self {
        self.options.push(SetSessionOption::display_time(time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn history_limit(mut self, lines: Option<usize>) -> Self {
        self.options.push(SetSessionOption::history_limit(lines));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    pub fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: Option<S>) -> Self {
        self.options.push(SetSessionOption::key_table(key_table));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn lock_after_time(mut self, number: Option<usize>) -> Self {
        self.options.push(SetSessionOption::lock_after_time(number));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn lock_command<S: Into<Cow<'a, str>>>(mut self, shell_command: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::lock_command(shell_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub fn lock_server(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::lock_server(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::message_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::message_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::message_command_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::message_command_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::message_command_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::message_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn message_command_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::message_command_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn message_limit(mut self, number: Option<usize>) -> Self {
        self.options.push(SetSessionOption::message_limit(number));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn message_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options.push(SetSessionOption::message_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_resize_pane(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::mouse_resize_pane(switch));
        self
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_pane(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::mouse_select_pane(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_window(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::mouse_select_window(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::mouse(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub fn mouse_utf8(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::mouse_utf8(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::pane_active_border_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::pane_active_border_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::pane_border_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::pane_border_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_active_border_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::pane_active_border_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_border_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::pane_border_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn prefix<S: Into<Cow<'a, str>>>(mut self, key: Option<S>) -> Self {
        self.options.push(SetSessionOption::prefix(key));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn prefix2<S: Into<Cow<'a, str>>>(mut self, key: Option<S>) -> Self {
        self.options.push(SetSessionOption::prefix2(key));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn renumber_windows(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::renumber_windows(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn repeat_time(mut self, time: Option<usize>) -> Self {
        self.options.push(SetSessionOption::repeat_time(time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn set_remain_on_exit(mut self, switch: Option<Switch>) -> Self {
        self.options
            .push(SetSessionOption::set_remain_on_exit(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::set_titles(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles_string<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::set_titles_string(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn silence_action(mut self, action: Option<Action>) -> Self {
        self.options.push(SetSessionOption::silence_action(action));
        self
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
    pub fn status(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::status(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub fn status_format<S: Into<Cow<'a, str>>>(mut self, format: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_format(format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_interval(mut self, interval: Option<usize>) -> Self {
        self.options
            .push(SetSessionOption::status_interval(interval));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_justify(mut self, status_justify: Option<StatusJustify>) -> Self {
        self.options
            .push(SetSessionOption::status_justify(status_justify));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_keys(mut self, status_keys: Option<StatusKeys>) -> Self {
        self.options
            .push(SetSessionOption::status_keys(status_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_left<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_left(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::status_left_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_left_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_left_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_left_length(mut self, length: Option<usize>) -> Self {
        self.options
            .push(SetSessionOption::status_left_length(length));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_left_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::status_left_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn status_position(mut self, status_position: Option<StatusPosition>) -> Self {
        self.options
            .push(SetSessionOption::status_position(status_position));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_right<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_right(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::status_right_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_right_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_right_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_right_length(mut self, length: Option<usize>) -> Self {
        self.options
            .push(SetSessionOption::status_right_length(length));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::status_right_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options.push(SetSessionOption::status_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn status_utf8(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::status_utf8(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn terminal_overrides<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self {
        self.options
            .push(SetSessionOption::terminal_overrides(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn update_environment(mut self, variable: Option<String>) -> Self {
        self.options
            .push(SetSessionOption::update_environment(variable));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    pub fn user_keys<S: Into<Cow<'a, str>>>(mut self, key: Option<S>) -> Self {
        self.options.push(SetSessionOption::user_keys(key));
        self
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
    pub fn visual_activity(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::visual_activity(switch));
        self
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
    pub fn visual_bell(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::visual_bell(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn visual_content(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::visual_content(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn visual_silence(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetSessionOption::visual_silence(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn word_separators<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self {
        self.options.push(SetSessionOption::word_separators(string));
        self
    }

    pub fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}
