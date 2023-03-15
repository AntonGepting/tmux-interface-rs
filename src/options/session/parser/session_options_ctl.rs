use crate::options::{GetSessionOption, SessionOptions, SetSessionOption};
use crate::{
    Action, Activity, DetachOnDestroy, Error, Status, StatusJustify, StatusKeys, StatusPosition,
    Switch, TmuxCommand, TmuxOutput,
};

// trait, subtrai for global local
pub trait SessionOptionsCtl<'a> {
    type Getter: GetSessionOption;
    type Setter: SetSessionOption;

    fn get_all(&self) -> Result<SessionOptions<'a>, Error>;
    fn set_all(self, server_options: SessionOptions<'a>) -> Result<TmuxOutput, Error>;

    fn get<T: std::str::FromStr>(&self, cmd: TmuxCommand<'a>) -> Result<Option<T>, Error>;

    fn set(&self, cmd: TmuxCommand<'a>) -> Result<TmuxOutput, Error>;

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn get_activity_action(&self) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::activity_action())
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn set_activity_action(&self, activity_action: Option<Action>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::activity_action(activity_action))
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn get_assume_paste_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::assume_paste_time())
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn set_assume_paste_time(&self, assume_paste_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::assume_paste_time(assume_paste_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_base_index(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::base_index())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_base_index(&self, base_index: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::base_index(base_index))
    }

    /// ### Manual
    ///
    /// ```text
    /// bell-action [any | none | current | other]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// bell-action [any | none | other]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_bell_action(&self) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::bell_action())
    }

    /// ### Manual
    ///
    /// ```text
    /// bell-action [any | none | current | other]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// bell-action [any | none | other]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_bell_action(&self, bell_action: Option<Action>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::bell_action(bell_action))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn get_bell_on_alert(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::bell_on_alert())
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn set_bell_on_alert(&self, bell_on_alert: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::bell_on_alert(bell_on_alert))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn get_buffer_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::buffer_limit())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn set_buffer_limit(&self, buffer_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::buffer_limit(buffer_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_default_command(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_command())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_default_command(&self, default_command: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_command(default_command))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_default_shell(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_shell())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_default_shell(&self, default_shell: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_shell(default_shell))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_default_path(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_path())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_default_path(&self, default_path: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_path(default_path))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn get_default_terminal(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_terminal())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn set_default_terminal(&self, default_terminal: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_terminal(default_terminal))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn get_default_size(&self) -> Result<Option<(usize, usize)>, Error> {
        self.get(Self::Getter::default_size())
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn set_default_size(&self, default_size: Option<(usize, usize)>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn get_destroy_unattached(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::destroy_unattached())
    }
    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn set_destroy_unattached(
        &self,
        destroy_unattached: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::destroy_unattached(destroy_unattached))
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
    fn get_detach_on_destroy(&self) -> Result<Option<DetachOnDestroy>, Error> {
        self.get(Self::Getter::detach_on_destroy())
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
    fn set_detach_on_destroy(
        &self,
        detach_on_destroy: Option<DetachOnDestroy>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::detach_on_destroy(detach_on_destroy))
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn get_display_panes_active_colour(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::display_panes_active_colour())
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn set_display_panes_active_colour(
        &self,
        display_panes_active_colour: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_active_colour(
            display_panes_active_colour,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_panes_colour(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::display_panes_colour())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_panes_colour(
        &self,
        display_panes_colour: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_colour(display_panes_colour))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_panes_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::display_panes_time())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_panes_time(
        &self,
        display_panes_time: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_panes_time(display_panes_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_display_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::display_time())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_display_time(&self, display_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::display_time(display_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_history_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::history_limit())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_history_limit(&self, history_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::history_limit(history_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn get_key_table(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::key_table())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn set_key_table(&self, key_table: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::key_table(key_table))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_lock_after_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::lock_after_time())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_lock_after_time(&self, lock_after_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_after_time(lock_after_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_lock_command(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::lock_command())
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_lock_command(&self, lock_command: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_command(lock_command))
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn get_lock_server(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::lock_server())
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn set_lock_server(&self, lock_server: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::lock_server(lock_server))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_attr())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_attr(&self, message_attr: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_attr(message_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_bg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_bg(&self, message_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_bg(message_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_message_command_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_attr())
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_message_command_attr(
        &self,
        message_command_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_attr(message_command_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_message_command_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_bg())
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_message_command_bg(
        &self,
        message_command_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_bg(message_command_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn get_default_size(&self) -> Result<Option, Error> {
        self.get(Self::Getter::default_size())
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn set_default_size(&self, default_size: Option) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_command_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_message_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_fg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_message_fg(&self, message_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_fg(message_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_message_command_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_command_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_message_command_style(
        &self,
        message_command_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_command_style(message_command_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn get_mesage_limit(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::mesage_limit())
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn set_mesage_limit(&self, message_limit: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mesage_limit(message_limit))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_message_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::message_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_message_style(&self, message_style: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::message_style(message_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_mouse_resize_pane(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_resize_pane())
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_mouse_resize_pane(
        &self,
        mouse_resize_pane: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_resize_pane(mouse_resize_pane))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_mouse_select_pane(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_select_pane())
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_mouse_select_pane(&self, default_size: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_select_pane(default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn get_select_window(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::select_window())
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn set_select_window(&self, select_window: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::select_window(select_window))
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn get_mouse(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse())
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn set_mouse(&self, mouse: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse(mouse))
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn get_mouse_utf8(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::mouse_utf8())
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn set_mouse_utf8(&self, mouse_utf8: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::mouse_utf8(mouse_utf8))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_active_border_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_bg())
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_active_border_bg(
        &self,
        pane_active_border_bg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_bg(pane_active_border_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_active_border_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_fg())
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_active_border_fg(
        &self,
        pane_active_border_fg: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_fg(pane_active_border_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_border_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_bg())
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_border_bg(&self, pane_border_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_bg(pane_border_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn get_pane_border_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_fg())
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn set_pane_border_fg(&self, pane_border_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_fg(pane_border_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn get_pane_active_border_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_active_border_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn set_pane_active_border_style(
        &self,
        pane_active_border_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_active_border_style(
            pane_active_border_style,
        ))
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn get_pane_border_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::pane_border_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn set_pane_border_style(
        &self,
        pane_border_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::pane_border_style(pane_border_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_prefix(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::prefix())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_prefix(&self, prefix: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::prefix(prefix))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn get_prefix2(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::prefix2())
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn set_prefix2(&self, prefix2: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::prefix2(prefix2))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn get_renumber_windows(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::renumber_windows())
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn set_renumber_windows(&self, renumber_windows: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::renumber_windows(renumber_windows))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_repeat_time(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::repeat_time())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_repeat_time(&self, repeat_time: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::repeat_time(repeat_time))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn get_set_remain_on_exit(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::set_remain_on_exit())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_set_remain_on_exit(
        &self,
        set_remain_on_exit: Option<Switch>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_remain_on_exit(set_remain_on_exit))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_set_titles(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::set_titles())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_set_titles(&self, set_titles: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_titles(set_titles))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_set_titles_string(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::set_titles_string())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_set_titles_string(
        &self,
        set_titles_string: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::set_titles_string(set_titles_string))
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn get_silence_action(&self) -> Result<Option<Action>, Error> {
        self.get(Self::Getter::silence_action())
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn set_silence_action(&self, silence_action: Option<Action>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::silence_action(silence_action))
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
    fn get_status(&self) -> Result<Option<Status>, Error> {
        self.get(Self::Getter::status())
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
    fn set_status(&self, status: Option<Status>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status(status))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_attr())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_attr(&self, status_attr: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_attr(status_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_bg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_bg(&self, status_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_bg(status_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_fg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_fg(&self, status_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_fg(status_fg))
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn get_status_format(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_format())
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn set_status_format(&self, status_format: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_format(status_format))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_interval(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_interval())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_interval(&self, status_interval: Option<usize>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_interval(status_interval))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_justify(&self) -> Result<Option<StatusJustify>, Error> {
        self.get(Self::Getter::status_justify())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_justify(
        &self,
        status_justify: Option<StatusJustify>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_justify(status_justify))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_keys(&self) -> Result<Option<StatusKeys>, Error> {
        self.get(Self::Getter::status_keys())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_keys(&self, status_keys: Option<StatusKeys>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_keys(status_keys))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_left(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_left(&self, status_left: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left(status_left))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_attr())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_attr(&self, status_left_attr: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_attr(status_left_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_bg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_bg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_bg(&self, status_left_bg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_bg(status_left_bg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_left_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_fg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_left_fg(&self, status_left_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_fg(status_left_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_left_length(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_left_length())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_left_length(
        &self,
        status_left_length: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_length(status_left_length))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_left_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_left_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_left_style(
        &self,
        status_left_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_left_style(status_left_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn get_status_position(&self) -> Result<Option<StatusPosition>, Error> {
        self.get(Self::Getter::status_position())
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn set_status_position(
        &self,
        status_position: Option<StatusPosition>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_position(status_position))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_right(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_right(&self, status_right: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right(status_right))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_right_attr(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_attr())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_right_attr(
        &self,
        status_right_attr: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_attr(status_right_attr))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_default_size(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::default_size())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_default_size(&self, default_size: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::default_size(default_size))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn get_status_right_fg(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_fg())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn set_status_right_fg(&self, status_right_fg: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_fg(status_right_fg))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn get_status_right_length(&self) -> Result<Option<usize>, Error> {
        self.get(Self::Getter::status_right_length())
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_status_right_length(
        &self,
        status_right_length: Option<usize>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_length(status_right_length))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_right_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_right_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_right_style(
        &self,
        status_right_style: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_right_style(status_right_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn get_status_style(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::status_style())
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn set_status_style(&self, status_style: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_style(status_style))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn get_status_utf8(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::status_utf8())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn set_status_utf8(&self, status_utf8: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::status_utf8(status_utf8))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_terminal_overrides(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::terminal_overrides())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn set_terminal_overrides(
        &self,
        terminal_overrides: Option<String>,
    ) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::terminal_overrides(terminal_overrides))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    // #[cfg(feature = "tmux_1_0")]
    // fn get_update_environment(&self) -> Result<Option<String>, Error> {
    // self.get(Self::Getter::update_environment())
    // }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    // #[cfg(feature = "tmux_1_0")]
    // fn set_update_environment(&self, update_environment: Option<Vec<String>>) -> Result<TmuxOutput, Error> {
    // self.set(Self::Setter::update_environment(update_environment))
    // }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn get_user_keys(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::user_keys())
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn set_user_keys(&self, user_keys: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::user_keys(user_keys))
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
    fn get_visual_activity(&self) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_activity())
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
    fn set_visual_activity(&self, visual_activity: Option<Activity>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_activity(visual_activity))
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
    fn get_visual_bell(&self) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_bell())
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
    fn set_visual_bell(&self, visual_bell: Option<Activity>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_bell(visual_bell))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn get_visual_content(&self) -> Result<Option<Switch>, Error> {
        self.get(Self::Getter::visual_content())
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn set_visual_content(&self, visual_content: Option<Switch>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_content(visual_content))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn get_visual_silence(&self) -> Result<Option<Activity>, Error> {
        self.get(Self::Getter::visual_silence())
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn set_visual_silence(&self, visual_silence: Option<Activity>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::visual_silence(visual_silence))
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn get_word_separators(&self) -> Result<Option<String>, Error> {
        self.get(Self::Getter::word_separators())
    }
    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn set_word_separators(&self, word_separators: Option<String>) -> Result<TmuxOutput, Error> {
        self.set(Self::Setter::word_separators(word_separators))
    }
}
