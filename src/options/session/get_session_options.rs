use super::get_session_option::GetSessionOption;
use crate::TmuxCommands;

#[derive(Debug)]
pub struct GetSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

#[derive(Debug)]
pub struct GetGlobalSessionOptions<'a>(GetSessionOptions<'a>);

//impl Getter for GetGlobalSessionOptions {
//fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
//GetGlobalSessionOption
//}
//}

//pub trait Getter {
//fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
//ShowOptions::new().option(name).value().build()
//}
//}

//impl Getter for GetSessionOptions {
//fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> GetSessionOption {
//ShowOptions::new().option(name.into()).value().build()
//}
//}

impl<'a> std::ops::Deref for GetGlobalSessionOptions<'a> {
    type Target = GetSessionOptions<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> GetSessionOptions<'a> {
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
    pub fn activity_action(mut self) -> Self {
        self.options.push(GetSessionOption::activity_action());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    pub fn assume_paste_time(mut self) -> Self {
        self.options.push(GetSessionOption::assume_paste_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn base_index(mut self) -> Self {
        self.options.push(GetSessionOption::base_index());
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
    pub fn bell_action(mut self) -> Self {
        self.options.push(GetSessionOption::bell_action());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    pub fn bell_on_alert(mut self) -> Self {
        self.options.push(GetSessionOption::BELL_ON_ALERT());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    pub fn buffer_limit(mut self) -> Self {
        self.options.push(GetSessionOption::buffer_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn default_command(mut self) -> Self {
        self.options.push(GetSessionOption::default_command());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn default_shell(mut self) -> Self {
        self.options.push(GetSessionOption::default_shell());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn default_path(mut self) -> Self {
        self.options.push(GetSessionOption::default_path());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn default_terminal(mut self) -> Self {
        self.options.push(GetSessionOption::default_terminal());
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
        self.options.push(GetSessionOption::default_size());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn destroy_unattached(mut self) -> Self {
        self.options.push(GetSessionOption::destroy_unattached());
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
    pub fn detach_on_destroy(mut self) -> Self {
        self.options.push(GetSessionOption::detach_on_destroy());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub fn display_panes_active_colour(mut self) -> Self {
        self.options
            .push(GetSessionOption::display_panes_active_colour());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_colour(mut self) -> Self {
        self.options.push(GetSessionOption::display_panes_colour());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes_time(mut self) -> Self {
        self.options.push(GetSessionOption::display_panes_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn display_time(mut self) -> Self {
        self.options.push(GetSessionOption::display_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn history_limit(mut self) -> Self {
        self.options.push(GetSessionOption::history_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    pub fn key_table(mut self) -> Self {
        self.options.push(GetSessionOption::key_table());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn lock_after_time(mut self) -> Self {
        self.options.push(GetSessionOption::lock_after_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn lock_command(mut self) -> Self {
        self.options.push(GetSessionOption::lock_command());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    pub fn lock_server(mut self) -> Self {
        self.options.push(GetSessionOption::lock_server());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_attr(mut self) -> Self {
        self.options.push(GetSessionOption::message_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_bg(mut self) -> Self {
        self.options.push(GetSessionOption::message_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_attr(mut self) -> Self {
        self.options.push(GetSessionOption::message_command_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_bg(mut self) -> Self {
        self.options.push(GetSessionOption::message_command_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn message_command_fg(mut self) -> Self {
        self.options.push(GetSessionOption::message_command_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn message_fg(mut self) -> Self {
        self.options.push(GetSessionOption::message_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn message_command_style(mut self) -> Self {
        self.options.push(GetSessionOption::message_command_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    pub fn message_limit(mut self) -> Self {
        self.options.push(GetSessionOption::message_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn message_style(mut self) -> Self {
        self.options.push(GetSessionOption::message_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_resize_pane(mut self) -> Self {
        self.options.push(GetSessionOption::mouse_resize_pane());
        self
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_pane(mut self) -> Self {
        self.options.push(GetSessionOption::mouse_select_pane());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    pub fn mouse_select_window(mut self) -> Self {
        self.options.push(GetSessionOption::mouse_select_window());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse(mut self) -> Self {
        self.options.push(GetSessionOption::mouse());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    pub fn mouse_utf8(mut self) -> Self {
        self.options.push(GetSessionOption::mouse_utf8());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_bg(mut self) -> Self {
        self.options.push(GetSessionOption::pane_active_border_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_fg(mut self) -> Self {
        self.options.push(GetSessionOption::pane_active_border_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_bg(mut self) -> Self {
        self.options.push(GetSessionOption::pane_border_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    pub fn pane_border_fg(mut self) -> Self {
        self.options.push(GetSessionOption::pane_border_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_active_border_style(mut self) -> Self {
        self.options
            .push(GetSessionOption::pane_active_border_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn pane_border_style(mut self) -> Self {
        self.options.push(GetSessionOption::pane_border_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn prefix(mut self) -> Self {
        self.options.push(GetSessionOption::prefix());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn prefix2(mut self) -> Self {
        self.options.push(GetSessionOption::prefix2());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn renumber_windows(mut self) -> Self {
        self.options.push(GetSessionOption::renumber_windows());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn repeat_time(mut self) -> Self {
        self.options.push(GetSessionOption::repeat_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    pub fn set_remain_on_exit(mut self) -> Self {
        self.options.push(GetSessionOption::set_remain_on_exit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles(mut self) -> Self {
        self.options.push(GetSessionOption::set_titles());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn set_titles_string(mut self) -> Self {
        self.options.push(GetSessionOption::set_titles_string());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn silence_action(mut self) -> Self {
        self.options.push(GetSessionOption::silence_action());
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
    pub fn status(mut self) -> Self {
        self.options.push(GetSessionOption::status());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_attr(mut self) -> SelSelf {
        self.options.push(GetSessionOption::status_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_bg(mut self) -> Self {
        self.options.push(GetSessionOption::status_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_fg(mut self) -> Self {
        self.options.push(GetSessionOption::status_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub fn status_format(mut self) -> Self {
        self.options.push(GetSessionOption::status_format());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_interval(mut self) -> Self {
        self.options.push(GetSessionOption::status_interval());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_justify(mut self) -> Self {
        self.options.push(GetSessionOption::status_justify());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_keys(mut self) -> Self {
        self.options.push(GetSessionOption::status_keys());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_left(mut self) -> Self {
        self.options.push(GetSessionOption::status_left());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_attr(mut self) -> Self {
        self.options.push(GetSessionOption::status_left_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_bg(mut self) -> Self {
        self.options.push(GetSessionOption::status_left_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_left_fg(mut self) -> Self {
        self.options.push(GetSessionOption::status_left_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_left_length(mut self) -> Self {
        self.options.push(GetSessionOption::status_left_length());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_left_style(mut self) -> Self {
        self.options.push(GetSessionOption::status_left_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub fn status_position(mut self) -> Self {
        self.options.push(GetSessionOption::status_position());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_right(mut self) -> Self {
        self.options.push(GetSessionOption::status_right());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_attr(mut self) -> Self {
        self.options.push(GetSessionOption::status_right_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_bg(mut self) -> Self {
        self.options.push(GetSessionOption::status_right_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn status_right_fg(mut self) -> Self {
        self.options.push(GetSessionOption::status_right_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn status_right_length(mut self) -> Self {
        self.options.push(GetSessionOption::status_right_length());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_right_style(mut self) -> Self {
        self.options.push(GetSessionOption::status_right_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub fn status_style(mut self) -> Self {
        self.options.push(GetSessionOption::status_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn status_utf8(mut self) -> Self {
        self.options.push(GetSessionOption::status_utf8());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn terminal_overrides(mut self) -> Self {
        self.options.push(GetSessionOption::terminal_overrides());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub fn update_environment(mut self) -> Self {
        self.options.push(GetSessionOption::update_environment());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    pub fn user_keys(mut self) -> Self {
        self.options.push(GetSessionOption::user_keys());
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
    pub fn visual_activity(mut self) -> Self {
        self.options.push(GetSessionOption::visual_activity());
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
    pub fn visual_bell(mut self) -> Self {
        self.options.push(GetSessionOption::visual_bell());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn visual_content(mut self) -> Self {
        self.options.push(GetSessionOption::visual_content());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub fn visual_silence(mut self) -> Self {
        self.options.push(GetSessionOption::visual_silence());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub fn word_separators(mut self) -> Self {
        self.options.push(GetSessionOption::word_separators());
        self
    }

    pub fn into_commands(self) -> TmuxCommands<'a> {
        self.options
    }
}

//#[test]
//fn get_session_options() {
//use crate::Tmux;

//let get_options = GetGlobalSessionOptions::new()
//.visual_silence()
//.word_separators()
//.into_commands();

//let output = Tmux::new().commands(get_options).output();

//dbg!(output);
//}
