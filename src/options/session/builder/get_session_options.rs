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
//! use tmux_interface::{Tmux, GetGlobalSessionOptions, GetSessionOptions};
//!
//! let options = GetGlobalSessionOptions::new().activity_action().assume_paste_time().into_commands();
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
//! use tmux_interface::{Tmux, GetLocalSessionOptions, GetSessionOptions};
//!
//! let options = GetLocalSessionOptions::new().activity_action().assume_paste_time().into_commands();
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

use crate::{GetSessionOption, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

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
    fn activity_action<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::activity_action(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8:
    /// ```text
    /// assume-paste-time milliseconds
    /// ```
    #[cfg(feature = "tmux_1_8")]
    fn assume_paste_time<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::assume_paste_time(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// base-index index
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn base_index<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::base_index(target));
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
    fn bell_action<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::bell_action(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.6:
    /// ```text
    /// bell-on-alert [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    fn bell_on_alert<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::BELL_ON_ALERT(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.4:
    /// ```text
    /// buffer-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    fn buffer_limit<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::buffer_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_command<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::default_command(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// default-shell path
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn default_shell<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::default_shell(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// default-path path
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn default_path<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::default_path(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// default-terminal terminal
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn default_terminal<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::default_terminal(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// default-size XxY
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn default_size<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::default_size(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// destroy-unattached [on | off]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn destroy_unattached<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::destroy_unattached(target));
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
    fn detach_on_destroy<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::detach_on_destroy(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// display-panes-active-colour colour
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn display_panes_active_colour<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::display_panes_active_colour(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_colour<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::display_panes_colour(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-panes-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_panes_time<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::display_panes_time(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// display-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn display_time<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::display_time(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// history-limit lines
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn history_limit<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::history_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// key-table key-table
    /// ```
    #[cfg(feature = "tmux_2_2")]
    fn key_table<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::key_table(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// lock-after-time number
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_after_time<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::lock_after_time(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-command shell-command
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn lock_command<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::lock_command(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.1:
    /// ```text
    /// lock-server [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    fn lock_server<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::lock_server(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_attr<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_attr<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_command_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_command_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// message-command-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn message_command_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_command_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// message-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn message_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-command-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_command_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_command_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v2.0:
    /// ```text
    /// message-limit number
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    fn message_limit<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// message-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn message_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::message_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-resize-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_resize_pane<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::mouse_resize_pane(target));
        self
    }
    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-pane [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_pane<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::mouse_select_pane(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.1:
    /// ```text
    /// mouse-select-window [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    fn mouse_select_window<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::mouse_select_window(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1:
    /// ```text
    /// mouse [on | off]
    /// ```
    #[cfg(feature = "tmux_2_1")]
    fn mouse<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::mouse(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.5 v2.2:
    /// ```text
    /// mouse-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    fn mouse_utf8<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::mouse_utf8(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::pane_active_border_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-active-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::pane_active_border_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::pane_border_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.9:
    /// ```text
    /// pane-border-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    fn pane_border_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::pane_border_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_active_border_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.options.push(Getter::pane_active_border_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn pane_border_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::pane_border_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// prefix key
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn prefix<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::prefix(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// prefix2 key
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn prefix2<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::prefix2(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// renumber-windows [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn renumber_windows<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::renumber_windows(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// repeat-time time
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn repeat_time<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::repeat_time(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.4:
    /// ```text
    /// set-remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    fn set_remain_on_exit<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::set_remain_on_exit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::set_titles(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// set-titles-string string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn set_titles_string<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::set_titles_string(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// silence-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn silence_action<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::silence_action(target));
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
    fn status<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_attr<S>(mut self, target: Option<S>) -> SelSelf {
        self.push(Getter::status_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// status-format[] format
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn status_format<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_format(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-interval interval
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_interval<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_interval(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-justify [left | centre | right]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_justify<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_justify(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_keys<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_keys(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_left(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_attr<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_left_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_left_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-left-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_left_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_left_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-left-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_left_length<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_left_length(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-left-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_left_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_left_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// status-position [top | bottom]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn status_position<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_position(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right string
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_right(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_attr<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_right_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_bg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_right_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// status-right-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn status_right_fg<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_right_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// status-right-length length
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn status_right_length<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_right_length(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-right-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_right_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_right_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn status_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// status-utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn status_utf8<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::status_utf8(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// terminal-overrides string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn terminal_overrides<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::terminal_overrides(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// update-environment[] variable
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn update_environment<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::update_environment(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6 v3.0:
    /// ```text
    /// user-keys
    /// ```
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    fn user_keys<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::user_keys(target));
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
    fn visual_activity<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::visual_activity(target));
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
    fn visual_bell<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::visual_bell(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// visual-content [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn visual_content<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::visual_content(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// visual-silence [on | off | both]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn visual_silence<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::visual_silence(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn word_separators<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Getter::word_separators(target));
        self
    }

    //fn into_commands<S>(self) -> TmuxCommands<'a> {
    //self.options
    //}
}
