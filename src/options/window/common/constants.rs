// aggressive-resize [on | off]
#[cfg(feature = "tmux_1_0")]
pub const AGGRESIVE_RESIZE: &str = "aggressive-resize";
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
