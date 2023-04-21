use crate::options::*;
#[cfg(feature = "tmux_2_9")]
use crate::WindowSize;
use crate::{StatusKeys, TmuxCommand};
use std::borrow::Cow;

// TODO: all options exist in get/set?
// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
pub trait SetWindowOptionTr: SetOptionTr + SetUserOption {
    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// activity-action [any | none | current | other]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn activity_action<'a, S>(target: Option<S>, activity: Option<Activity>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, ACTIVITY_ACTION, activity.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// aggressive-resize [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn aggressive_resize<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, AGGRESSIVE_RESIZE, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn allow_rename<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, ALLOW_RENAME, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn alternate_screen<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, ALTERNATE_SCREEN, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn automatic_rename<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, AUTOMATIC_RENAME, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn automatic_rename_format<'a, S, T>(target: Option<S>, format: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, AUTOMATIC_RENAME_FORMAT, format)
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_interval<'a, S>(target: Option<S>, interval: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, C0_CHANGE_INTERVAL, interval.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_trigger<'a, S>(target: Option<S>, trigger: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, C0_CHANGE_TRIGGER, trigger.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_colour<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, CLOCK_MODE_COLOUR, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_style<'a, S>(
        target: Option<S>,
        clock_mode_style: Option<ClockModeStyle>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(
            target,
            CLOCK_MODE_STYLE,
            clock_mode_style.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_height<'a, S>(target: Option<S>, height: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, FORCE_HEIGHT, height.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_width<'a, S>(target: Option<S>, width: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, FORCE_WIDTH, width.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn layout_history_limit<'a, S>(target: Option<S>, limit: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, LAYOUT_HISTORY_LIMIT, limit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_height<'a, S>(target: Option<S>, height: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MAIN_PANE_HEIGHT, height.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_width<'a, S>(target: Option<S>, width: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MAIN_PANE_WIDTH, width.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_attr<'a, S, T>(target: Option<S>, attributes: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MODE_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MODE_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MODE_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn mode_keys<'a, S>(target: Option<S>, mode_keys: Option<StatusKeys>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MODE_KEYS, mode_keys.map(|s| s.to_string()))
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
    fn mode_mouse<'a, S>(target: Option<S>, mode_mouse: Option<ModeMouse>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MODE_MOUSE, mode_mouse.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn mode_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MODE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn monitor_activity<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MONITOR_ACTIVITY, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn monitor_content<'a, S, T>(target: Option<S>, match_string: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MONITOR_CONTENT, match_string)
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn monitor_bell<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MONITOR_BELL, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn monitor_silence<'a, S>(target: Option<S>, interval: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, MONITOR_SILENCE, interval.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_height<'a, S>(target: Option<S>, height: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, OTHER_PANE_HEIGHT, height.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_width<'a, S>(target: Option<S>, width: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, OTHER_PANE_WIDTH, width.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_active_border_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_ACTIVE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-active-border-bg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_active_border_bg<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_ACTIVE_BORDER_BG, style)
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-active-border-fg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_active_border_fg<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_ACTIVE_BORDER_FG, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn pane_base_index<'a, S>(target: Option<S>, index: Option<usize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_BASE_INDEX, index.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-border-bg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_border_bg<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_BORDER_BG, style)
    }

    /// ### Manual
    ///
    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-border-fg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    fn pane_border_fg<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_BORDER_FG, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_format<'a, S, T>(target: Option<S>, format: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_BORDER_FORMAT, format)
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_status<'a, S>(
        target: Option<S>,
        pane_border_status: Option<PaneBorderStatus>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(
            target,
            PANE_BORDER_STATUS,
            pane_border_status.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_border_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, PANE_BORDER_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn remain_on_exit<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, REMAIN_ON_EXIT, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn synchronize_panes<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, SYNCHRONIZE_PANES, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn utf8<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, UTF8, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_active_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_ACTIVE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_attr<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_BELL_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_BELL_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_BELL_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_attr<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CONTENT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CONTENT_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CONTENT_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_attr<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ACTIVITY_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-bg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_bg<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ACTIVITY_BG, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_fg<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ACTIVITY_FG, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_attr<'a, S, T>(target: Option<S>, attributes: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_attr<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CURRENT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CURRENT_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CURRENT_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_attr<'a, S, T>(
        target: Option<S>,
        attributes: Option<S>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ALERT_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ALEERT_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ALERT_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_activity_style<'a, S, T>(
        target: Option<T>,
        style: Option<S>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_ACTIVITY_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_bell_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_BELL_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn window_status_content_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CONTENT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_current_format<'a, S, T>(
        target: Option<T>,
        string: Option<S>,
    ) -> TmuxCommand<'a>
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CURRENT_FORMAT, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_attr<'a, S, T>(
        target: Option<S>,
        attributes: Option<T>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_LAST_ATTR, attributes)
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_bg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_LAST_BG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_fg<'a, S, T>(target: Option<S>, colour: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_LAST_FG, colour)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_current_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_CURRENT_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_format<'a, S, T>(target: Option<S>, string: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_FORMAT, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_last_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_LAST_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn window_status_separator<'a, S, T>(target: Option<S>, string: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_SEPARATOR, string)
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STATUS_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn window_size<'a, S>(target: Option<S>, window_size: Option<WindowSize>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_SIZE, window_size.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn word_separators<'a, S, T>(target: Option<S>, string: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WORD_SEPARATORS, string)
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WINDOW_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn wrap_search<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, WRAP_SEARCH, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn xterm_keys<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set_ext(target, XTERM_KEYS, switch.map(|s| s.to_string()))
    }
}
