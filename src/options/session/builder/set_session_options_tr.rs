// NOTE: DRY, global and local session options structures have same methods therefore common setter
// and getter traits were choosen for common use super::set_session_option::Self::Setter;
//
use crate::{
    Action, Activity, DetachOnDestroy, SetSessionOptionTr, Status, StatusJustify, StatusKeys,
    StatusPosition, Switch, TmuxCommand, TmuxCommands,
};
use std::borrow::Cow;

pub trait SetSessionOptionsTr<'a> {
    type Setter: SetSessionOptionTr;

    fn new() -> Self;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn push_cmds(&mut self, options: TmuxCommands<'a>);

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn activity_action<T>(mut self, target: Option<T>, action: Option<Action>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::activity_action(target, action));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn assume_paste_time<T>(mut self, target: Option<T>, milliseconds: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::assume_paste_time(target, milliseconds));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn base_index<T>(mut self, target: Option<T>, index: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::base_index(target, index));
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
    fn bell_action<T>(mut self, target: Option<T>, action: Option<Action>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::bell_action(target, action));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn bell_on_alert<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::bell_on_alert(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn buffer_limit<T>(mut self, target: Option<T>, limit: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::buffer_limit(target, limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_command<T, S>(mut self, target: Option<T>, shell_command: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::default_command(target, shell_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_shell<T, S>(mut self, target: Option<T>, path: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::default_shell(target, path));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn default_path<T, S: Into<Cow<'a, str>>>(mut self, target: Option<T>, path: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::default_path(target, path));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn default_terminal<S, T>(mut self, target: Option<T>, terminal: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::default_terminal(target, terminal));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn default_size<T>(mut self, target: Option<T>, size: Option<(usize, usize)>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::default_size(target, size));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn destroy_unattached<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::destroy_unattached(target, switch));
        self
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
    fn detach_on_destroy<T>(
        mut self,
        target: Option<T>,
        detach_on_destroy: Option<DetachOnDestroy>,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::detach_on_destroy(target, detach_on_destroy));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn display_panes_active_colour<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::display_panes_active_colour(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_colour<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::display_panes_colour(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_time<T>(mut self, target: Option<T>, time: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::display_panes_time(target, time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_time<T>(mut self, target: Option<T>, time: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::display_time(target, time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn history_limit<T>(mut self, target: Option<T>, lines: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::history_limit(target, lines));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn key_table<T, S>(mut self, target: Option<T>, key_table: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::key_table(target, key_table));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_after_time<T>(mut self, target: Option<T>, number: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::lock_after_time(target, number));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_command<T, S>(mut self, target: Option<T>, shell_command: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::lock_command(target, shell_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn lock_server<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::lock_server(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_attr<T, S>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_attr<T, S>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.options
            .push(Self::Self::Setter::message_command_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_command_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_command_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_command_style<T, S>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_command_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn message_limit<T>(mut self, target: Option<T>, number: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_limit(target, number));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_style<T, S>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::message_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_resize_pane<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mouse_resize_pane(target, switch));
        self
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_pane<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mouse_select_pane(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_window<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mouse_select_window(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn mouse<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mouse(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn mouse_utf8<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mouse_utf8(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_active_border_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_active_border_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_active_border_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_active_border_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_border_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn prefix<T, S>(mut self, target: Option<T>, key: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::prefix(target, key));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn prefix2<T, S>(mut self, target: Option<T>, key: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::prefix2(target, key));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn renumber_windows<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::renumber_windows(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn repeat_time<T>(mut self, target: Option<T>, time: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::repeat_time(target, time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_remain_on_exit<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::set_remain_on_exit(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::set_titles(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles_string<T, S>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::set_titles_string(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn silence_action<T>(mut self, target: Option<T>, action: Option<Action>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::silence_action(target, action));
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
    fn status<T>(mut self, target: Option<T>, status: Option<Status>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status(target, status));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_attr<T, S>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn status_format<T, I, S>(mut self, target: Option<T>, format: Option<I>) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::status_format(target, format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_interval<T>(mut self, target: Option<T>, interval: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_interval(target, interval));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_justify<T>(mut self, target: Option<T>, status_justify: Option<StatusJustify>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_justify(target, status_justify));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_keys<T>(mut self, target: Option<T>, status_keys: Option<StatusKeys>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_keys(target, status_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left<T, S>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_left(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_attr<T, S>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_left_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_left_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_left_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left_length<T>(mut self, target: Option<T>, length: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_left_length(target, length));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_left_style<T, S>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_left_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn status_position<T>(
        mut self,
        target: Option<T>,
        status_position: Option<StatusPosition>,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_position(target, status_position));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right<T, S>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_right(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_attr<T, S>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_right_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_bg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_right_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_fg<T, S>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_right_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right_length<T>(mut self, target: Option<T>, length: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_right_length(target, length));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_right_style<T, S>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_right_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_style<T, S>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn status_utf8<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::status_utf8(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn terminal_overrides<S, T>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::terminal_overrides(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn update_environment<T, I, S>(mut self, target: Option<T>, variable: Option<I>) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::update_environment(target, variable));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn user_keys<T, I, S>(mut self, target: Option<T>, key: Option<I>) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push_cmds(Self::Setter::user_keys(target, key));
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
    /// ```text
    /// visual-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn visual_activity<T>(mut self, target: Option<T>, activity: Option<Activity>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::visual_activity(target, activity));
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
    /// ```text
    /// visual-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn visual_bell<T>(mut self, target: Option<T>, activity: Option<Activity>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::visual_bell(target, activity));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn visual_content<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::visual_content(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn visual_silence<T>(mut self, target: Option<T>, activity: Option<Activity>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::visual_silence(target, activity));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn word_separators<T, S: Into<Cow<'a, str>>>(
        mut self,
        target: Option<T>,
        string: Option<S>,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::word_separators(target, string));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
