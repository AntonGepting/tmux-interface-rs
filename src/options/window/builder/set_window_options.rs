// NOTE: DRY, global and local window options structures have same methods therefore common setter
// and getter traits were choosen for common use super::set_window_option::Self::Setter;
//
// use crate::ModeMouse;
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
use crate::ModeMouse;
#[cfg(feature = "tmux_2_3")]
use crate::PaneBorderStatus;
#[cfg(feature = "tmux_2_9")]
use crate::WindowSize;
use crate::{ClockModeStyle, SetWindowOptionExt, StatusKeys, Switch, TmuxCommand, TmuxCommands};
use std::borrow::Cow;

pub trait SetWindowOptions<'a> {
    type Setter: SetWindowOptionExt;

    fn new() -> Self;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn push_cmds(&mut self, options: TmuxCommands<'a>);

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// aggressive-resize [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn aggressive_resize<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::aggressive_resize(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn allow_rename<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::allow_rename(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn alternate_screen<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::alternate_screen(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn automatic_rename<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::automatic_rename(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn automatic_rename_format<T, S>(mut self, target: Option<T>, format: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::automatic_rename_format(target, format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_interval<T>(mut self, target: Option<T>, c0_change_interval: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::c0_change_interval(target, c0_change_interval));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_trigger<S, T>(mut self, target: Option<T>, c0_change_trigger: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::c0_change_trigger(target, c0_change_trigger));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_colour<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::clock_mode_colour(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_style<T>(
        mut self,
        target: Option<T>,
        clock_mode_style: Option<ClockModeStyle>,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::clock_mode_style(target, clock_mode_style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_height<T>(mut self, target: Option<T>, height: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::force_height(target, height));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_width<T>(mut self, target: Option<T>, width: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::force_width(target, width));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn layout_history_limit<T>(mut self, target: Option<T>, limit: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::layout_history_limit(target, limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_height<T>(mut self, target: Option<T>, height: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::main_pane_height(target, height));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_width<T>(mut self, target: Option<T>, width: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::main_pane_width(target, width));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mode_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mode_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mode_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn mode_keys<T>(mut self, target: Option<T>, mode_keys: Option<StatusKeys>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mode_keys(target, mode_keys));
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
    fn mode_mouse<T>(mut self, target: Option<T>, mode_mouse: Option<ModeMouse>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mode_mouse(target, mode_mouse));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn mode_style<S, T>(mut self, target: Option<T>, mode_style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::mode_style(target, mode_style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn monitor_activity<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::monitor_activity(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn monitor_content<S, T>(mut self, target: Option<T>, match_string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::monitor_content(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn monitor_bell<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::monitor_bell(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn monitor_silence<T>(mut self, target: Option<T>, interval: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::monitor_silence(target, interval));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_height<T>(mut self, target: Option<T>, height: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::other_pane_height(target, height));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_width<T>(mut self, target: Option<T>, width: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::other_pane_width(target, width));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_active_border_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_active_border_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn pane_base_index<T>(mut self, target: Option<T>, index: Option<usize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_base_index(target, index));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_format<S, T>(mut self, target: Option<T>, format: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_format(target, format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_status<T>(
        mut self,
        target: Option<T>,
        pane_border_status: Option<PaneBorderStatus>,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_status(target, pane_border_status));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
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
    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn remain_on_exit<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::remain_on_exit(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn synchronize_panes<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::synchronize_panes(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn utf8<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::utf8(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_active_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_active_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_attr(
            target, attributes,
        ));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-bg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_bg(mut self) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_bg(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_fg<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_fg(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_alert_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_alert_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_alert_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_activity_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_bell_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn window_status_content_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_current_format<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_format(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_attr<S, T>(mut self, target: Option<T>, attributes: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_attr(target, attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_bg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_bg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_fg<S, T>(mut self, target: Option<T>, colour: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_fg(target, colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_current_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_format<S, T>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_format(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_last_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn window_status_separator<S, T>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_separator(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_status_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn window_size<T>(mut self, target: Option<T>, window_size: Option<WindowSize>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_size(target, window_size));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn word_separators<S, T>(mut self, target: Option<T>, string: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::word_separators(target, string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::window_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn wrap_search<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::wrap_search(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn xterm_keys<T>(mut self, target: Option<T>, switch: Option<Switch>) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(Self::Setter::xterm_keys(target, switch));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
