//! # Selective Session Options Command Getter Builder
//!
//! Allows building commands for getting global and "local" session options
//!
//! Differences between global and "local" session options ([tmux man](https://man7.org/linux/man-pages/man1/tmux.1.html#OPTIONS))
//!
//! ## Global
//!
//! ### Examples
//!
//! ```
//! let options = GetGlobalSessionOptions::new().activity_action().assume_paste_time();
//! let output = Tmux::with_commands(options).output().unwrap();
//! // parse
//! ```
//!
//! request:
//! ```text
//! tmux show-options -g activity-action ; show-options -g assume-paste-time
//! ```
//!
//! response:
//! ```text
//! activity-action other
//! assume-paste-time 1
//! ```
//!
//!
//! ## Local
//!
//! ### Examples
//!
//! ```
//! let options = GetLocalSessionOptions::new().activity_action().assume_paste_time();
//! let output = Tmux::with_commands(options).output().unwrap();
//! // parse
//! ```
//!
//! request:
//! ```text
//! tmux show-options activity-action ; show-options assume-paste-time
//! ```
//!
//! response:
//! ```text
//! ```

use crate::options::session::get_session_option::{
    GetGlobalSessionOption, GetLocalSessionOption, GetSessionOption,
};
use crate::{TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct GetGlobalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

#[derive(Debug)]
pub struct GetLocalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

// XXX: both are same, optimize
impl<'a> GetSessionOptions<'a, GetLocalSessionOption> for GetLocalSessionOptions<'a> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.options.push(cmd.into())
    }

    fn into_commands(self) -> TmuxCommands<'a> {
        self.options
    }
}

// XXX: both are same, optimize
impl<'a> GetSessionOptions<'a, GetGlobalSessionOption> for GetGlobalSessionOptions<'a> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.options.push(cmd.into())
    }

    fn into_commands(self) -> TmuxCommands<'a> {
        self.options
    }
}

pub trait GetSessionOptions<'a, Getter: GetSessionOption> {
    fn new() -> Self;

    fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T);

    fn into_commands(self) -> TmuxCommands<'a>;

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn activity_action(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::activity_action());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn assume_paste_time(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::assume_paste_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn base_index(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::base_index());
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
    fn bell_action(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::bell_action());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn bell_on_alert(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::BELL_ON_ALERT());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn buffer_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::buffer_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_command(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::default_command());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_shell(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::default_shell());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn default_path(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::default_path());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn default_terminal(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::default_terminal());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn default_size(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::default_size());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn destroy_unattached(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::destroy_unattached());
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
    fn detach_on_destroy(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::detach_on_destroy());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn display_panes_active_colour(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::display_panes_active_colour());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_colour(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::display_panes_colour());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_time(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::display_panes_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_time(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::display_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn history_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::history_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn key_table(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::key_table());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_after_time(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::lock_after_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_command(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::lock_command());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn lock_server(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::lock_server());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_command_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_command_bg());
        self
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
    fn message_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_command_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_command_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn message_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::message_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_resize_pane(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mouse_resize_pane());
        self
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_pane(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mouse_select_pane());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_window(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mouse_select_window());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn mouse(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mouse());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn mouse_utf8(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mouse_utf8());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_active_border_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_active_border_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_border_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_border_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_active_border_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.options.push(Getter::pane_active_border_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_border_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_border_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn prefix(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::prefix());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn prefix2(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::prefix2());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn renumber_windows(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::renumber_windows());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn repeat_time(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::repeat_time());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_remain_on_exit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::set_remain_on_exit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::set_titles());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles_string(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::set_titles_string());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn silence_action(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::silence_action());
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
    fn status(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_attr(mut self) -> SelSelf {
        self.push(Getter::status_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn status_format(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_format());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_interval(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_interval());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_justify(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_justify());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_keys(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_keys());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_left());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_left_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_left_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_left_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left_length(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_left_length());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_left_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_left_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn status_position(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_position());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_right());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_right_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_right_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_right_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right_length(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_right_length());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_right_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_right_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn status_utf8(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::status_utf8());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn terminal_overrides(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::terminal_overrides());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn update_environment(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::update_environment());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn user_keys(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::user_keys());
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
    fn visual_activity(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::visual_activity());
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
    fn visual_bell(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::visual_bell());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn visual_content(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::visual_content());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn visual_silence(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::visual_silence());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn word_separators(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::word_separators());
        self
    }

    //fn into_commands(self) -> TmuxCommands<'a> {
    //self.options
    //}
}
