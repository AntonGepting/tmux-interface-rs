use super::*;
use crate::options::common::{cow_parse, get_parts, option_to_string};
use crate::options::StatusKeys;
use crate::Error;
use crate::Switch;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
use crate::ModeMouse;

// TODO: check types
#[derive(PartialEq, Default, Clone, Debug)]
pub struct WindowOptions<'a> {
    /// tmux ^1.0:
    /// ```text
    /// aggressive-resize [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub aggressive_resize: Option<Switch>,

    /// tmux ^1.6 v3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub allow_rename: Option<Switch>,

    /// tmux ^1.2 v3.0:
    /// ```text
    // alternate-screen [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub alternate_screen: Option<Switch>,

    /// tmux ^1.0:
    /// ```text
    /// automatic-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")] // 0.8
    pub automatic_rename: Option<Switch>,

    /// tmux ^1.9:
    /// ```text
    /// automatic-rename-format format
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub automatic_rename_format: Option<Cow<'a, str>>,

    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-interval interval
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_interval: Option<usize>,

    /// tmux ^1.7 v2.1:
    /// ```text
    /// c0-change-trigger trigger
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_trigger: Option<usize>,

    /// tmux ^1.0:
    /// ```text
    /// clock-mode-colour colour
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_colour: Option<Cow<'a, str>>,

    /// tmux ^1.0:
    /// ```text
    /// clock-mode-style [12 | 24]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_style: Option<ClockModeStyle>,

    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-height height
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_height: Option<usize>,

    /// tmux ^1.0 v2.9:
    /// ```text
    /// force-width width
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_width: Option<usize>,

    /// tmux ^1.7 v1.8:
    /// ```text
    /// layout-history-limit limit
    /// ```
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub layout_history_limit: Option<usize>,

    /// tmux ^1.0:
    /// ```text
    /// main-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_height: Option<usize>,

    /// tmux ^1.0:
    /// ```text
    /// main-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_width: Option<usize>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_attr: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_bg: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// mode-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_fg: Option<Cow<'a, str>>,

    /// tmux ^1.0:
    /// ```text
    /// mode-keys [vi | emacs]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub mode_keys: Option<StatusKeys>,

    /// tmux 1.6:
    /// ```text
    /// mode-mouse [on | off | copy-mode]
    /// ```
    /// tmux ^1.0 v2.1:
    /// ```text
    /// mode-mouse [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub mode_mouse: Option<ModeMouse>,

    /// tmux ^1.9:
    /// ```text
    ///mode-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub mode_style: Option<Cow<'a, str>>,

    /// tmux ^1.0:
    /// ```text
    /// monitor-activity [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub monitor_activity: Option<Switch>,

    /// tmux ^1.0 v2.0:
    /// ```text
    /// monitor-content match-string
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub monitor_content: Option<Cow<'a, str>>,

    /// tmux ^2.6:
    /// ```text
    /// monitor-bell [on | off]
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub monitor_bell: Option<Switch>,

    /// tmux ^1.4:
    /// ```text
    /// monitor-silence [interval]
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub monitor_silence: Option<usize>,

    /// tmux ^1.4:
    /// ```text
    /// other-pane-height height
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_height: Option<usize>,

    /// tmux ^1.4:
    /// ```text
    /// other-pane-width width
    /// ```
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_width: Option<usize>,

    /// tmux ^1.9:
    /// ```text
    /// pane-active-border-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub pane_active_border_style: Option<Cow<'a, str>>,

    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-active-border-bg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub pane_active_border_bg: Option<Cow<'a, str>>,

    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-active-border-fg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub pane_active_border_fg: Option<Cow<'a, str>>,

    /// tmux ^1.6:
    /// ```text
    /// pane-base-index index
    /// ```
    #[cfg(feature = "tmux_1_6")]
    pub pane_base_index: Option<usize>,

    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-border-bg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub pane_border_bg: Option<Cow<'a, str>>,

    /// tmux ^0.8 v1.9:
    /// ```text
    /// pane-border-fg style
    /// ```
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub pane_border_fg: Option<Cow<'a, str>>,

    /// tmux ^2.3:
    /// ```text
    /// pane-border-format format
    /// ```
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_format: Option<Cow<'a, str>>,

    /// tmux ^2.3:
    /// ```text
    /// pane-border-status [off | top | bottom]
    /// ```
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_status: Option<PaneBorderStatus>,

    /// tmux ^2.0:
    /// ```text
    /// pane-border-style style
    /// ```
    #[cfg(feature = "tmux_2_0")]
    pub pane_border_style: Option<Cow<'a, str>>,

    /// tmux ^1.0 v3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub remain_on_exit: Option<Switch>,

    /// tmux ^1.2 v3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    pub synchronize_panes: Option<Switch>,

    /// tmux ^1.0 v2.2:
    /// ```text
    /// utf8 [on | off]
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub utf8: Option<Switch>,

    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_active_style: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_attr: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_bg: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-bell-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_fg: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_attr: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_bg: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-content-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_fg: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_attr: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-bg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_bg: Option<Cow<'a, str>>,

    /// tmux ^1.6 v1.9:
    /// ```text
    /// window-status-activity-fg attributes
    /// ```
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_fg: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_attr: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_bg: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_fg: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_attr: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_bg: Option<Cow<'a, str>>,

    /// tmux ^1.0 v1.9:
    /// ```text
    /// window-status-current-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_fg: Option<Cow<'a, str>>,

    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_attr: Option<Cow<'a, str>>,

    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_bg: Option<Cow<'a, str>>,

    /// tmux ^1.3 v1.6:
    /// ```text
    /// window-status-alert-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_fg: Option<Cow<'a, str>>,

    /// tmux ^1.9:
    /// ```text
    /// window-status-activity-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub window_status_activity_style: Option<Cow<'a, str>>,

    /// tmux ^1.9:
    /// ```text
    /// window-status-bell-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub window_status_bell_style: Option<Cow<'a, str>>,

    /// tmux ^1.9 v2.0:
    /// ```text
    /// window-status-content-style style
    /// ```
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub window_status_content_style: Option<Cow<'a, str>>,

    /// tmux ^1.2:
    /// ```text
    /// window-status-current-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub window_status_current_format: Option<Cow<'a, str>>,

    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-attr attributes
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_attr: Option<Cow<'a, str>>,

    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-bg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_bg: Option<Cow<'a, str>>,

    /// tmux ^1.8 v1.9:
    /// ```text
    /// window-status-last-fg colour
    /// ```
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_fg: Option<Cow<'a, str>>,

    /// tmux ^1.9:
    /// ```text
    /// window-status-current-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub window_status_current_style: Option<Cow<'a, str>>,

    /// tmux ^1.2:
    /// ```text
    /// window-status-format string
    /// ```
    #[cfg(feature = "tmux_1_2")]
    pub window_status_format: Option<Cow<'a, str>>,

    /// tmux ^1.9:
    /// ```text
    /// window-status-last-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub window_status_last_style: Option<Cow<'a, str>>,

    /// tmux ^1.7:
    /// ```text
    /// window-status-separator string
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub window_status_separator: Option<Cow<'a, str>>,

    /// tmux ^1.9:
    /// ```text
    /// window-status-style style
    /// ```
    #[cfg(feature = "tmux_1_9")]
    pub window_status_style: Option<Cow<'a, str>>,

    /// tmux ^2.9:
    /// ```text
    /// window-size largest | smallest | manual | latest
    /// ```
    #[cfg(feature = "tmux_2_9")]
    pub window_size: Option<WindowSize>,

    /// tmux ^1.2 v1.6:
    /// ```text
    /// word-separators string
    /// ```
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub word_separators: Option<Cow<'a, str>>,

    /// tmux ^2.1 v3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_style: Option<Cow<'a, str>>,

    /// tmux ^1.7:
    /// ```text
    /// wrap-search [on | off]
    /// ```
    #[cfg(feature = "tmux_1_7")]
    pub wrap_search: Option<Switch>,

    /// tmux ^1.0:
    /// ```text
    ///  xterm-keys [on | off]
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub xterm_keys: Option<Switch>,

    /// tmux ^1.0:
    /// ```text
    /// @user-option-name value
    /// ```
    #[cfg(feature = "tmux_1_0")]
    pub user_options: HashMap<String, Option<Cow<'a, str>>>,
}

// TODO: check default values from tmux sources

impl<'a> fmt::Display for WindowOptions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = Vec::new();
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, AGGRESSIVE_RESIZE, &self.aggressive_resize);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, ALLOW_RENAME, &self.allow_rename);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, ALTERNATE_SCREEN, &self.alternate_screen);
        #[cfg(feature = "tmux_1_0")] // 0.8
        option_to_string(&mut v, AUTOMATIC_RENAME, &self.automatic_rename);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            AUTOMATIC_RENAME_FORMAT,
            &self.automatic_rename_format,
        );
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, C0_CHANGE_INTERVAL, &self.c0_change_interval);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, C0_CHANGE_TRIGGER, &self.c0_change_trigger);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, CLOCK_MODE_COLOUR, &self.clock_mode_colour);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, CLOCK_MODE_STYLE, &self.clock_mode_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        option_to_string(&mut v, FORCE_HEIGHT, &self.force_height);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        option_to_string(&mut v, FORCE_WIDTH, &self.force_width);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
        option_to_string(&mut v, LAYOUT_HISTORY_LIMIT, &self.layout_history_limit);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MAIN_PANE_HEIGHT, &self.main_pane_height);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MAIN_PANE_WIDTH, &self.main_pane_width);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MODE_ATTR, &self.mode_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MODE_BG, &self.mode_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, MODE_FG, &self.mode_fg);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MODE_KEYS, &self.mode_keys);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        option_to_string(&mut v, MODE_MOUSE, &self.mode_mouse);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, MODE_STYLE, &self.mode_style);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, MONITOR_ACTIVITY, &self.monitor_activity);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        option_to_string(&mut v, MONITOR_CONTENT, &self.monitor_content);
        #[cfg(feature = "tmux_2_6")]
        option_to_string(&mut v, MONITOR_BELL, &self.monitor_bell);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, MONITOR_SILENCE, &self.monitor_silence);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, OTHER_PANE_HEIGHT, &self.other_pane_height);
        #[cfg(feature = "tmux_1_4")]
        option_to_string(&mut v, OTHER_PANE_WIDTH, &self.other_pane_width);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            PANE_ACTIVE_BORDER_STYLE,
            &self.pane_active_border_style,
        );
        #[cfg(feature = "tmux_1_6")]
        option_to_string(&mut v, PANE_BASE_INDEX, &self.pane_base_index);
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, PANE_BORDER_BG, &self.pane_border_bg);
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, PANE_BORDER_FG, &self.pane_border_fg);
        #[cfg(feature = "tmux_2_3")]
        option_to_string(&mut v, PANE_BORDER_FORMAT, &self.pane_border_format);
        #[cfg(feature = "tmux_2_3")]
        option_to_string(&mut v, PANE_BORDER_STATUS, &self.pane_border_status);
        #[cfg(feature = "tmux_2_0")]
        option_to_string(&mut v, PANE_BORDER_STYLE, &self.pane_border_style);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, REMAIN_ON_EXIT, &self.remain_on_exit);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
        option_to_string(&mut v, SYNCHRONIZE_PANES, &self.synchronize_panes);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        option_to_string(&mut v, UTF8, &self.utf8);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, WINDOW_ACTIVE_STYLE, &self.window_active_style);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_BELL_ATTR,
            &self.window_status_bell_attr,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_BELL_BG, &self.window_status_bell_bg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_BELL_FG, &self.window_status_bell_fg);
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_ATTR,
            &self.window_status_content_attr,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_BG,
            &self.window_status_content_bg,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_FG,
            &self.window_status_content_fg,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_ATTR,
            &self.window_status_activity_attr,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_BG,
            &self.window_status_activity_bg,
        );
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_FG,
            &self.window_status_activity_fg,
        );
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_ATTR, &self.window_status_attr);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_BG, &self.window_status_bg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_FG, &self.window_status_fg);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_ATTR,
            &self.window_status_current_attr,
        );
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_BG,
            &self.window_status_current_bg,
        );
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_FG,
            &self.window_status_current_fg,
        );
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ALERT_ATTR,
            &self.window_status_alert_attr,
        );
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        option_to_string(&mut v, WINDOW_STATUS_ALERT_BG, &self.window_status_alert_bg);
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        option_to_string(&mut v, WINDOW_STATUS_ALERT_FG, &self.window_status_alert_fg);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_ACTIVITY_STYLE,
            &self.window_status_activity_style,
        );
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_BELL_STYLE,
            &self.window_status_bell_style,
        );
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CONTENT_STYLE,
            &self.window_status_content_style,
        );
        #[cfg(feature = "tmux_1_2")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_FORMAT,
            &self.window_status_current_format,
        );
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        option_to_string(
            &mut v,
            WINDOW_STATUS_LAST_ATTR,
            &self.window_status_last_attr,
        );
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_LAST_BG, &self.window_status_last_bg);
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        option_to_string(&mut v, WINDOW_STATUS_LAST_FG, &self.window_status_last_fg);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_CURRENT_STYLE,
            &self.window_status_current_style,
        );
        #[cfg(feature = "tmux_1_2")]
        option_to_string(&mut v, WINDOW_STATUS_FORMAT, &self.window_status_format);
        #[cfg(feature = "tmux_1_9")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_LAST_STYLE,
            &self.window_status_last_style,
        );
        #[cfg(feature = "tmux_1_7")]
        option_to_string(
            &mut v,
            WINDOW_STATUS_SEPARATOR,
            &self.window_status_separator,
        );
        #[cfg(feature = "tmux_1_9")]
        option_to_string(&mut v, WINDOW_STATUS_STYLE, &self.window_status_style);
        #[cfg(feature = "tmux_2_9")]
        option_to_string(&mut v, WINDOW_SIZE, &self.window_size);
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
        option_to_string(&mut v, WORD_SEPARATORS, &self.word_separators);
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        option_to_string(&mut v, WINDOW_STYLE, &self.window_style);
        #[cfg(feature = "tmux_1_7")]
        option_to_string(&mut v, WRAP_SEARCH, &self.wrap_search);
        #[cfg(feature = "tmux_1_0")]
        option_to_string(&mut v, XTERM_KEYS, &self.xterm_keys);
        // option_to_string(&mut v, USER_OPTIONS, &self.user_options);
        let s = v.join("\n");
        write!(f, "{}", s)
    }
}
impl<'a> WindowOptions<'a> {
    ///
    /// ```text
    /// tmux show-options -g -w
    /// ```
    pub fn new() -> Self {
        let options = WindowOptions::default();

        #[cfg(feature = "tmux_1_0")]
        let options = options.aggressive_resize(Some(AGGRESSIVE_RESIZE_DEFAULT));
        // allow-passthrough off
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
        let options = options.allow_rename(Some(ALLOW_RENAME_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        let options = options.alternate_screen(Some(ALTERNATE_SCREEN_DEFAULT));
        #[cfg(feature = "tmux_1_0")] // 0.8
        let options = options.automatic_rename(Some(AUTOMATIC_RENAME_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.automatic_rename_format(Some(AUTOMATIC_RENAME_FORMAT_DEFAULT));
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let options = options.c0_change_interval(Some(C0_CHANGE_INTERVAL_DEFAULT));
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let options = options.c0_change_trigger(Some(C0_CHANGE_TRIGGER_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.clock_mode_colour(Some(CLOCK_MODE_COLOUR_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.clock_mode_style(Some(CLOCK_MODE_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let options = options.force_height(Some(FORCE_HEIGHT_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let options = options.force_width(Some(FORCE_WIDTH_DEFAULT));
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
        let options = options.layout_history_limit(Some(LAYOUT_HISTORY_LIMIT_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.main_pane_height(Some(MAIN_PANE_HEIGHT_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.main_pane_width(Some(MAIN_PANE_WIDTH_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_attr(Some(MODE_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_bg(Some(MODE_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_fg(Some(MODE_FG_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.mode_keys(Some(MODE_KEYS_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let options = options.mode_mouse(Some(MODE_MOUSE_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.mode_style(Some(MODE_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.monitor_activity(Some(MONITOR_ACTIVITY_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.monitor_content(Some(MONITOR_CONTENT_DEFAULT));
        #[cfg(feature = "tmux_2_6")]
        let options = options.monitor_bell(Some(MONITOR_BELL_DEFAULT));
        #[cfg(feature = "tmux_1_4")]
        let options = options.monitor_silence(Some(MONITOR_SILENCE_DEFAULT));
        #[cfg(feature = "tmux_1_4")]
        let options = options.other_pane_height(Some(OTHER_PANE_HEIGHT_DEFAULT));
        #[cfg(feature = "tmux_1_4")]
        let options = options.other_pane_width(Some(OTHER_PANE_WIDTH_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.pane_active_border_style(Some(PANE_ACTIVE_BORDER_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        let options = options.pane_active_border_bg(Some(PANE_ACTIVE_BORDER_BG_DEFAULT));
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        let options = options.pane_active_border_fg(Some(PANE_ACTIVE_BORDER_FG_DEFAULT));
        #[cfg(feature = "tmux_1_6")]
        let options = options.pane_base_index(Some(PANE_BASE_INDEX_DEFAULT));
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        let options = options.pane_border_bg(Some(PANE_BORDER_BG_DEFAULT));
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
        let options = options.pane_border_fg(Some(PANE_BORDER_FG_DEFAULT));
        #[cfg(feature = "tmux_2_3")]
        let options = options.pane_border_format(Some(PANE_BORDER_FORMAT_DEFAULT));
        #[cfg(feature = "tmux_2_3")]
        let options = options.pane_border_status(Some(PANE_BORDER_STATUS_DEFAULT));
        #[cfg(feature = "tmux_2_0")]
        let options = options.pane_border_style(Some(PANE_BORDER_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
        let options = options.remain_on_exit(Some(REMAIN_ON_EXIT_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
        let options = options.synchronize_panes(Some(SYNCHRONIZE_PANES_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let options = options.utf8(Some(UTF8_DEFAULT));
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let options = options.window_active_style(Some(WINDOW_ACTIVE_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_attr(Some(WINDOW_STATUS_BELL_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_bg(Some(WINDOW_STATUS_BELL_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_fg(Some(WINDOW_STATUS_BELL_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_attr(Some(WINDOW_STATUS_CONTENT_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_bg(Some(WINDOW_STATUS_CONTENT_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_fg(Some(WINDOW_STATUS_CONTENT_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options =
            options.window_status_activity_attr(Some(WINDOW_STATUS_ACTIVITY_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_bg(Some(WINDOW_STATUS_ACTIVITY_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_fg(Some(WINDOW_STATUS_ACTIVITY_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_attr(Some(WINDOW_STATUS_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_bg(Some(WINDOW_STATUS_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_fg(Some(WINDOW_STATUS_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_attr(Some(WINDOW_STATUS_CURRENT_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_bg(Some(WINDOW_STATUS_CURRENT_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_fg(Some(WINDOW_STATUS_CURRENT_FG_DEFAULT));
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_attr(Some(WINDOW_STATUS_ALERT_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_bg(Some(WINDOW_STATUS_ALERT_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_fg(Some(WINDOW_STATUS_ALERT_FG_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options =
            options.window_status_activity_style(Some(WINDOW_STATUS_ACTIVITY_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_bell_style(Some(WINDOW_STATUS_BELL_STYLE_DEFAULT));
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options =
            options.window_status_content_style(Some(WINDOW_STATUS_CONTENT_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_2")]
        let options =
            options.window_status_current_format(Some(WINDOW_STATUS_CURRENT_FORMAT_DEFAULT));
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_attr(Some(WINDOW_STATUS_LAST_ATTR_DEFAULT));
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_bg(Some(WINDOW_STATUS_LAST_BG_DEFAULT));
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_fg(Some(WINDOW_STATUS_LAST_FG_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options =
            options.window_status_current_style(Some(WINDOW_STATUS_CURRENT_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_2")]
        let options = options.window_status_format(Some(WINDOW_STATUS_FORMAT_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_last_style(Some(WINDOW_STATUS_LAST_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_7")]
        let options = options.window_status_separator(Some(WINDOW_STATUS_SEPARATOR_DEFAULT));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_style(Some(WINDOW_STATUS_STYLE_DEFAULT));
        #[cfg(feature = "tmux_2_9")]
        let options = options.window_size(Some(WINDOW_SIZE_DEFAULT));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
        let options = options.word_separators(Some(WORD_SEPARATORS_DEFAULT));
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let options = options.window_style(Some(WINDOW_STYLE_DEFAULT));
        #[cfg(feature = "tmux_1_7")]
        let options = options.wrap_search(Some(WRAP_SEARCH_DEFAULT));
        #[cfg(feature = "tmux_1_0")]
        let options = options.xterm_keys(Some(XTERM_KEYS_DEFAULT));
        options
    }

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
//impl FromStr for WindowOptions {
//type Err = Error;

impl<'a> WindowOptions<'a> {
    #[cfg(feature = "tmux_1_0")]
    pub fn aggressive_resize(mut self, aggressive_resize: Option<Switch>) -> Self {
        self.aggressive_resize = aggressive_resize;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub fn allow_rename(mut self, allow_rename: Option<Switch>) -> Self {
        self.allow_rename = allow_rename;
        self
    }
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub fn alternate_screen(mut self, alternate_screen: Option<Switch>) -> Self {
        self.alternate_screen = alternate_screen;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn automatic_rename(mut self, automatic_rename: Option<Switch>) -> Self {
        self.automatic_rename = automatic_rename;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn automatic_rename_format<S: Into<Cow<'a, str>>>(
        mut self,
        automatic_rename_format: Option<S>,
    ) -> Self {
        self.automatic_rename_format = automatic_rename_format.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_interval(mut self, c0_change_interval: Option<usize>) -> Self {
        self.c0_change_interval = c0_change_interval;
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_trigger(mut self, c0_change_trigger: Option<usize>) -> Self {
        self.c0_change_trigger = c0_change_trigger;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_colour<S: Into<Cow<'a, str>>>(
        mut self,
        clock_mode_colour: Option<S>,
    ) -> Self {
        self.clock_mode_colour = clock_mode_colour.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_style(mut self, clock_mode_style: Option<ClockModeStyle>) -> Self {
        self.clock_mode_style = clock_mode_style;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub fn force_height(mut self, force_height: Option<usize>) -> Self {
        self.force_height = force_height;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub fn force_width(mut self, force_width: Option<usize>) -> Self {
        self.force_width = force_width;
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub fn layout_history_limit(mut self, layout_history_limit: Option<usize>) -> Self {
        self.layout_history_limit = layout_history_limit;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn main_pane_height(mut self, main_pane_height: Option<usize>) -> Self {
        self.main_pane_height = main_pane_height;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn main_pane_width(mut self, main_pane_width: Option<usize>) -> Self {
        self.main_pane_width = main_pane_width;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_attr<S: Into<Cow<'a, str>>>(mut self, mode_attr: Option<S>) -> Self {
        self.mode_attr = mode_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_bg<S: Into<Cow<'a, str>>>(mut self, mode_bg: Option<S>) -> Self {
        self.mode_bg = mode_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_fg<S: Into<Cow<'a, str>>>(mut self, mode_fg: Option<S>) -> Self {
        self.mode_fg = mode_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn mode_keys(mut self, mode_keys: Option<StatusKeys>) -> Self {
        self.mode_keys = mode_keys;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn mode_mouse(mut self, mode_mouse: Option<ModeMouse>) -> Self {
        self.mode_mouse = mode_mouse;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn mode_style<S: Into<Cow<'a, str>>>(mut self, mode_style: Option<S>) -> Self {
        self.mode_style = mode_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn monitor_activity(mut self, monitor_activity: Option<Switch>) -> Self {
        self.monitor_activity = monitor_activity;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn monitor_content<S: Into<Cow<'a, str>>>(mut self, monitor_content: Option<S>) -> Self {
        self.monitor_content = monitor_content.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn monitor_bell(mut self, monitor_bell: Option<Switch>) -> Self {
        self.monitor_bell = monitor_bell;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn monitor_silence(mut self, monitor_silence: Option<usize>) -> Self {
        self.monitor_silence = monitor_silence;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_height(mut self, other_pane_height: Option<usize>) -> Self {
        self.other_pane_height = other_pane_height;
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn other_pane_width(mut self, other_pane_width: Option<usize>) -> Self {
        self.other_pane_width = other_pane_width;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn pane_active_border_style<S: Into<Cow<'a, str>>>(
        mut self,
        pane_active_border_style: Option<S>,
    ) -> Self {
        self.pane_active_border_style = pane_active_border_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_bg<S: Into<Cow<'a, str>>>(
        mut self,
        pane_active_border_bg: Option<S>,
    ) -> Self {
        self.pane_active_border_bg = pane_active_border_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn pane_active_border_fg<S: Into<Cow<'a, str>>>(
        mut self,
        pane_active_border_fg: Option<S>,
    ) -> Self {
        self.pane_active_border_fg = pane_active_border_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn pane_base_index(mut self, pane_base_index: Option<usize>) -> Self {
        self.pane_base_index = pane_base_index;
        self
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn pane_border_bg<S: Into<Cow<'a, str>>>(mut self, pane_border_bg: Option<S>) -> Self {
        self.pane_border_bg = pane_border_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn pane_border_fg<S: Into<Cow<'a, str>>>(mut self, pane_border_fg: Option<S>) -> Self {
        self.pane_border_fg = pane_border_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_format<S: Into<Cow<'a, str>>>(
        mut self,
        pane_border_format: Option<S>,
    ) -> Self {
        self.pane_border_format = pane_border_format.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_status(mut self, pane_border_status: Option<PaneBorderStatus>) -> Self {
        self.pane_border_status = pane_border_status;
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn pane_border_style<S: Into<Cow<'a, str>>>(
        mut self,
        pane_border_style: Option<S>,
    ) -> Self {
        self.pane_border_style = pane_border_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub fn remain_on_exit(mut self, remain_on_exit: Option<Switch>) -> Self {
        self.remain_on_exit = remain_on_exit;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    pub fn synchronize_panes(mut self, synchronize_panes: Option<Switch>) -> Self {
        self.synchronize_panes = synchronize_panes;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub fn utf8(mut self, utf8: Option<Switch>) -> Self {
        self.utf8 = utf8;
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_active_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_active_style: Option<S>,
    ) -> Self {
        self.window_active_style = window_active_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_attr: Option<S>,
    ) -> Self {
        self.window_status_bell_attr = window_status_bell_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_bg: Option<S>,
    ) -> Self {
        self.window_status_bell_bg = window_status_bell_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_fg: Option<S>,
    ) -> Self {
        self.window_status_bell_fg = window_status_bell_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_attr: Option<S>,
    ) -> Self {
        self.window_status_content_attr = window_status_content_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_bg: Option<S>,
    ) -> Self {
        self.window_status_content_bg = window_status_content_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_fg: Option<S>,
    ) -> Self {
        self.window_status_content_fg = window_status_content_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_attr: Option<S>,
    ) -> Self {
        self.window_status_activity_attr = window_status_activity_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_bg: Option<S>,
    ) -> Self {
        self.window_status_activity_bg = window_status_activity_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_fg: Option<S>,
    ) -> Self {
        self.window_status_activity_fg = window_status_activity_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_attr: Option<S>,
    ) -> Self {
        self.window_status_attr = window_status_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bg<S: Into<Cow<'a, str>>>(mut self, window_status_bg: Option<S>) -> Self {
        self.window_status_bg = window_status_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_fg<S: Into<Cow<'a, str>>>(mut self, window_status_fg: Option<S>) -> Self {
        self.window_status_fg = window_status_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_attr: Option<S>,
    ) -> Self {
        self.window_status_current_attr = window_status_current_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_bg: Option<S>,
    ) -> Self {
        self.window_status_current_bg = window_status_current_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_fg: Option<S>,
    ) -> Self {
        self.window_status_current_fg = window_status_current_fg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub fn window_status_alert_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_alert_attr: Option<S>,
    ) -> Self {
        self.window_status_alert_attr = window_status_alert_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub fn window_status_alert_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_alert_bg: Option<S>,
    ) -> Self {
        self.window_status_alert_bg = window_status_alert_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub fn window_status_alert_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_alert_fg: Option<S>,
    ) -> Self {
        self.window_status_alert_fg = window_status_alert_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_activity_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_activity_style: Option<S>,
    ) -> Self {
        self.window_status_activity_style = window_status_activity_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_bell_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_bell_style: Option<S>,
    ) -> Self {
        self.window_status_bell_style = window_status_bell_style.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn window_status_content_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_content_style: Option<S>,
    ) -> Self {
        self.window_status_content_style = window_status_content_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_current_format<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_format: Option<S>,
    ) -> Self {
        self.window_status_current_format = window_status_current_format.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_attr<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_attr: Option<S>,
    ) -> Self {
        self.window_status_last_attr = window_status_last_attr.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_bg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_bg: Option<S>,
    ) -> Self {
        self.window_status_last_bg = window_status_last_bg.map(|s| s.into());
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_fg<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_fg: Option<S>,
    ) -> Self {
        self.window_status_last_fg = window_status_last_fg.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_current_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_current_style: Option<S>,
    ) -> Self {
        self.window_status_current_style = window_status_current_style.map(|s| s.into());
        self
    }
    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_format<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_format: Option<S>,
    ) -> Self {
        self.window_status_format = window_status_format.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_last_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_last_style: Option<S>,
    ) -> Self {
        self.window_status_last_style = window_status_last_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn window_status_separator<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_separator: Option<S>,
    ) -> Self {
        self.window_status_separator = window_status_separator.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_style<S: Into<Cow<'a, str>>>(
        mut self,
        window_status_style: Option<S>,
    ) -> Self {
        self.window_status_style = window_status_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn window_size(mut self, window_size: Option<WindowSize>) -> Self {
        self.window_size = window_size;
        self
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub fn word_separators(mut self, word_separators: Option<Switch>) -> Self {
        self.word_separators = word_separators;
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_style<S: Into<Cow<'a, str>>>(mut self, window_style: Option<S>) -> Self {
        self.window_style = window_style.map(|s| s.into());
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn wrap_search(mut self, wrap_search: Option<Switch>) -> Self {
        self.wrap_search = wrap_search;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn xterm_keys(mut self, xterm_keys: Option<Switch>) -> Self {
        self.xterm_keys = xterm_keys;
        self
    }
}

impl<'a> FromStr for WindowOptions<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut window_options = WindowOptions::default();

        for line in s.lines() {
            if let Some((name, _i, value)) = get_parts(line) {
                match name {
                    #[cfg(feature = "tmux_1_0")]
                    AGGRESSIVE_RESIZE => {
                        window_options.aggressive_resize = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
                    ALLOW_RENAME => {
                        window_options.allow_rename = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
                    ALTERNATE_SCREEN => {
                        window_options.alternate_screen = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")] // 0.8
                    AUTOMATIC_RENAME => {
                        window_options.automatic_rename = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    AUTOMATIC_RENAME_FORMAT => {
                        window_options.automatic_rename_format = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
                    C0_CHANGE_INTERVAL => {
                        window_options.c0_change_interval = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
                    C0_CHANGE_TRIGGER => {
                        window_options.c0_change_trigger = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    CLOCK_MODE_COLOUR => window_options.clock_mode_colour = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    CLOCK_MODE_STYLE => {
                        window_options.clock_mode_style = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
                    FORCE_HEIGHT => {
                        window_options.force_height = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
                    FORCE_WIDTH => window_options.force_width = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
                    LAYOUT_HISTORY_LIMIT => {
                        window_options.layout_history_limit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    MAIN_PANE_HEIGHT => {
                        window_options.main_pane_height = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_0")]
                    MAIN_PANE_WIDTH => {
                        window_options.main_pane_width = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MODE_ATTR => window_options.mode_attr = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MODE_BG => window_options.mode_bg = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    MODE_FG => window_options.mode_fg = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    MODE_KEYS => window_options.mode_keys = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
                    MODE_MOUSE => window_options.mode_mouse = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_9")]
                    MODE_STYLE => window_options.mode_style = cow_parse(value),
                    #[cfg(feature = "tmux_1_0")]
                    MONITOR_ACTIVITY => {
                        window_options.monitor_activity = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
                    MONITOR_CONTENT => window_options.monitor_content = cow_parse(value),
                    #[cfg(feature = "tmux_2_6")]
                    MONITOR_BELL => {
                        window_options.monitor_bell = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    MONITOR_SILENCE => {
                        window_options.monitor_silence = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    OTHER_PANE_HEIGHT => {
                        window_options.other_pane_height = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_4")]
                    OTHER_PANE_WIDTH => {
                        window_options.other_pane_width = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_1_9")]
                    PANE_ACTIVE_BORDER_STYLE => {
                        window_options.pane_active_border_style = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
                    PANE_ACTIVE_BORDER_BG => {
                        window_options.pane_active_border_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
                    PANE_ACTIVE_BORDER_FG => {
                        window_options.pane_active_border_fg = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_6")]
                    PANE_BASE_INDEX => {
                        window_options.pane_base_index = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
                    PANE_BORDER_BG => window_options.pane_border_bg = cow_parse(value),
                    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
                    PANE_BORDER_FG => window_options.pane_border_fg = cow_parse(value),
                    #[cfg(feature = "tmux_2_3")]
                    PANE_BORDER_FORMAT => window_options.pane_border_format = cow_parse(value),
                    #[cfg(feature = "tmux_2_3")]
                    PANE_BORDER_STATUS => {
                        window_options.pane_border_status = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(feature = "tmux_2_0")]
                    PANE_BORDER_STYLE => window_options.pane_border_style = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
                    REMAIN_ON_EXIT => {
                        window_options.remain_on_exit = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
                    SYNCHRONIZE_PANES => {
                        window_options.synchronize_panes = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
                    UTF8 => window_options.utf8 = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
                    WINDOW_ACTIVE_STYLE => window_options.window_active_style = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BELL_ATTR => {
                        window_options.window_status_bell_attr = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BELL_BG => {
                        window_options.window_status_bell_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BELL_FG => {
                        window_options.window_status_bell_fg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CONTENT_ATTR => {
                        window_options.window_status_content_attr = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CONTENT_BG => {
                        window_options.window_status_content_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CONTENT_FG => {
                        window_options.window_status_content_fg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ACTIVITY_ATTR => {
                        window_options.window_status_activity_attr = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ACTIVITY_BG => {
                        window_options.window_status_activity_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ACTIVITY_FG => {
                        window_options.window_status_activity_fg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_ATTR => window_options.window_status_attr = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_BG => window_options.window_status_bg = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_FG => window_options.window_status_fg = cow_parse(value),
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CURRENT_ATTR => {
                        window_options.window_status_current_attr = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CURRENT_BG => {
                        window_options.window_status_current_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_CURRENT_FG => {
                        window_options.window_status_current_fg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
                    WINDOW_STATUS_ALERT_ATTR => {
                        window_options.window_status_alert_attr = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
                    WINDOW_STATUS_ALERT_BG => {
                        window_options.window_status_alert_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
                    WINDOW_STATUS_ALERT_FG => {
                        window_options.window_status_alert_fg = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_ACTIVITY_STYLE => {
                        window_options.window_status_activity_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_BELL_STYLE => {
                        window_options.window_status_bell_style = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
                    WINDOW_STATUS_CONTENT_STYLE => {
                        window_options.window_status_content_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_2")]
                    WINDOW_STATUS_CURRENT_FORMAT => {
                        window_options.window_status_current_format = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_LAST_ATTR => {
                        window_options.window_status_last_attr = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_LAST_BG => {
                        window_options.window_status_last_bg = cow_parse(value)
                    }
                    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
                    WINDOW_STATUS_LAST_FG => {
                        window_options.window_status_last_fg = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_CURRENT_STYLE => {
                        window_options.window_status_current_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_2")]
                    WINDOW_STATUS_FORMAT => window_options.window_status_format = cow_parse(value),
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_LAST_STYLE => {
                        window_options.window_status_last_style = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_7")]
                    WINDOW_STATUS_SEPARATOR => {
                        window_options.window_status_separator = cow_parse(value)
                    }
                    #[cfg(feature = "tmux_1_9")]
                    WINDOW_STATUS_STYLE => window_options.window_status_style = cow_parse(value),
                    #[cfg(feature = "tmux_2_9")]
                    WINDOW_SIZE => window_options.window_size = value.and_then(|s| s.parse().ok()),
                    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
                    WORD_SEPARATORS => {
                        window_options.word_separators = value.and_then(|s| s.parse().ok())
                    }
                    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
                    WINDOW_STYLE => window_options.window_style = cow_parse(value),
                    #[cfg(feature = "tmux_1_7")]
                    WRAP_SEARCH => window_options.wrap_search = value.and_then(|s| s.parse().ok()),
                    #[cfg(feature = "tmux_1_0")]
                    XTERM_KEYS => window_options.xterm_keys = value.and_then(|s| s.parse().ok()),
                    _ => {
                        // if user option (@user_option value)
                        if let Some(name) = name.strip_prefix('@') {
                            window_options
                                .user_options
                                .insert(name.to_string(), cow_parse(value));
                        }
                    }
                }
            }
        }

        Ok(window_options)
    }
}
