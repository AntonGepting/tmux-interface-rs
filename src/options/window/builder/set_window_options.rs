// NOTE: DRY, global and local window options structures have same methods therefore common setter
// and getter traits were choosen for common use super::set_window_option::Self::Setter;
//
#[cfg(feature = "tmux_2_9")]
use crate::WindowSize;
use crate::{
    ClockModeStyle, PaneBorderStatus, SetGlobalWindowOption, SetLocalWindowOption,
    SetWindowOptionExt, StatusKeys, Switch, TmuxCommand, TmuxCommands,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct SetLocalWindowOptions<'a> {
    pub options: TmuxCommands<'a>,
}

#[derive(Debug)]
pub struct SetGlobalWindowOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetWindowOptions<'a> for SetLocalWindowOptions<'a> {
    type Setter = SetLocalWindowOption;

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

impl<'a> SetWindowOptions<'a> for SetGlobalWindowOptions<'a> {
    type Setter = SetGlobalWindowOption;

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

//impl<'a> SetGlobalWindowOptions<'a> {}

//impl<'a> SetLocalWindowOptions<'a> {
//pub fn new() -> Self{
//Self {
//options: TmuxCommands::new(),
//}
//}

////pub fn push(&mut self, option: TmuxCommand<'a>) {
////self.push(option);
////}
//}

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
    fn aggressive_resize(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::aggressive_resize(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn allow_rename(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::allow_rename(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn alternate_screen(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::alternate_screen(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn automatic_rename(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::automatic_rename(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn automatic_rename_format(mut self, format: Option<String>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::automatic_rename_format(format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_interval(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::c0_change_interval());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_trigger<S: Into<Cow<'a, str>>>(mut self, trigger: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::c0_change_trigger());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_colour<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::clock_mode_colour(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_style(mut self, clock_mode_style: Option<ClockModeStyle>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::clock_mode_style(clock_mode_style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_height(mut self, height: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::force_height(height));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_width(mut self, width: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::force_width(width));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn layout_history_limit(mut self, limit: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::layout_history_limit(limit));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_height(mut self, height: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::main_pane_height(height));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_width(mut self, width: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::main_pane_width(width));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_attr<'a, S: Cow<'a, str>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mode_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_bg<'a, S: Cow<'a, str>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mode_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_fg<'a, S: Cow<'a, str>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mode_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn mode_keys(mut self, mode_keys: Option<StatusKeys>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mode_keys(mode_keys));
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
    fn mode_mouse(mut self, mode_mouse: Option<ModeMouse>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mode_mouse(mode_mouse));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn mode_style<S: Into<Cow<'a, str>>>(mut self, mode_style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::mode_style(mode_style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn monitor_activity(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::monitor_activity(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn monitor_content<S: Into<Cow<'a, str>>>(mut self, match_string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::monitor_content());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn monitor_bell(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::monitor_bell(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn monitor_silence(mut self, interval: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::monitor_silence(interval));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_height(mut self, height: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::other_pane_height(height));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_width(mut self, width: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::other_pane_width(width));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_active_border_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_active_border_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn pane_base_index(mut self, index: Option<usize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_base_index(index));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_format<S: Into<Cow<'a, str>>>(mut self, format: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_format(format));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_status(mut self, pane_border_status: Option<PaneBorderStatus>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_status(pane_border_status));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_border_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::pane_border_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn remain_on_exit(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::remain_on_exit(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn synchronize_panes(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::synchronize_panes(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn utf8(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::utf8(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_active_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_active_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_attr(attributes));
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
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_fg<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_fg(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_alert_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_alert_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_alert_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_activity_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_activity_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_bell_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_bell_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn window_status_content_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_content_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_current_format<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_format(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_attr<S: Into<Cow<'a, str>>>(mut self, attributes: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_attr(attributes));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_bg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_bg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_fg<S: Into<Cow<'a, str>>>(mut self, colour: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_fg(colour));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_current_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_current_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_format<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_format(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_last_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_last_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn window_status_separator<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_separator(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_status_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn window_size(mut self, window_size: Option<WindowSize>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_size(window_size));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn word_separators<S: Into<Cow<'a, str>>>(mut self, string: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::word_separators(string));
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::window_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn wrap_search(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::wrap_search(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn xterm_keys(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(Self::Setter::xterm_keys(switch));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
