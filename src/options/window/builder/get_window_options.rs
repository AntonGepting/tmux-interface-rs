use crate::options::window::get_window_option::{
    GetGlobalWindowOption, GetLocalWindowOption, GetWindowOption,
};
use crate::{TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct GetGlobalWindowOptions<'a> {
    pub options: TmuxCommands<'a>,
}

#[derive(Debug)]
pub struct GetLocalWindowOptions<'a> {
    pub options: TmuxCommands<'a>,
}

// XXX: both are same, optimize
impl<'a> GetWindowOptions<'a, GetLocalWindowOption> for GetLocalWindowOptions<'a> {
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
impl<'a> GetWindowOptions<'a, GetGlobalWindowOption> for GetGlobalWindowOptions<'a> {
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

pub trait GetWindowOptions<'a, Getter: GetWindowOption> {
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
    fn aggressive_resize(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::aggressive_resize());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    fn allow_rename(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::allow_rename());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    fn alternate_screen(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::alternate_screen());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    fn automatic_rename(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::automatic_rename());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn automatic_rename_format(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::automatic_rename_format());
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
        self.push(Getter::c0_change_interval());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    fn c0_change_trigger(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::c0_change_trigger());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_colour(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::clock_mode_colour());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn clock_mode_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::clock_mode_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_height(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::force_height());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    fn force_width(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::force_width());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    fn layout_history_limit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::layout_history_limit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_height(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::main_pane_height());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn main_pane_width(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::main_pane_width());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mode_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mode_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn mode_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mode_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn mode_keys(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mode_keys());
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
    fn mode_mouse(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mode_mouse());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn mode_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::mode_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn monitor_activity(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::monitor_activity());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    fn monitor_content(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::monitor_content());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    fn monitor_bell(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::monitor_bell());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn monitor_silence(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::monitor_silence());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_height(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::other_pane_height());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    fn other_pane_width(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::other_pane_width());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_active_border_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_active_border_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    fn pane_base_index(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_base_index());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_format(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_border_format());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    fn pane_border_status(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_border_status());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    fn pane_border_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::pane_border_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    fn remain_on_exit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::remain_on_exit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    fn synchronize_panes(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::synchronize_panes());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    fn utf8(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::utf8());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_active_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_active_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_bell_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_bell_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_bell_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_bell_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_content_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_content_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_content_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_content_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_activity_attr());
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
        self.push(Getter::window_status_activity_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    fn window_status_activity_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_activity_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_current_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_current_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    fn window_status_current_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_current_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_alert_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_alert_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    fn window_status_alert_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_alert_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_activity_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_activity_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_bell_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_bell_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    fn window_status_content_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_content_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_current_format(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_current_format());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_attr(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_last_attr());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_bg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_last_bg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    fn window_status_last_fg(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_last_fg());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_current_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_current_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    fn window_status_format(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_format());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_last_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_last_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn window_status_separator(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_separator());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    fn window_status_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_status_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    fn window_size(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_size());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    fn word_separators(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::word_separators());
        self
    }

    /// ### Manual
    ///
    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    fn window_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::window_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    fn wrap_search(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::wrap_search());
        self
    }

    /// ### Manual
    ///
    /// tmux ^1.0:
    /// ```text
    /// xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    fn xterm_keys(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(Getter::xterm_keys());
        self
    }

    // XXX: user options?
    //pub user_options: Option<HashMap<String, String>>
}
