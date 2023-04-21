use crate::{GetWindowOptionTr, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

pub trait GetWindowOptionsTr<'a, Getter: GetWindowOptionTr> {
    fn new() -> Self;

    fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T);

    fn into_commands(self) -> TmuxCommands<'a>;

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// aggressive-resize [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn aggressive_resize<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::aggressive_resize(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn allow_rename<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::allow_rename(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn alternate_screen<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::alternate_screen(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn automatic_rename<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::automatic_rename(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn automatic_rename_format<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::automatic_rename_format(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_interval<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::c0_change_interval(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_trigger<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::c0_change_trigger(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_colour<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::clock_mode_colour(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::clock_mode_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_height<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::force_height(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_width<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::force_width(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn layout_history_limit<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::layout_history_limit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_height<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::main_pane_height(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_width<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::main_pane_width(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::mode_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::mode_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::mode_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn mode_keys<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::mode_keys(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.1:
    /// ```text
    /// mode-mouse [on | off]
    /// ```
    ///
    /// tmux 1.6:
    /// ```text
    /// mode-mouse [on | off | copy-mode]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    fn mode_mouse<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::mode_mouse(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn mode_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::mode_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn monitor_activity<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::monitor_activity(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn monitor_content<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::monitor_content(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn monitor_bell<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::monitor_bell(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn monitor_silence<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::monitor_silence(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_height<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::other_pane_height(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_width<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::other_pane_width(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_active_border_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_active_border_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-active-border-bg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_active_border_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-active-border-fg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_active_border_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn pane_base_index<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_base_index(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-border-bg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_border_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_border_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-border-fg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_border_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_border_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_format<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_border_format(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_status<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_border_status(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_border_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::pane_border_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn remain_on_exit<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::remain_on_exit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn synchronize_panes<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::synchronize_panes(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn utf8<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::utf8(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_active_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_active_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_bell_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_bell_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_bell_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_content_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_content_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_content_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_activity_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-bg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_activity_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_activity_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_current_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_current_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_current_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_alert_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_alert_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_alert_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_activity_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_activity_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_bell_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_bell_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn window_status_content_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_content_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_current_format<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_current_format(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_attr<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_last_attr(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_bg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_last_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_fg<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_last_fg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_current_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_current_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_format<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_format(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_last_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_last_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn window_status_separator<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_separator(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_status_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn window_size<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_size(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn word_separators<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::word_separators(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_style<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::window_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn wrap_search<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::wrap_search(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn xterm_keys<T>(mut self, target: Option<T>) -> Self
    where
        Self: Sized,
        T: Into<Cow<'a, str>>,
    {
        self.push(Getter::xterm_keys(target));
        self
    }

    // XXX: user options?
    //pub user_options: Option<HashMap<String, String>>
}
