// NOTE: DRY, global and local session options structures have same methods therefore common setter
// and getter traits were choosen for common use super::set_session_option::Self::Setter;
//
use crate::{Action, Activity, DetachOnDestroy, Status, StatusJustify};
use crate::{
    SetGlobalSessionOption, SetLocalSessionOption, SetSessionOption, StatusKeys, StatusPosition,
    Switch, TmuxCommand, TmuxCommands,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct SetLocalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

#[derive(Debug)]
pub struct SetGlobalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetSessionOptions<'a> for SetLocalSessionOptions<'a> {
    type Setter = SetLocalSessionOption;

    fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }

    fn push_cmds(&mut self, options: TmuxCommands<'a>) {
        self.options.push_cmds(options);
    }

    fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}

impl<'a> SetSessionOptions<'a> for SetGlobalSessionOptions<'a> {
    type Setter = SetGlobalSessionOption;

    fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }

    fn push_cmds(&mut self, options: TmuxCommands<'a>) {
        self.options.push_cmds(options);
    }

    fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}

//impl<'a> SetGlobalSessionOptions<'a> {}

//impl<'a> SetLocalSessionOptions<'a> {
//pub fn new() -> Self{
//Self {
//options: TmuxCommands::new(),
//}
//}

////pub fn push(&mut self, option: TmuxCommand<'a>) {
////self.push(option);
////}
//}

pub trait SetSessionOptions<'a> {
    type Setter: SetSessionOption;

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
    fn activity_action(mut self, action: Option<Action>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::activity_action(action));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn assume_paste_time(mut self, milliseconds: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::assume_paste_time(milliseconds));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn base_index(mut self, index: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::base_index(index));
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
    fn bell_action(mut self, action: Option<Action>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::bell_action(action));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn bell_on_alert(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::BELL_ON_ALERT(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn buffer_limit(mut self, limit: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::buffer_limit(limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_command<S: Into<Cow<'a, str>>>(mut self, shell_command: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_command(shell_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_shell<S: Into<Cow<'a, str>>>(mut self, path: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_shell(path));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn default_path<S: Into<Cow<'a, str>>>(mut self, path: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_path(path));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn default_terminal<S: Into<Cow<'a, str>>>(mut self, terminal: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_terminal(terminal));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn default_size(mut self, size: Option<(usize, usize)>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::default_size(size));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn destroy_unattached(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::destroy_unattached(switch));
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
    fn detach_on_destroy(mut self, detach_on_destroy: Option<DetachOnDestroy>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::detach_on_destroy(detach_on_destroy));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn display_panes_active_colour<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::display_panes_active_colour(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_colour<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::display_panes_colour(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_time(mut self, time: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::display_panes_time(time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_time(mut self, time: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::display_time(time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn history_limit(mut self, lines: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::history_limit(lines));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn key_table<S: Into<Cow<'a, str>>>(mut self, key_table: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::key_table(key_table));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_after_time(mut self, number: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::lock_after_time(number));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_command<S: Into<Cow<'a, str>>>(mut self, shell_command: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::lock_command(shell_command));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn lock_server(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::lock_server(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.options
            .push(Self::Self::Setter::message_command_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_command_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_command_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_command_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_command_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn message_limit(mut self, number: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_limit(number));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::message_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_resize_pane(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mouse_resize_pane(switch));
        self
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_pane(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mouse_select_pane(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_window(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mouse_select_window(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn mouse(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mouse(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn mouse_utf8(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mouse_utf8(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.options
            .push(Self::Setter::pane_active_border_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.options
            .push(Self::Setter::pane_active_border_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_active_border_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.options
            .push(Self::Setter::pane_active_border_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_border_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn prefix<S: Into<Cow<'a, str>>>(mut self, key: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::prefix(key));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn prefix2<S: Into<Cow<'a, str>>>(mut self, key: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::prefix2(key));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn renumber_windows(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::renumber_windows(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn repeat_time(mut self, time: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::repeat_time(time));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_remain_on_exit(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::set_remain_on_exit(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::set_titles(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles_string<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::set_titles_string(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn silence_action(mut self, action: Option<Action>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::silence_action(action));
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
    fn status(mut self, status: Option<Status>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status(status));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn status_format(mut self, format: Option<Vec<String>>) -> Self
    where
        Self: Sized,
    {
        self.push_cmds(Self::Setter::status_format(format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_interval(mut self, interval: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_interval(interval));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_justify(mut self, status_justify: Option<StatusJustify>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_justify(status_justify));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_keys(mut self, status_keys: Option<StatusKeys>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_keys(status_keys));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_left(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.options
            .push(Self::Setter::status_left_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_left_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_left_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left_length(mut self, length: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_left_length(length));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_left_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_left_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn status_position(mut self, status_position: Option<StatusPosition>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_position(status_position));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_right(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.options
            .push(Self::Setter::status_right_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_right_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_right_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right_length(mut self, length: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_right_length(length));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_right_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_right_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn status_utf8(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::status_utf8(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn terminal_overrides<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::terminal_overrides(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn update_environment(mut self, variable: Option<Vec<String>>) -> Self
    where
        Self: Sized,
    {
        self.push_cmds(Self::Setter::update_environment(variable));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys[] key
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn user_keys<S: Into<Cow<'a, str>>>(mut self, key: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::user_keys(key));
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
    fn visual_activity(mut self, activity: Option<Activity>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::visual_activity(activity));
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
    fn visual_bell(mut self, activity: Option<Activity>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::visual_bell(activity));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn visual_content(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::visual_content(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn visual_silence(mut self, activity: Option<Activity>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::visual_silence(activity));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn word_separators<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::word_separators(string));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
