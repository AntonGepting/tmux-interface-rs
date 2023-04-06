use crate::options::*;
use crate::TmuxCommand;
use std::borrow::Cow;

// NOTE: ADR: compile time or run time parametrisation for global local option set/get
// * compile time: trais
// * runtime: struct field with user given setter/getter

// TODO: all options exist in get/set?
// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
// default implementation for getting options, by default local options
pub trait GetWindowOption: GetOptionExt {
    fn all<'a, S: Into<Cow<'a, str>>>(target: Option<S>) -> TmuxCommand<'a> {
        Self::get_all(target)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// aggressive-resize [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn aggressive_resize<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, AGGRESSIVE_RESIZE)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn allow_rename<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, ALLOW_RENAME)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn alternate_screen<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, ALTERNATE_SCREEN)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn automatic_rename<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, AUTOMATIC_RENAME)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn automatic_rename_format<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, AUTOMATIC_RENAME_FORMAT)
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_interval<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, C0_CHANGE_INTERVAL)
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_trigger<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, C0_CHANGE_TRIGGER)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_colour<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, CLOCK_MODE_COLOUR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, CLOCK_MODE_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_height<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, FORCE_HEIGHT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_width<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, FORCE_WIDTH)
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn layout_history_limit<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, LAYOUT_HISTORY_LIMIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_height<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MAIN_PANE_HEIGHT)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_width<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MAIN_PANE_WIDTH)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, MODE_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, MODE_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, MODE_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn mode_keys<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MODE_KEYS)
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
    fn mode_mouse<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, MODE_MOUSE)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn mode_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MODE_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn monitor_activity<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MONITOR_ACTIVITY)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn monitor_content<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, MONITOR_CONTENT)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn monitor_bell<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MONITOR_BELL)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn monitor_silence<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, MONITOR_SILENCE)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_height<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, OTHER_PANE_HEIGHT)
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_width<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, OTHER_PANE_WIDTH)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_active_border_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, PANE_ACTIVE_BORDER_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn pane_base_index<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, PANE_BASE_INDEX)
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_format<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, PANE_BORDER_FORMAT)
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_status<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, PANE_BORDER_STATUS)
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_border_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, PANE_BORDER_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn remain_on_exit<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, REMAIN_ON_EXIT)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn synchronize_panes<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, SYNCHRONIZE_PANES)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn utf8<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, UTF8)
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_active_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_ACTIVE_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_BELL_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_BELL_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_BELL_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CONTENT_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CONTENT_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CONTENT_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ACTIVITY_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-bg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ACTIVITY_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ACTIVITY_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CURRENT_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CURRENT_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CURRENT_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ALERT_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ALERT_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_ALERT_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_activity_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_ACTIVITY_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_bell_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_BELL_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn window_status_content_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_CONTENT_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_current_format<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_CURRENT_FORMAT)
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_attr<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_LAST_ATTR)
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_bg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_LAST_BG)
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_fg<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STATUS_LAST_FG)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_current_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_CURRENT_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_format<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_FORMAT)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_last_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_LAST_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn window_status_separator<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_SEPARATOR)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STATUS_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn window_size<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_SIZE)
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn word_separators<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WORD_SEPARATORS)
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_style<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get(target, WINDOW_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn wrap_search<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WRAP_SEARCH)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn xterm_keys<'a, T>(target: Option<T>) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, XTERM_KEYS)
    }
}
