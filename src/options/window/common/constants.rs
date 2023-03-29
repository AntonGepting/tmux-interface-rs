use crate::{ClockModeStyle, PaneBorderStatus, StatusKeys, Switch, WindowSize};

// aggressive-resize [on | off]
#[cfg(feature = "tmux_1_0")]
pub const AGGRESSIVE_RESIZE: &str = "aggressive-resize";
// allow-rename [on | off]
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
pub const ALLOW_RENAME: &str = "allow-rename";
// alternate-screen [on | off]
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
pub const ALTERNATE_SCREEN: &str = "alternate-screen";
// automatic-rename [on | off]
#[cfg(feature = "tmux_1_0")] // 0.8
pub const AUTOMATIC_RENAME: &str = "automatic-rename";
// automatic-rename-format format
#[cfg(feature = "tmux_1_9")]
pub const AUTOMATIC_RENAME_FORMAT: &str = "automatic-rename-format";
// c0-change-interval interval
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_INTERVAL: &str = "c0-change-interval";
// c0-change-trigger trigger
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_TRIGGER: &str = "c0-change-trigger";
// clock-mode-colour colour
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_COLOUR: &str = "clock-mode-colour";
// clock-mode-style [12 | 24]
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_STYLE: &str = "clock-mode-style";
// force-height height
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_HEIGHT: &str = "force-height";
// force-width width
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_WIDTH: &str = "force-width";
// layout-history-limit limit
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const LAYOUT_HISTORY_LIMIT: &str = "layout-history-limit";
// main-pane-height height
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_HEIGHT: &str = "main-pane-height";
// main-pane-width width
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_WIDTH: &str = "main-pane-width";
// mode-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_ATTR: &str = "mode-attr";
// mode-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_BG: &str = "mode-bg";
// mode-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_FG: &str = "mode-fg";
// mode-keys [vi | emacs]
#[cfg(feature = "tmux_1_0")]
pub const MODE_KEYS: &str = "mode-keys";
// mode-mouse [on | off]
//tmux 1.6: mode-mouse [on | off | copy-mode]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const MODE_MOUSE: &str = "mode-mouse";
// mode-style style
#[cfg(feature = "tmux_1_9")]
pub const MODE_STYLE: &str = "mode-style";
// monitor-activity [on | off]
#[cfg(feature = "tmux_1_0")]
pub const MONITOR_ACTIVITY: &str = "monitor-activity";
// monitor-content match-string
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const MONITOR_CONTENT: &str = "monitor-content";
// monitor-bell [on | off]
#[cfg(feature = "tmux_2_6")]
pub const MONITOR_BELL: &str = "monitor-bell";
// monitor-silence [interval]
#[cfg(feature = "tmux_1_4")]
pub const MONITOR_SILENCE: &str = "monitor-silence";
// other-pane-height height
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_HEIGHT: &str = "other-pane-height";
// other-pane-width width
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_WIDTH: &str = "other-pane-width";
// pane-active-border-style style
#[cfg(feature = "tmux_2_0")]
pub const PANE_ACTIVE_BORDER_STYLE: &str = "pane-active-border-style";
// pane-base-index index
#[cfg(feature = "tmux_1_6")]
pub const PANE_BASE_INDEX: &str = "pane-base-index";
// pane-border-format format
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_FORMAT: &str = "pane-border-format";
// pane-border-status [off | top | bottom]
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_STATUS: &str = "pane-border-status";
// pane-border-style style
#[cfg(feature = "tmux_2_0")]
pub const PANE_BORDER_STYLE: &str = "pane-border-style";
// remain-on-exit [on | off]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
pub const REMAIN_ON_EXIT: &str = "remain-on-exit";
// synchronize-panes [on | off]
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
pub const SYNCHRONIZE_PANES: &str = "synchronize-panes";
// utf8 [on | off]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub const UTF8: &str = "utf8";
// window-active-style style
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_ACTIVE_STYLE: &str = "window-active-style";
// window-status-bell-attr attributes
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_ATTR: &str = "window-status-bell-attr";
// window-status-bell-bg colour
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_BG: &str = "window-status-bell-bg";
// window-status-bell-fg colour
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_FG: &str = "window-status-bell-fg";
// window-status-content-attr attributes
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_ATTR: &str = "window-status-content-attr";
// window-status-content-bg colour
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_BG: &str = "window-status-content-bg";
// window-status-content-fg colour
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_FG: &str = "window-status-content-fg";
// window-status-activity-attr attributes
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_ATTR: &str = "window-status-activity-attr";
// window-status-activity-bg attributes
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_BG: &str = "window-status-activity-bg";
// window-status-activity-fg attributes
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_FG: &str = "window-status-activity-fg";
// window-status-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ATTR: &str = "window-status-attr";
// window-status-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BG: &str = "window-status-bg";
// window-status-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_FG: &str = "window-status-fg";
// window-status-current-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_ATTR: &str = "window-status-current-attr";
// window-status-current-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_BG: &str = "window-status-current-bg";
// window-status-current-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_FG: &str = "window-status-current-fg";
// window-status-alert-attr attributes
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_ATTR: &str = "window-status-alert-attr";
// window-status-alert-bg colour
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_BG: &str = "window-status-alert-bg";
// window-status-alert-fg colour
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_FG: &str = "window-status-alert-fg";
// window-status-activity-style style
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_ACTIVITY_STYLE: &str = "window-status-activity-style";
// window-status-bell-style style
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_BELL_STYLE: &str = "window-status-bell-style";
// window-status-content-style style
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const WINDOW_STATUS_CONTENT_STYLE: &str = "window-status-content-style";
// window-status-current-format string
#[cfg(feature = "tmux_1_2")]
pub const WINDOW_STATUS_CURRENT_FORMAT: &str = "window-status-current-format";
// window-status-last-attr attributes
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_ATTR: &str = "window-status-last-attr";
// window-status-last-bg colour
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_BG: &str = "window-status-last-bg";
// window-status-last-fg colour
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_FG: &str = "window-status-last-fg";
// window-status-current-style style
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_CURRENT_STYLE: &str = "window-status-current-style";
// window-status-format string
#[cfg(feature = "tmux_1_2")]
pub const WINDOW_STATUS_FORMAT: &str = "window-status-format";
// window-status-last-style style
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_LAST_STYLE: &str = "window-status-last-style";
// window-status-separator string
#[cfg(feature = "tmux_1_7")]
pub const WINDOW_STATUS_SEPARATOR: &str = "window-status-separator";
// window-status-style style
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_STYLE: &str = "window-status-style";
// window-size largest | smallest | manual | latest
#[cfg(feature = "tmux_2_9")]
pub const WINDOW_SIZE: &str = "window-size";
// word-separators string
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
pub const WORD_SEPARATORS: &str = "word-separators";
// window-style style
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_STYLE: &str = "window-style";
// wrap-search [on | off]
#[cfg(feature = "tmux_1_7")]
pub const WRAP_SEARCH: &str = "wrap-search";
// xterm-keys [on | off]
#[cfg(feature = "tmux_1_0")]
pub const XTERM_KEYS: &str = "xterm-keys";

// XXX: user options?
//pub user_options: Option<HashMap<String, String>>ub const AGGRESIVE_RESIZE: &str = "aggresive-resize";

/// ```text
/// aggressive-resize off
/// ```
#[cfg(feature = "tmux_1_0")]
pub const AGGRESSIVE_RESIZE_DEFAULT: Switch = Switch::Off;

/// ```text
/// allow-passthrough off
/// ```

/// ```text
/// allow-rename off
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
pub const ALLOW_RENAME_DEFAULT: Switch = Switch::Off;

/// ```text
/// alternate-screen on
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
pub const ALTERNATE_SCREEN_DEFAULT: Switch = Switch::On;

/// ```text
/// automatic-rename on
/// ```
#[cfg(feature = "tmux_1_0")] // 0.8
pub const AUTOMATIC_RENAME_DEFAULT: Switch = Switch::On;

/// ```text
/// automatic-rename-format "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}"
/// ```
#[cfg(feature = "tmux_1_9")]
pub const AUTOMATIC_RENAME_FORMAT_DEFAULT: &str =
    "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}";

/// ```text
/// c0-change-interval
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_INTERVAL_DEFAULT: &str = "";

/// ```text
/// c0-change-trigger //?
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_TRIGGER: &str = "";

/// ```text
/// clock-mode-colour blue // 4
/// ```
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_COLOUR_DEFAULT: &str = "blue";

/// ```text
/// clock-mode-style 24 // 1
/// ```
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_STYLE_DEFAULT: ClockModeStyle = ClockModeStyle::_24;

/// ```text
/// force-height 0
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_HEIGHT_DEFAULT: usize = 0;

/// ```text
/// force-width 0
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_WIDTH_DEFAULT: usize = 0;

/// ```text
/// layout-history-limit //?
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const LAYOUT_HISTORY_DEFAULT: usize = 0;

/// ```text
/// main-pane-height 24
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_HEIGHT_DEFAULT: usize = 24;

/// ```text
/// main-pane-width 80
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_WIDTH_DEFAULT: usize = 80;

/// ```text
/// mode-attr 0 // 0
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_ATTR_DEFAULT: usize = 0;

/// ```text
/// mode-bg yellow // 3
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_BG_DEFAULT: usize = 0;

/// ```text
/// mode-fg black // 0
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_FG_DEFAULT: usize = 0;

/// ```text
/// mode-keys vi // modekey_emacs
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MODE_KEYS_DEFAULT: StatusKeys = StatusKeys::Vi;

/// ```text
/// mode-mouse
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const MODE_MOUSE_DEFAULT: usize = 0;

/// ```text
/// mode-style fg=black,bg=yellow
/// ```
#[cfg(feature = "tmux_1_9")]
pub const MODE_STYLE_DEFAULT: &str = "fg=black,bg=yellow";

/// ```text
/// monitor-activity off
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MONITOR_ACTIVITY_DEFAULT: Switch = Switch::Off;

/// ```text
/// monitor-content //?
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const MONITOR_CONTENT_DEFAULT: usize = 0;

/// ```text
/// monitor-bell on
/// ```
#[cfg(feature = "tmux_2_6")]
pub const MONITOR_BELL_DEFAULT: Switch = Switch::On;

/// ```text
/// monitor-silence 0
/// ```
#[cfg(feature = "tmux_1_4")]
pub const MONITOR_SILENCE_DEFAULT: usize = 0;

/// ```text
/// other-pane-height 0
/// ```
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_HEIGHT_DEFAULT: usize = 0;

/// ```text
/// other-pane-width 0
/// ```
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_WIDTH_DEFAULT: usize = 0;

/// ```text
/// pane-active-border-style fg=green
/// ```
#[cfg(feature = "tmux_2_0")]
pub const PANE_ACTIVE_BORDER_STYLE_DEFAULT: &str = "fg=green";

/// ```text
/// pane-active-border-bg 8 // 8
/// ```
//pub const PANE_ACTIVE_BORDER_BG_DEFAULT: usize = 0;

/// ```text
/// pane-active-border-fg 2 // 2
/// ```
//pub const PANE_ACTIVE_BOREDER_FG_DEFAULT: usize = 0;

/// ```text
/// pane-base-index 0
/// ```
#[cfg(feature = "tmux_1_6")]
pub const PANE_BASE_INDEX_DEFAULT: usize = 0;

/// ```text
/// pane-border-bg 8 // 8
/// ```
// pub const PANE_BORDER_BG_DEFAULT: usize = 0;

/// ```text
/// pane-border-fg 8 // 8
/// ```
// pub const PANE_BORDER_FG_DEFAULT: usize = 0;

/// ```text
/// pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"
/// ```
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_FORMAT_DEFAULT: &str =
    "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"";

/// ```text
/// pane-border-status off
/// ```
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_STATUS_DEFAULT: PaneBorderStatus = PaneBorderStatus::Off;

/// ```text
/// pane-border-style default
/// ```
#[cfg(feature = "tmux_2_0")]
pub const PANE_BORDER_STYLE_DEFAULT: &str = "fg=colour238,bg=colour235";

/// ```text
/// remain-on-exit off
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
pub const REMAIN_ON_EXIT_DEFAULT: Switch = Switch::Off;

/// ```text
/// synchronize-panes off
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
pub const SYNCHRONIZE_PANES_DEFAULT: Switch = Switch::Off;

// #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
// utf8

/// ```text
/// window-active-style default
/// ```
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_ACTIVE_STYLE_DEFAULT: &str = "default";

/// ```text
/// window-size latest
/// ```
#[cfg(feature = "tmux_2_9")]
pub const WINDOW_SIZE_DEFAULT: WindowSize = WindowSize::Largest;

/// ```text
/// window-style default
/// ```
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_STYLE_DEFAULT: &str = "default";

/// ```text
/// window-status-activity-attr grid_attr_reverse // ?
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_ATTR_DEFAULT: usize = 0;

/// ```text
/// window-status-activity-bg 8 // 8
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_BG_DEFAULT: usize = 0;

/// ```text
/// window-status-activity-fg 8 // 8
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_FG_DEFAULT: usize = 0;

/// ```text
/// window-status-activity-style reverse
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_ACTIVITY_STYLE_DEFAULT: &str = "reverse";

/// ```text
/// window-status-attr 0 //
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ATTR_DEFAULT: usize = 0;

/// ```text
/// window-status-bell-attr gridattrreverse // ?
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_ATTR_DEFAULT: usize = 0;

/// ```text
/// window-status-bell-bg 8 // 8
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_BG_DEFAULT: usize = 0;

/// ```text
/// window-status-bell-fg 8 // 8
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_FG_DEFAULT: usize = 0;

/// ```text
/// window-status-bell-style reverse
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_BELL_STYLE_DEFAULT: &str = "fg=colour253,bg=colour1,bright";

/// ```text
/// window-status-bg 8 // 8
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BG_DEFAULT: usize = 0;

/// ```text
/// window-status-current-attr 0 //
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_ATTR_DEFAULT: usize = 0;

/// ```text
/// window-status-current-bg 8 //
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_BG_DEFAULT: usize = 0;

/// ```text
/// window-status-current-fg 8 //
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_FG_DEFAULT: usize = 0;

/// ```text
/// window-status-current-format #I:#W#{?window_flags,#{window_flags}, }
/// ```
#[cfg(feature = "tmux_1_2")]
pub const WINDOW_STATUS_CURRENT_FORMAT_DEFAULT: &str = "#I:#W#{?window_flags,#{window_flags}, }";

/// ```text
/// window-status-current-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_CURRENT_STYLE_DEFAULT: &str = "fg=colour22,bg=colour114";

/// ```text
/// window-status-fg 8 // 8
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_FG_DEFAULT: usize = 0;

/// ```text
/// window-status-format #I:#W#{?window_flags,#{window_flags}, }
/// ```
#[cfg(feature = "tmux_1_2")]
pub const WINDOW_STATUS_FORMAT_DEFAULT: &str = "#I:#W#{?window_flags,#{window_flags}, }";

/// ```text
/// window-status-last-attr 0 //
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_ATTR_DEFAULT: usize = 0;

/// ```text
/// window-status-last-bg 8 //
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_BG_DEFAULT: usize = 0;

/// ```text
/// window-status-last-fg 8 //
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_FG_DEFAULT: usize = 0;

/// ```text
/// window-status-last-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_LAST_STYLE_DEFAULT: &str = "default";

/// ```text
/// window-status-separator " "
/// ```
#[cfg(feature = "tmux_1_7")]
pub const WINDOW_STATUS_SEPARATOR_DEFAULT: &str = "\" \"";

/// ```text
/// window-status-style default
/// ```
pub const WINDOW_STATUS_STYLE_DEFAULT: &str = "default";

/// ```text
/// wrap-search on
/// ```
#[cfg(feature = "tmux_1_7")]
pub const WRAP_SEARCH_DEFAULT: Switch = Switch::On;

/// ```text
/// xterm-keys on
/// ```
#[cfg(feature = "tmux_1_0")]
pub const XTERM_KEYS_DEFAULT: Switch = Switch::On;
