#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
use crate::ModeMouse;
#[cfg(feature = "tmux_2_3")]
use crate::PaneBorderStatus;
#[cfg(feature = "tmux_2_9")]
use crate::WindowSize;
use crate::{ClockModeStyle, StatusKeys, Switch};

/// tmux ^1.0:
/// ```text
/// aggressive-resize
/// ```
#[cfg(feature = "tmux_1_0")]
pub const AGGRESSIVE_RESIZE: &str = "aggressive-resize";

/// tmux ^1.6 v3.0:
/// ```text
/// allow-rename
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
pub const ALLOW_RENAME: &str = "allow-rename";

/// ```text
/// alternate-screen
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
pub const ALTERNATE_SCREEN: &str = "alternate-screen";

/// ```text
/// automatic-rename
/// ```
#[cfg(feature = "tmux_1_0")] // 0.8
pub const AUTOMATIC_RENAME: &str = "automatic-rename";

/// ```text
/// automatic-rename-format
/// ```
#[cfg(feature = "tmux_1_9")]
pub const AUTOMATIC_RENAME_FORMAT: &str = "automatic-rename-format";

/// ```text
/// c0-change-interval
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_INTERVAL: &str = "c0-change-interval";

/// ```text
/// c0-change-trigger
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_TRIGGER: &str = "c0-change-trigger";

/// ```text
/// clock-mode-colour
/// ```
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_COLOUR: &str = "clock-mode-colour";

/// ```text
/// clock-mode-style
/// ```
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_STYLE: &str = "clock-mode-style";

/// ```text
/// force-height
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_HEIGHT: &str = "force-height";

/// ```text
/// force-width
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_WIDTH: &str = "force-width";

/// ```text
/// layout-history-limit
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const LAYOUT_HISTORY_LIMIT: &str = "layout-history-limit";

/// ```text
/// main-pane-height
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_HEIGHT: &str = "main-pane-height";

/// ```text
/// main-pane-width
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_WIDTH: &str = "main-pane-width";

/// ```text
/// mode-attr
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_ATTR: &str = "mode-attr";

/// ```text
/// mode-bg
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_BG: &str = "mode-bg";

/// ```text
/// mode-fg
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_FG: &str = "mode-fg";

/// ```text
/// mode-keys
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MODE_KEYS: &str = "mode-keys";

/// ```text
/// mode-mouse
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const MODE_MOUSE: &str = "mode-mouse";

/// ```text
/// mode-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const MODE_STYLE: &str = "mode-style";

/// ```text
/// monitor-activity
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MONITOR_ACTIVITY: &str = "monitor-activity";

/// ```text
/// monitor-content
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const MONITOR_CONTENT: &str = "monitor-content";

/// ```text
/// monitor-bell
/// ```
#[cfg(feature = "tmux_2_6")]
pub const MONITOR_BELL: &str = "monitor-bell";

/// ```text
/// monitor-silence
/// ```
#[cfg(feature = "tmux_1_4")]
pub const MONITOR_SILENCE: &str = "monitor-silence";

/// ```text
/// other-pane-height
/// ```
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_HEIGHT: &str = "other-pane-height";

/// ```text
/// other-pane-width
/// ```
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_WIDTH: &str = "other-pane-width";

/// ```text
/// pane-active-border-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const PANE_ACTIVE_BORDER_STYLE: &str = "pane-active-border-style";

/// ```text
/// pane-base-index
/// ```
#[cfg(feature = "tmux_1_6")]
pub const PANE_BASE_INDEX: &str = "pane-base-index";

/// ```text
/// pane-border-format
/// ```
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_FORMAT: &str = "pane-border-format";

/// ```text
/// pane-border-status
/// ```
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_STATUS: &str = "pane-border-status";

/// ```text
/// pane-border-style
/// ```
#[cfg(feature = "tmux_2_0")]
pub const PANE_BORDER_STYLE: &str = "pane-border-style";

/// ```text
/// remain-on-exit
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
pub const REMAIN_ON_EXIT: &str = "remain-on-exit";

/// ```text
/// synchronize-panes
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
pub const SYNCHRONIZE_PANES: &str = "synchronize-panes";

/// ```text
/// utf8
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub const UTF8: &str = "utf8";

/// ```text
/// window-active-style
/// ```
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_ACTIVE_STYLE: &str = "window-active-style";

/// ```text
/// window-status-bell-attr
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_ATTR: &str = "window-status-bell-attr";

/// ```text
/// window-status-bell-bg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_BG: &str = "window-status-bell-bg";

/// ```text
/// window-status-bell-fg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_FG: &str = "window-status-bell-fg";

/// ```text
/// window-status-content-attr
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_ATTR: &str = "window-status-content-attr";

/// ```text
/// window-status-content-bg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_BG: &str = "window-status-content-bg";

/// ```text
/// window-status-content-fg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_FG: &str = "window-status-content-fg";

/// ```text
/// window-status-activity-attr
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_ATTR: &str = "window-status-activity-attr";

/// ```text
/// window-status-activity-bg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_BG: &str = "window-status-activity-bg";

/// ```text
/// window-status-activity-fg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_FG: &str = "window-status-activity-fg";

/// ```text
/// window-status-attr
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ATTR: &str = "window-status-attr";

/// ```text
/// window-status-bg
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BG: &str = "window-status-bg";

/// ```text
/// window-status-fg
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_FG: &str = "window-status-fg";

/// ```text
/// window-status-current-attr
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_ATTR: &str = "window-status-current-attr";

/// ```text
/// window-status-current-bg
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_BG: &str = "window-status-current-bg";

/// ```text
/// window-status-current-fg
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_FG: &str = "window-status-current-fg";

/// ```text
/// window-status-alert-attr
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_ATTR: &str = "window-status-alert-attr";

/// ```text
/// window-status-alert-bg
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_BG: &str = "window-status-alert-bg";

/// ```text
/// window-status-alert-fg
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_FG: &str = "window-status-alert-fg";

/// ```text
/// window-status-activity-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_ACTIVITY_STYLE: &str = "window-status-activity-style";

/// ```text
/// window-status-bell-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_BELL_STYLE: &str = "window-status-bell-style";

/// ```text
/// window-status-content-style
/// ```
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const WINDOW_STATUS_CONTENT_STYLE: &str = "window-status-content-style";

/// ```text
/// window-status-current-format
/// ```
#[cfg(feature = "tmux_1_2")]
pub const WINDOW_STATUS_CURRENT_FORMAT: &str = "window-status-current-format";

/// ```text
/// window-status-last-attr
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_ATTR: &str = "window-status-last-attr";

/// ```text
/// window-status-last-bg
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_BG: &str = "window-status-last-bg";

/// ```text
/// window-status-last-fg
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_FG: &str = "window-status-last-fg";

/// ```text
/// window-status-current-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_CURRENT_STYLE: &str = "window-status-current-style";

/// ```text
/// window-status-format
/// ```
#[cfg(feature = "tmux_1_2")]
pub const WINDOW_STATUS_FORMAT: &str = "window-status-format";

/// ```text
/// window-status-last-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_LAST_STYLE: &str = "window-status-last-style";

/// ```text
/// window-status-separator
/// ```
#[cfg(feature = "tmux_1_7")]
pub const WINDOW_STATUS_SEPARATOR: &str = "window-status-separator";

/// ```text
/// window-status-style
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_STYLE: &str = "window-status-style";

/// ```text
/// window-size
/// ```
#[cfg(feature = "tmux_2_9")]
pub const WINDOW_SIZE: &str = "window-size";

/// ```text
/// word-separators
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
pub const WORD_SEPARATORS: &str = "word-separators";

/// ```text
/// window-style
/// ```
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_STYLE: &str = "window-style";

/// ```text
/// wrap-search
/// ```
#[cfg(feature = "tmux_1_7")]
pub const WRAP_SEARCH: &str = "wrap-search";

/// ```text
/// xterm-keys
/// ```
#[cfg(feature = "tmux_1_0")]
pub const XTERM_KEYS: &str = "xterm-keys";

/// ```text
/// aggressive-resize
/// ```
#[cfg(feature = "tmux_1_0")]
pub const AGGRESSIVE_RESIZE_DEFAULT: Switch = Switch::Off;

/// ```text
/// allow-passthrough off
/// ```

/// tmux ^2.7 v3.0:
/// ```text
/// allow-rename off
/// ```
///
/// tmux ^1.6 v2.6:
/// ```text
/// allow-rename on
/// ```
#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_3_0")))]
pub const ALLOW_RENAME_DEFAULT: Switch = Switch::Off;
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_7")))]
pub const ALLOW_RENAME_DEFAULT: Switch = Switch::On;

/// tmux ^1.2 v3.0:
/// ```text
/// alternate-screen on
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
pub const ALTERNATE_SCREEN_DEFAULT: Switch = Switch::On;

/// tmux ^1.0:
/// ```text
/// automatic-rename on
/// ```
#[cfg(feature = "tmux_1_0")] // 0.8
pub const AUTOMATIC_RENAME_DEFAULT: Switch = Switch::On;

/// tmux ^1.9:
/// ```text
/// automatic-rename-format "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}"
/// ```
#[cfg(feature = "tmux_1_9")]
pub const AUTOMATIC_RENAME_FORMAT_DEFAULT: &str =
    "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}";

/// tmux ^1.7 v2.1:
/// ```text
/// c0-change-interval interval
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_INTERVAL_DEFAULT: usize = 100;

/// tmux ^1.7 v2.1:
/// ```text
/// c0-change-trigger trigger
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
pub const C0_CHANGE_TRIGGER_DEFAULT: usize = 250;

/// tmux ^1.0:
/// ```text
/// clock-mode-colour blue
/// ```
// default_num = 4
// colour.c
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_COLOUR_DEFAULT: &str = "blue";

/// tmux ^1.0:
/// ```text
/// clock-mode-style 24
/// ```
// default_num = 1
// options.c
#[cfg(feature = "tmux_1_0")]
pub const CLOCK_MODE_STYLE_DEFAULT: ClockModeStyle = ClockModeStyle::H24;

/// tmux ^1.0 v2.9:
/// ```text
/// force-height 0
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_HEIGHT_DEFAULT: usize = 0;

/// tmux ^1.0 v2.9:
/// ```text
/// force-width 0
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
pub const FORCE_WIDTH_DEFAULT: usize = 0;

/// tmux ^1.0 v1.8:
/// ```text
/// layout-history-limit 20
/// ```
#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
pub const LAYOUT_HISTORY_LIMIT_DEFAULT: usize = 20;

/// tmux ^1.0:
/// ```text
/// main-pane-height 24
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_HEIGHT_DEFAULT: usize = 24;

/// tmux ^1.0:
/// ```text
/// main-pane-width 80
/// ```
#[cfg(feature = "tmux_1_0")]
pub const MAIN_PANE_WIDTH_DEFAULT: usize = 80;

/// tmux ^1.0 v1.9:
/// ```text
/// mode-attr none
/// ```
// default_num = 0
// attributes.c
// FIXME: type change, create attributes type
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// mode-bg yellow
/// ```
// default_num = 3
// colour.c
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_BG_DEFAULT: &str = "yellow";

/// tmux ^1.0 v1.9:
/// ```text
/// mode-fg black
/// ```
// default_num = 0
// colour.c
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MODE_FG_DEFAULT: &str = "black";

/// tmux ^1.0:
/// ```text
/// mode-keys emacs
/// ```
// default_num = MODEKEY_EMACS = 0
#[cfg(feature = "tmux_1_0")]
pub const MODE_KEYS_DEFAULT: StatusKeys = StatusKeys::Emacs;

/// tmux ^1.0 v2.1:
/// ```text
/// mode-mouse off
/// ```
// default_num = 0
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const MODE_MOUSE_DEFAULT: ModeMouse = ModeMouse::Off;

/// tmux ^1.9:
/// ```text
/// mode-style bg=yellow,fg=black
/// ```
#[cfg(feature = "tmux_1_9")]
pub const MODE_STYLE_DEFAULT: &str = "bg=yellow,fg=black";

/// tmux ^1.0:
/// ```text
/// monitor-activity off
/// ```
// default_num = 0 (OPTIONS_TABLE_FLAG)
#[cfg(feature = "tmux_1_0")]
pub const MONITOR_ACTIVITY_DEFAULT: Switch = Switch::Off;

/// tmux ^1.0 v2.0:
/// ```text
/// monitor-content ""
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const MONITOR_CONTENT_DEFAULT: &str = "";

/// tmux ^2.6:
/// ```text
/// monitor-bell on
/// ```
#[cfg(feature = "tmux_2_6")]
pub const MONITOR_BELL_DEFAULT: Switch = Switch::On;

/// tmux ^1.4:
/// ```text
/// monitor-silence 0
/// ```
#[cfg(feature = "tmux_1_4")]
pub const MONITOR_SILENCE_DEFAULT: usize = 0;

/// tmux ^1.4:
/// ```text
/// other-pane-height 0
/// ```
// number -> str? ^3.3?
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_HEIGHT_DEFAULT: usize = 0;

/// tmux ^1.4:
/// ```text
/// other-pane-width 0
/// ```
// number -> str? ^3.3?
#[cfg(feature = "tmux_1_4")]
pub const OTHER_PANE_WIDTH_DEFAULT: usize = 0;

/// tmux ^3.2:
/// ```text
/// pane-active-border-style #{?pane_in_mode,fg=yellow,#{?synchronize-panes,fg=red,fg=green}}
/// ```
///
/// tmux ^1.9 v3.2:
/// ```text
/// pane-active-border-style fg=green
/// ```
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_3_2")))]
pub const PANE_ACTIVE_BORDER_STYLE_DEFAULT: &str = "fg=green";
#[cfg(feature = "tmux_3_2")]
pub const PANE_ACTIVE_BORDER_STYLE_DEFAULT: &str =
    "#{?pane_in_mode,fg=yellow,#{?synchronize-panes,fg=red,fg=green}}";

/// tmux ^0.8 v1.9:
/// ```text
/// pane-active-border-bg default
/// ```
// default: 8
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub const PANE_ACTIVE_BORDER_BG_DEFAULT: &str = "default";

/// tmux ^0.8 v1.9:
/// ```text
/// pane-active-border-fg green
/// ```
// 2
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub const PANE_ACTIVE_BOREDER_FG_DEFAULT: &str = "green";

/// tmux ^1.6:
/// ```text
/// pane-base-index 0
/// ```
#[cfg(feature = "tmux_1_6")]
pub const PANE_BASE_INDEX_DEFAULT: usize = 0;

/// tmux ^0.8 v1.9:
/// ```text
/// pane-border-bg default
/// ```
// default: 8
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub const PANE_BORDER_BG_DEFAULT: &str = "default";

/// tmux ^0.8 v1.9:
/// ```text
/// pane-border-fg default
/// ```
// default: 8
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
pub const PANE_BORDER_FG_DEFAULT: &str = "default";

/// tmux ^2.3:
/// ```text
/// pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"
/// ```
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_FORMAT_DEFAULT: &str =
    "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"";

/// tmux ^2.3:
/// ```text
/// pane-border-status off
/// ```
#[cfg(feature = "tmux_2_3")]
pub const PANE_BORDER_STATUS_DEFAULT: PaneBorderStatus = PaneBorderStatus::Off;

/// tmux ^2.0:
/// ```text
/// pane-border-style default
/// ```
#[cfg(feature = "tmux_2_0")]
pub const PANE_BORDER_STYLE_DEFAULT: &str = "default";

/// tmux ^1.0 v3.0:
/// ```text
/// remain-on-exit off
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
pub const REMAIN_ON_EXIT_DEFAULT: Switch = Switch::Off;

/// tmux ^1.2 v3.2:
/// ```text
/// synchronize-panes off
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
pub const SYNCHRONIZE_PANES_DEFAULT: Switch = Switch::Off;

/// tmux ^1.0 v2.2:
/// ```text
/// utf8 off
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub const UTF8_DEFAULT: Switch = Switch::Off;

/// tmux ^2.1 v3.0:
/// ```text
/// window-active-style default
/// ```
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_ACTIVE_STYLE_DEFAULT: &str = "default";

/// tmux ^2.9 v3.1:
/// ```text
/// window-size smallest
/// ```
///
/// tmux ^3.1:
/// ```text
/// window-size latest
/// ```
#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_1")))]
pub const WINDOW_SIZE_DEFAULT: WindowSize = WindowSize::Smallest;
#[cfg(feature = "tmux_3_1")]
pub const WINDOW_SIZE_DEFAULT: WindowSize = WindowSize::Latest;

/// tmux ^2.1 v3.0:
/// ```text
/// window-style default
/// ```
#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
pub const WINDOW_STYLE_DEFAULT: &str = "default";

/// tmux ^1.6 v1.9:
/// ```text
/// window-status-activity-attr reverse
/// ```
/// grid_attr_reverse
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_ATTR_DEFAULT: &str = "reverse";

/// tmux ^1.6 v1.9:
/// ```text
/// window-status-activity-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_BG_DEFAULT: &str = "default";

/// tmux ^1.6 v1.9:
/// ```text
/// window-status-activity-fg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ACTIVITY_FG_DEFAULT: &str = "default";

/// tmux ^1.9:
/// ```text
/// window-status-activity-style reverse
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_ACTIVITY_STYLE_DEFAULT: &str = "reverse";

/// tmux ^1.0 v1.9:
/// ```text
/// window-status-attr none
/// ```
// 0
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_ATTR_DEFAULT: &str = "none";

/// tmux ^1.6 v1.9:
/// ```text
/// window-status-bell-attr reverse
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_ATTR_DEFAULT: &str = "reverse";

/// tmux ^1.6 v1.9:
/// ```text
/// window-status-bell-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_BG_DEFAULT: &str = "default";

/// tmux ^1.6 v1.9:
/// ```text
/// window-status-bell-fg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BELL_FG_DEFAULT: &str = "default";

/// ```text
/// window-status-content-attr
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_ATTR_DEFAULT: &str = "reverse";

/// ```text
/// window-status-content-bg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_BG_DEFAULT: &str = "default";

/// ```text
/// window-status-content-fg
/// ```
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CONTENT_FG_DEFAULT: &str = "default";

/// tmux ^1.9:
/// ```text
/// window-status-bell-style reverse
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_BELL_STYLE_DEFAULT: &str = "reverse";

/// tmux ^1.9 v2.0:
/// ```text
/// window-status-content-style reverse
/// ```
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const WINDOW_STATUS_CONTENT_STYLE_DEFAULT: &str = "reverse";

/// tmux ^1.0 v1.9:
/// ```text
/// window-status-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_BG_DEFAULT: &str = "default";

/// tmux ^1.0 v1.9:
/// ```text
/// window-status-current-attr none
/// ```
// 0
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// window-status-current-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_BG_DEFAULT: &str = "default";

/// tmux ^1.0 v1.9:
/// ```text
/// window-status-current-fg default
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_CURRENT_FG_DEFAULT: &str = "default";

/// tmux ^1.3 v1.6:
/// ```text
/// window-status-alert-attr reverse
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_ATTR_DEFAULT: &str = "reverse";

/// tmux ^1.3 v1.6:
/// ```text
/// window-status-alert-bg default
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_BG_DEFAULT: &str = "default";

/// tmux ^1.3 v1.6:
/// ```text
/// window-status-alert-fg default
/// ```
#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
pub const WINDOW_STATUS_ALERT_FG_DEFAULT: &str = "default";

/// tmux ^2.1:
/// ```text
/// window-status-current-format #I:#W#{?window_flags,#{window_flags}, }
/// ```
///
/// tmux ^1.2 v2.1:
/// ```text
/// window-status-current-format #I:#W#F
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
pub const WINDOW_STATUS_CURRENT_FORMAT_DEFAULT: &str = "#I:#W#F";
#[cfg(feature = "tmux_2_1")]
pub const WINDOW_STATUS_CURRENT_FORMAT_DEFAULT: &str = "#I:#W#{?window_flags,#{window_flags}, }";

/// tmux ^1.9:
/// ```text
/// window-status-current-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_CURRENT_STYLE_DEFAULT: &str = "default";

/// tmux ^1.0 v1.9:
/// ```text
/// window-status-fg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_FG_DEFAULT: &str = "default";

/// tmux ^2.1:
/// ```text
/// window-status-format #I:#W#{?window_flags,#{window_flags}, }
/// ```
///
///  tmux ^1.2 v2.1:
/// ```text
/// window-status-format #I:#W#F
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
pub const WINDOW_STATUS_FORMAT_DEFAULT: &str = "#I:#W#F";
#[cfg(feature = "tmux_2_1")]
pub const WINDOW_STATUS_FORMAT_DEFAULT: &str = "#I:#W#{?window_flags,#{window_flags}, }";

/// tmux ^1.8 v1.9:
/// ```text
/// window-status-last-attr none
/// ```
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_ATTR_DEFAULT: &str = "none";

/// tmux ^1.8 v1.9:
/// ```text
/// window-status-last-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_BG_DEFAULT: &str = "default";

/// tmux ^1.8 v1.9:
/// ```text
/// window-status-last-fg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
pub const WINDOW_STATUS_LAST_FG_DEFAULT: &str = "default";

/// tmux ^1.9:
/// ```text
/// window-status-last-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_LAST_STYLE_DEFAULT: &str = "default";

/// tmux ^1.7:
/// ```text
/// window-status-separator   // empty space: " "
/// ```
// FIXME not quite the same string, but token separation problem in parsing
#[cfg(feature = "tmux_1_7")]
pub const WINDOW_STATUS_SEPARATOR_DEFAULT: &str = "\" \"";

/// tmux ^1.2 v1.6:
/// ```text
/// word-separators
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
pub const WORD_SEPARATORS_DEFAULT: &str = " -_@";

/// tmux ^1.9:
/// ```text
/// window-status-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const WINDOW_STATUS_STYLE_DEFAULT: &str = "default";

/// tmux ^1.7:
/// ```text
/// wrap-search on
/// ```
#[cfg(feature = "tmux_1_7")]
pub const WRAP_SEARCH_DEFAULT: Switch = Switch::On;

/// tmux ^1.0:
/// ```text
/// xterm-keys on
/// ```
#[cfg(feature = "tmux_1_0")]
pub const XTERM_KEYS_DEFAULT: Switch = Switch::On;
