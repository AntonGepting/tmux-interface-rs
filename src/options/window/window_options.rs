use super::*;
use crate::options::StatusKeys;
use crate::Switch;

// XXX: conditionals?
// NOTE: total num: 70 (u128)
//pub const AGGRESIVE_RESIZE: u128 = 1;
//pub const ALLOW_RENAME: u128 = 1 << 1;
//pub const ALTERNAME_SCREEN: u128 = 1 << 2;
//pub const AUTOMATIC_RENAME: u128 = 1 << 3;
//pub const AUTOMATIC_RENAME_FORMAT: u128 = 1 << 4;
//pub const C0_CHANGE_INTERVAL: u128 = 1 << 5;
//pub const C0_CHANGE_TRIGGER: u128 = 1 << 6;
//pub const CLOCK_MODE_COLOUR: u128 = 1 << 7;
//pub const CLOCK_MODE_STYLE: u128 = 1 << 8;
//pub const FORCE_HEIGHT: u128 = 1 << 9;
//pub const FORCE_WIDTH: u128 = 1 << 10;
//pub const LAYOUT_HISTORY_LIMIT: u128 = 1 << 11;
//pub const MAIN_PANE_HEIGHT: u128 = 1 << 12;
//pub const MAIN_PANE_WIDTH: u128 = 1 << 13;
//pub const MODE_ATTR: u128 = 1 << 14;
//pub const MODE_BG: u128 = 1 << 15;
//pub const MODE_FG: u128 = 1 << 16;
//pub const MODE_KEYS: u128 = 1 << 17;
//pub const MODE_MOUSE: u128 = 1 << 18;
//pub const MODE_STYLE: u128 = 1 << 19;
//pub const MONITOR_ACTIVITY: u128 = 1 << 20;
//pub const MONITOR_CONTENT: u128 = 1 << 21;
//pub const MONITOR_BELL: u128 = 1 << 22;
//pub const MONITOR_SILENCE: u128 = 1 << 23;
//pub const OTHER_PANE_HEIGHT: u128 = 1 << 24;
//pub const OTHER_PANE_WIDTH: u128 = 1 << 25;
//pub const PANE_ACTIVE_BORDER_STYLE: u128 = 1 << 26;
//pub const PANE_BASE_INDEX: u128 = 1 << 27;
//pub const PANE_BORDER_FORMAT: u128 = 1 << 28;
//pub const PANE_BORDER_STATUS: u128 = 1 << 29;
//pub const PANE_BORDER_STYLE: u128 = 1 << 30;
//pub const REMAIN_ON_EXIT: u128 = 1 << 31;
//pub const SYNCHRONIZE_PANES: u128 = 1 << 32;
//pub const UTF8: u128 = 1 << 33;
//pub const WINDOW_ACTIVE_STYLE: u128 = 1 << 34;
//pub const WINDOW_STATUS_BELL_ATTR: u128 = 1 << 35;
//pub const WINDOW_STATUS_BELL_BG: u128 = 1 << 36;
//pub const WINDOW_STATUS_BELL_FG: u128 = 1 << 37;
//pub const WINDOW_STATUS_CONTENT_ATTR: u128 = 1 << 38;
//pub const WINDOW_STATUS_CONTENT_BG: u128 = 1 << 39;
//pub const WINDOW_STATUS_CONTENT_FG: u128 = 1 << 40;
//pub const WINDOW_STATUS_ACTIVITY_ATTR: u128 = 1 << 41;
//pub const WINDOW_STATUS_ACTIVITY_BG: u128 = 1 << 42;
//pub const WINDOW_STATUS_ACTIVITY_FG: u128 = 1 << 43;
//pub const WINDOW_STATUS_ATTR: u128 = 1 << 44;
//pub const WINDOW_STATUS_BG: u128 = 1 << 45;
//pub const WINDOW_STATUS_FG: u128 = 1 << 46;
//pub const WINDOW_STATUS_CURRENT_ATTR: u128 = 1 << 47;
//pub const WINDOW_STATUS_CURRENT_BG: u128 = 1 << 48;
//pub const WINDOW_STATUS_CURRENT_FG: u128 = 1 << 49;
//pub const WINDOW_STATUS_ALERT_ATTR: u128 = 1 << 50;
//pub const WINDOW_STATUS_ALERT_BG: u128 = 1 << 51;
//pub const WINDOW_STATUS_ALERT_FG: u128 = 1 << 52;
//pub const WINDOW_STATUS_ACTIVITY_STYLE: u128 = 1 << 53;
//pub const WINDOW_STATUS_BELL_STYLE: u128 = 1 << 54;
//pub const WINDOW_STATUS_CONTENT_STYLE: u128 = 1 << 55;
//pub const WINDOW_STATUS_CURRENT_FORMAT: u128 = 1 << 56;
//pub const WINDOW_STATUS_LAST_ATTR: u128 = 1 << 57;
//pub const WINDOW_STATUS_LAST_BG: u128 = 1 << 58;
//pub const WINDOW_STATUS_LAST_FG: u128 = 1 << 59;
//pub const WINDOW_STATUS_CURRENT_STYLE: u128 = 1 << 60;
//pub const WINDOW_STATUS_FORMAT: u128 = 1 << 61;
//pub const WINDOW_STATUS_LAST_STYLE: u128 = 1 << 62;
//pub const WINDOW_STATUS_SEPARATOR: u128 = 1 << 63;
//pub const WINDOW_STATUS_STYLE: u128 = 1 << 64;
//pub const WINDOW_SIZE: u128 = 1 << 65;
//pub const WORD_SEPARATORS: u128 = 1 << 66;
//pub const WINDOW_STYLE: u128 = 1 << 67;
//pub const WRAP_SEARCH: u128 = 1 << 68;
//pub const XTERM_KEYS: u128 = 1 << 69;

//pub const WINDOW_OPTIONS_NONE: u128 = 0;
//////pub const SERVER_OPTIONS_DEFAULT: usize = ;
//pub const WINDOW_OPTIONS_ALL: u128 = AGGRESIVE_RESIZE
//| ALLOW_RENAME
//| ALTERNAME_SCREEN
//| AUTOMATIC_RENAME
//| AUTOMATIC_RENAME_FORMAT
//| C0_CHANGE_INTERVAL
//| C0_CHANGE_TRIGGER
//| CLOCK_MODE_COLOUR
//| CLOCK_MODE_STYLE
//| FORCE_HEIGHT
//| FORCE_WIDTH
//| LAYOUT_HISTORY_LIMIT
//| MAIN_PANE_HEIGHT
//| MAIN_PANE_WIDTH
//| MODE_ATTR
//| MODE_BG
//| MODE_FG
//| MODE_KEYS
//| MODE_MOUSE
//| MODE_STYLE
//| MONITOR_ACTIVITY
//| MONITOR_CONTENT
//| MONITOR_BELL
//| MONITOR_SILENCE
//| OTHER_PANE_HEIGHT
//| OTHER_PANE_WIDTH
//| PANE_ACTIVE_BORDER_STYLE
//| PANE_BASE_INDEX
//| PANE_BORDER_FORMAT
//| PANE_BORDER_STATUS
//| PANE_BORDER_STYLE
//| REMAIN_ON_EXIT
//| SYNCHRONIZE_PANES
//| UTF8
//| WINDOW_ACTIVE_STYLE
//| WINDOW_STATUS_BELL_ATTR
//| WINDOW_STATUS_BELL_BG
//| WINDOW_STATUS_BELL_FG
//| WINDOW_STATUS_CONTENT_ATTR
//| WINDOW_STATUS_CONTENT_BG
//| WINDOW_STATUS_CONTENT_FG
//| WINDOW_STATUS_ACTIVITY_ATTR
//| WINDOW_STATUS_ACTIVITY_BG
//| WINDOW_STATUS_ACTIVITY_FG
//| WINDOW_STATUS_ATTR
//| WINDOW_STATUS_BG
//| WINDOW_STATUS_FG
//| WINDOW_STATUS_CURRENT_ATTR
//| WINDOW_STATUS_CURRENT_BG
//| WINDOW_STATUS_CURRENT_FG
//| WINDOW_STATUS_ALERT_ATTR
//| WINDOW_STATUS_ALERT_BG
//| WINDOW_STATUS_ALERT_FG
//| WINDOW_STATUS_ACTIVITY_STYLE
//| WINDOW_STATUS_BELL_STYLE
//| WINDOW_STATUS_CONTENT_STYLE
//| WINDOW_STATUS_CURRENT_FORMAT
//| WINDOW_STATUS_LAST_ATTR
//| WINDOW_STATUS_LAST_BG
//| WINDOW_STATUS_LAST_FG
//| WINDOW_STATUS_CURRENT_STYLE
//| WINDOW_STATUS_FORMAT
//| WINDOW_STATUS_LAST_STYLE
//| WINDOW_STATUS_SEPARATOR
//| WINDOW_STATUS_STYLE
//| WINDOW_SIZE
//| WORD_SEPARATORS
//| WINDOW_STYLE
//| WRAP_SEARCH
//| XTERM_KEYS;

//#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
//pub const WINDOW_OPTIONS_NUM: usize = 0;
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_1")))]
//pub const WINDOW_OPTIONS_NUM: usize = 24;
//#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
//pub const WINDOW_OPTIONS_NUM: usize = 25;
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_3")))]
//pub const WINDOW_OPTIONS_NUM: usize = 29;
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
//pub const WINDOW_OPTIONS_NUM: usize = 32;
//#[cfg(all(feature = "tmux_1_4", not(feature = "tmux_1_5")))]
//pub const WINDOW_OPTIONS_NUM: usize = 35;
//#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_6")))]
//pub const WINDOW_OPTIONS_NUM: usize = 35;
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_7")))]
//pub const WINDOW_OPTIONS_NUM: usize = 42;
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
//pub const WINDOW_OPTIONS_NUM: usize = 47;
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//pub const WINDOW_OPTIONS_NUM: usize = 49;
//#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_1_9a")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36;
//#[cfg(all(feature = "tmux_1_9a", not(feature = "tmux_2_0")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36;
//#[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_1")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36;
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
//pub const WINDOW_OPTIONS_NUM: usize = 35;
//#[cfg(all(feature = "tmux_2_2", not(feature = "tmux_2_3")))]
//pub const WINDOW_OPTIONS_NUM: usize = 34;
//#[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36;
//#[cfg(all(feature = "tmux_2_4", not(feature = "tmux_2_5")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36;
//#[cfg(all(feature = "tmux_2_5", not(feature = "tmux_2_6")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36;
//#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_2_7")))]
//pub const WINDOW_OPTIONS_NUM: usize = 37;
//#[cfg(all(feature = "tmux_2_7", not(feature = "tmux_2_8")))]
//pub const WINDOW_OPTIONS_NUM: usize = 37;
//#[cfg(all(feature = "tmux_2_8", not(feature = "tmux_2_9")))]
//pub const WINDOW_OPTIONS_NUM: usize = 37;
//#[cfg(all(feature = "tmux_2_9", not(feature = "tmux_2_9a")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36; // NOTE: 37 with double window-size entry
//#[cfg(all(feature = "tmux_2_9a", not(feature = "tmux_3_0")))]
//pub const WINDOW_OPTIONS_NUM: usize = 36; // NOTE: 37 with double window-size entry
//#[cfg(all(feature = "tmux_3_0", not(feature = "tmux_3_0a")))]
//pub const WINDOW_OPTIONS_NUM: usize = 31;
//#[cfg(all(feature = "tmux_3_0a", not(feature = "tmux_3_1")))]
//pub const WINDOW_OPTIONS_NUM: usize = 31;
//#[cfg(all(feature = "tmux_3_1", not(feature = "tmux_3_1a")))]
//pub const WINDOW_OPTIONS_NUM: usize = 31;
//#[cfg(all(feature = "tmux_3_1a", not(feature = "tmux_3_1b")))]
//pub const WINDOW_OPTIONS_NUM: usize = 31;
//#[cfg(all(feature = "tmux_3_1b", not(feature = "tmux_3_1c")))]
//pub const WINDOW_OPTIONS_NUM: usize = 31;
//#[cfg(all(feature = "tmux_3_1c", not(feature = "tmux_3_2")))]
//pub const WINDOW_OPTIONS_NUM: usize = 31;
//#[cfg(feature = "tmux_3_2")]
//pub const WINDOW_OPTIONS_NUM: usize = 30;
// FIXME: must be 33? 2 added

// TODO: waiting for const generics stabilization https://github.com/rust-lang/rust/issues/44580
//pub const WINDOW_OPTIONS: [(
//&str,
//fn(o: &mut WindowOptions, i: Option<usize>, s: &str),
//fn(o: &WindowOptions) -> Option<String>,
//u128,
//); WINDOW_OPTIONS_NUM] = [
//#[cfg(feature = "tmux_1_0")]
//(
//"aggressive-resize",
//|o, _, s| o.aggressive_resize = s.parse().ok(),
//|o| o.aggressive_resize.as_ref().map(|v| v.to_string()),
//AGGRESIVE_RESIZE,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
//(
//"allow-rename",
//|o, _, s| o.allow_rename = s.parse().ok(),
//|o| o.allow_rename.as_ref().map(|v| v.to_string()),
//ALLOW_RENAME,
//),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
//(
//"alternate-screen",
//|o, _, s| o.alternate_screen = s.parse().ok(),
//|o| o.alternate_screen.as_ref().map(|v| v.to_string()),
//ALTERNAME_SCREEN,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"automatic-rename",
//|o, _, s| o.automatic_rename = s.parse().ok(),
//|o| o.automatic_rename.as_ref().map(|v| v.to_string()),
//AUTOMATIC_RENAME,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"automatic-rename-format",
//|o, _, s| o.automatic_rename_format = Some(s.to_string()),
//|o| {
//o.automatic_rename_format
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//AUTOMATIC_RENAME_FORMAT,
//),
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
//(
//"c0-change-interval",
//|o, _, s| o.c0_change_interval = s.parse().ok(),
//|o| o.c0_change_interval.as_ref().map(|v| v.to_string()),
//C0_CHANGE_INTERVAL,
//),
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
//(
//"c0-change-trigger",
//|o, _, s| o.c0_change_trigger = s.parse().ok(),
//|o| o.c0_change_trigger.as_ref().map(|v| v.to_string()),
//C0_CHANGE_TRIGGER,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"clock-mode-colour",
//|o, _, s| o.clock_mode_colour = Some(s.to_string()),
//|o| {
//o.clock_mode_colour
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//CLOCK_MODE_COLOUR,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"clock-mode-style",
//|o, _, s| o.clock_mode_style = s.parse().ok(),
//|o| o.clock_mode_style.as_ref().map(|v| v.to_string()),
//CLOCK_MODE_STYLE,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
//(
//"force-height",
//|o, _, s| o.force_height = s.parse().ok(),
//|o| o.force_height.as_ref().map(|v| v.to_string()),
//FORCE_HEIGHT,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
//(
//"force-width",
//|o, _, s| o.force_width = s.parse().ok(),
//|o| o.force_width.as_ref().map(|v| v.to_string()),
//FORCE_WIDTH,
//),
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
//(
//"layout-history-limit",
//|o, _, s| o.layout_history_limit = s.parse().ok(),
//|o| o.layout_history_limit.as_ref().map(|v| v.to_string()),
//LAYOUT_HISTORY_LIMIT,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"main-pane-height",
//|o, _, s| o.main_pane_height = s.parse().ok(),
//|o| o.main_pane_height.as_ref().map(|v| v.to_string()),
//MAIN_PANE_HEIGHT,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"main-pane-width",
//|o, _, s| o.main_pane_width = s.parse().ok(),
//|o| o.main_pane_width.as_ref().map(|v| v.to_string()),
//MAIN_PANE_WIDTH,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"mode-attr",
//|o, _, s| o.mode_attr = s.parse().ok(),
//|o| o.mode_attr.as_ref().map(|v| v.to_string()),
//MODE_ATTR,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"mode-bg",
//|o, _, s| o.mode_bg = s.parse().ok(),
//|o| o.mode_bg.as_ref().map(|v| v.to_string()),
//MODE_BG,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"mode-fg",
//|o, _, s| o.mode_fg = s.parse().ok(),
//|o| o.mode_fg.as_ref().map(|v| v.to_string()),
//MODE_FG,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"mode-keys",
//|o, _, s| o.mode_keys = s.parse().ok(),
//|o| o.mode_keys.as_ref().map(|v| v.to_string()),
//MODE_KEYS,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
//(
//"mode-mouse",
//|o, _, s| o.mode_mouse = s.parse().ok(),
//|o| o.mode_mouse.as_ref().map(|v| v.to_string()),
//MODE_MOUSE,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"mode-style",
//|o, _, s| o.mode_style = Some(s.to_string()),
//|o| {
//o.mode_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//MODE_STYLE,
//),
//#[cfg(feature = "tmux_1_0")]
//(
//"monitor-activity",
//|o, _, s| o.monitor_activity = s.parse().ok(),
//|o| o.monitor_activity.as_ref().map(|v| v.to_string()),
//MONITOR_ACTIVITY,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
//(
//"monitor-content",
//|o, _, s| o.monitor_content = s.parse().ok(),
//|o| o.monitor_content.as_ref().map(|v| v.to_string()),
//MONITOR_CONTENT,
//),
//#[cfg(feature = "tmux_2_6")]
//(
//"monitor-bell",
//|o, _, s| o.monitor_bell = s.parse().ok(),
//|o| o.monitor_bell.as_ref().map(|v| v.to_string()),
//MONITOR_BELL,
//),
//#[cfg(feature = "tmux_1_4")]
//(
//"monitor-silence",
//|o, _, s| o.monitor_silence = s.parse().ok(),
//|o| o.monitor_silence.as_ref().map(|v| v.to_string()),
//MONITOR_SILENCE,
//),
//#[cfg(feature = "tmux_1_4")]
//(
//"other-pane-height",
//|o, _, s| o.other_pane_height = s.parse().ok(),
//|o| o.other_pane_height.as_ref().map(|v| v.to_string()),
//OTHER_PANE_HEIGHT,
//),
//#[cfg(feature = "tmux_1_4")]
//(
//"other-pane-width",
//|o, _, s| o.other_pane_width = s.parse().ok(),
//|o| o.other_pane_width.as_ref().map(|v| v.to_string()),
//OTHER_PANE_WIDTH,
//),
//#[cfg(feature = "tmux_2_0")]
//(
//"pane-active-border-style",
//|o, _, s| o.pane_active_border_style = Some(s.to_string()),
//|o| {
//o.pane_active_border_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//PANE_ACTIVE_BORDER_STYLE,
//),
//#[cfg(feature = "tmux_1_6")]
//(
//"pane-base-index",
//|o, _, s| o.pane_base_index = s.parse().ok(),
//|o| o.pane_base_index.as_ref().map(|v| v.to_string()),
//PANE_BASE_INDEX,
//),
//#[cfg(feature = "tmux_2_3")]
//(
//"pane-border-format",
//|o, _, s| o.pane_border_format = Some(s.to_string()),
//|o| {
//o.pane_border_format
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//PANE_BORDER_FORMAT,
//),
//#[cfg(feature = "tmux_2_3")]
//(
//"pane-border-status",
//|o, _, s| o.pane_border_status = s.parse().ok(),
//|o| o.pane_border_status.as_ref().map(|v| v.to_string()),
//PANE_BORDER_STATUS,
//),
//#[cfg(feature = "tmux_2_0")]
//(
//"pane-border-style",
//|o, _, s| o.pane_border_style = Some(s.to_string()),
//|o| {
//o.pane_border_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//PANE_BORDER_STYLE,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
//(
//"remain-on-exit",
//|o, _, s| o.remain_on_exit = s.parse().ok(),
//|o| o.remain_on_exit.as_ref().map(|v| v.to_string()),
//REMAIN_ON_EXIT,
//),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
//(
//"synchronize-panes",
//|o, _, s| o.synchronize_panes = s.parse().ok(),
//|o| o.synchronize_panes.as_ref().map(|v| v.to_string()),
//SYNCHRONIZE_PANES,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
//(
//"utf8",
//|o, _, s| o.utf8 = s.parse().ok(),
//|o| o.utf8.as_ref().map(|v| v.to_string()),
//UTF8,
//),
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
//(
//"window-active-style",
//|o, _, s| o.window_active_style = Some(s.to_string()),
//|o| {
//o.window_active_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_ACTIVE_STYLE,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-bell-attr",
//|o, _, s| o.window_status_bell_attr = Some(s.to_string()),
//|o| {
//o.window_status_bell_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_BELL_ATTR,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-bell-bg",
//|o, _, s| o.window_status_bell_bg = Some(s.to_string()),
//|o| {
//o.window_status_bell_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_BELL_BG,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-bell-fg",
//|o, _, s| o.window_status_bell_fg = Some(s.to_string()),
//|o| {
//o.window_status_bell_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_BELL_FG,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-content-attr",
//|o, _, s| o.window_status_content_attr = Some(s.to_string()),
//|o| {
//o.window_status_content_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CONTENT_ATTR,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-content-bg",
//|o, _, s| o.window_status_content_bg = Some(s.to_string()),
//|o| {
//o.window_status_content_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CONTENT_BG,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-content-fg",
//|o, _, s| o.window_status_content_fg = Some(s.to_string()),
//|o| {
//o.window_status_content_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CONTENT_FG,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-activity-attr",
//|o, _, s| o.window_status_activity_attr = Some(s.to_string()),
//|o| {
//o.window_status_activity_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ACTIVITY_ATTR,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-activity-bg",
//|o, _, s| o.window_status_activity_bg = Some(s.to_string()),
//|o| {
//o.window_status_activity_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ACTIVITY_BG,
//),
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//(
//"window-status-activity-fg",
//|o, _, s| o.window_status_activity_fg = Some(s.to_string()),
//|o| {
//o.window_status_activity_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ACTIVITY_FG,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"window-status-attr",
//|o, _, s| o.window_status_attr = Some(s.to_string()),
//|o| {
//o.window_status_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ATTR,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"window-status-bg",
//|o, _, s| o.window_status_bg = Some(s.to_string()),
//|o| {
//o.window_status_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_BG,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"window-status-fg",
//|o, _, s| o.window_status_fg = Some(s.to_string()),
//|o| {
//o.window_status_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_FG,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"window-status-current-attr",
//|o, _, s| o.window_status_current_attr = Some(s.to_string()),
//|o| {
//o.window_status_current_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CURRENT_ATTR,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"window-status-current-bg",
//|o, _, s| o.window_status_current_bg = Some(s.to_string()),
//|o| {
//o.window_status_current_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CURRENT_BG,
//),
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//(
//"window-status-current-fg",
//|o, _, s| o.window_status_current_fg = Some(s.to_string()),
//|o| {
//o.window_status_current_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CURRENT_FG,
//),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
//(
//"window-status-alert-attr",
//|o, _, s| o.window_status_alert_attr = Some(s.to_string()),
//|o| {
//o.window_status_alert_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ALERT_ATTR,
//),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
//(
//"window-status-alert-bg",
//|o, _, s| o.window_status_alert_bg = Some(s.to_string()),
//|o| {
//o.window_status_alert_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ALERT_BG,
//),
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
//(
//"window-status-alert-fg",
//|o, _, s| o.window_status_alert_fg = Some(s.to_string()),
//|o| {
//o.window_status_alert_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ALERT_FG,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"window-status-activity-style",
//|o, _, s| o.window_status_activity_style = Some(s.to_string()),
//|o| {
//o.window_status_activity_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_ACTIVITY_STYLE,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"window-status-bell-style",
//|o, _, s| o.window_status_bell_style = Some(s.to_string()),
//|o| {
//o.window_status_bell_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_BELL_STYLE,
//),
//#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
//(
//"window-status-content-style",
//|o, _, s| o.window_status_content_style = Some(s.to_string()),
//|o| {
//o.window_status_content_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CONTENT_STYLE,
//),
//#[cfg(feature = "tmux_1_2")]
//(
//"window-status-current-format",
//|o, _, s| o.window_status_current_format = Some(s.to_string()),
//|o| {
//o.window_status_current_format
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CURRENT_FORMAT,
//),
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//(
//"window-status-last-attr",
//|o, _, s| o.window_status_last_attr = Some(s.to_string()),
//|o| {
//o.window_status_last_attr
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_LAST_ATTR,
//),
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//(
//"window-status-last-bg",
//|o, _, s| o.window_status_last_bg = Some(s.to_string()),
//|o| {
//o.window_status_last_bg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_LAST_BG,
//),
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//(
//"window-status-last-fg",
//|o, _, s| o.window_status_last_fg = Some(s.to_string()),
//|o| {
//o.window_status_last_fg
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_LAST_FG,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"window-status-current-style",
//|o, _, s| o.window_status_current_style = Some(s.to_string()),
//|o| {
//o.window_status_current_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_CURRENT_STYLE,
//),
//#[cfg(feature = "tmux_1_2")]
//(
//"window-status-format",
//|o, _, s| o.window_status_format = Some(s.to_string()),
//|o| {
//o.window_status_format
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_FORMAT,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"window-status-last-style",
//|o, _, s| o.window_status_last_style = Some(s.to_string()),
//|o| {
//o.window_status_last_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_LAST_STYLE,
//),
//#[cfg(feature = "tmux_1_7")]
//(
//"window-status-separator",
//|o, _, s| o.window_status_separator = Some(s.to_string()),
//|o| {
//o.window_status_separator
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_SEPARATOR,
//),
//#[cfg(feature = "tmux_1_9")]
//(
//"window-status-style",
//|o, _, s| o.window_status_style = Some(s.to_string()),
//|o| {
//o.window_status_style
//.as_ref()
//.map(|v| format!("\"{}\"", v.to_string()))
//},
//WINDOW_STATUS_STYLE,
//),
//#[cfg(feature = "tmux_2_9")]
//(
//"window-size",
//|o, _, s| o.window_size = s.parse().ok(),
//|o| o.window_size.as_ref().map(|v| v.to_string()),
//WINDOW_SIZE,
//),
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
//(
//"word-separators",
//|o, _, s| o.word_separators = s.parse().ok(),
//|o| o.word_separators.as_ref().map(|v| v.to_string()),
//WORD_SEPARATORS,
//),
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
//(
//"window-style",
//|o, _, s| o.window_style = s.parse().ok(),
//|o| o.window_style.as_ref().map(|v| v.to_string()),
//WINDOW_STYLE,
//),
//#[cfg(feature = "tmux_1_7")]
//(
//"wrap-search",
//|o, _, s| o.wrap_search = s.parse().ok(),
//|o| o.wrap_search.as_ref().map(|v| v.to_string()),
//WRAP_SEARCH,
//),
//#[cfg(feature = "tmux_1_2")]
//(
//"xterm-keys",
//|o, _, s| o.xterm_keys = s.parse().ok(),
//|o| o.xterm_keys.as_ref().map(|v| v.to_string()),
//XTERM_KEYS,
//),
//];

// TODO: check types
// 31 Available window options are:
#[derive(PartialEq, Clone, Debug)]
pub struct WindowOptions {
    //aggressive-resize [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub aggressive_resize: Option<Switch>,
    //allow-rename [on | off]
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    pub allow_rename: Option<Switch>,
    //alternate-screen [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    pub alternate_screen: Option<Switch>,
    //automatic-rename [on | off]
    #[cfg(feature = "tmux_1_0")] // 0.8
    pub automatic_rename: Option<Switch>,
    //automatic-rename-format format
    #[cfg(feature = "tmux_1_9")]
    pub automatic_rename_format: Option<String>,
    //c0-change-interval interval
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_interval: Option<usize>,
    //c0-change-trigger trigger
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub c0_change_trigger: Option<usize>,
    //clock-mode-colour colour
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_colour: Option<String>,
    //clock-mode-style [12 | 24]
    #[cfg(feature = "tmux_1_0")]
    pub clock_mode_style: Option<ClockModeStyle>,
    //force-height height
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_height: Option<usize>,
    //force-width width
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    pub force_width: Option<usize>,
    //layout-history-limit limit
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    pub layout_history_limit: Option<usize>,
    //main-pane-height height
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_height: Option<usize>,
    //main-pane-width width
    #[cfg(feature = "tmux_1_0")]
    pub main_pane_width: Option<usize>,
    //mode-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_attr: Option<String>,
    // mode-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_bg: Option<String>,
    // mode-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub mode_fg: Option<String>,
    //mode-keys [vi | emacs]
    #[cfg(feature = "tmux_1_0")]
    pub mode_keys: Option<StatusKeys>,
    //mode-mouse [on | off]
    //tmux 1.6: mode-mouse [on | off | copy-mode]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub mode_mouse: Option<Switch>,
    //mode-style style
    #[cfg(feature = "tmux_1_9")]
    pub mode_style: Option<String>,
    //monitor-activity [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub monitor_activity: Option<Switch>,
    //monitor-content match-string
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub monitor_content: Option<String>,
    //monitor-bell [on | off]
    #[cfg(feature = "tmux_2_6")]
    pub monitor_bell: Option<Switch>,
    //monitor-silence [interval]
    #[cfg(feature = "tmux_1_4")]
    pub monitor_silence: Option<usize>,
    //other-pane-height height
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_height: Option<usize>,
    //other-pane-width width
    #[cfg(feature = "tmux_1_4")]
    pub other_pane_width: Option<usize>,
    //pane-active-border-style style
    #[cfg(feature = "tmux_2_0")]
    pub pane_active_border_style: Option<String>,
    //pane-base-index index
    #[cfg(feature = "tmux_1_6")]
    pub pane_base_index: Option<usize>,
    //pane-border-format format
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_format: Option<String>,
    //pane-border-status [off | top | bottom]
    #[cfg(feature = "tmux_2_3")]
    pub pane_border_status: Option<PaneBorderStatus>,
    //pane-border-style style
    #[cfg(feature = "tmux_2_0")]
    pub pane_border_style: Option<String>,
    //remain-on-exit [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    pub remain_on_exit: Option<Switch>,
    //synchronize-panes [on | off]
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    pub synchronize_panes: Option<Switch>,
    //utf8 [on | off]
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    pub utf8: Option<Switch>,
    //window-active-style style
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_active_style: Option<String>,
    //window-status-bell-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_attr: Option<String>,
    //window-status-bell-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_bg: Option<String>,
    //window-status-bell-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_bell_fg: Option<String>,
    //window-status-content-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_attr: Option<String>,
    //window-status-content-bg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_bg: Option<String>,
    //window-status-content-fg colour
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_content_fg: Option<String>,
    //window-status-activity-attr attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_attr: Option<String>,
    //window-status-activity-bg attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_bg: Option<String>,
    //window-status-activity-fg attributes
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub window_status_activity_fg: Option<String>,
    //window-status-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_attr: Option<String>,
    //window-status-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_bg: Option<String>,
    //window-status-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_fg: Option<String>,
    //window-status-current-attr attributes
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_attr: Option<String>,
    //window-status-current-bg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_bg: Option<String>,
    //window-status-current-fg colour
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub window_status_current_fg: Option<String>,
    //window-status-alert-attr attributes
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_attr: Option<String>,
    //window-status-alert-bg colour
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_bg: Option<String>,
    //window-status-alert-fg colour
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    pub window_status_alert_fg: Option<String>,
    //window-status-activity-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_activity_style: Option<String>,
    //window-status-bell-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_bell_style: Option<String>,
    //window-status-content-style style
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub window_status_content_style: Option<String>,
    //window-status-current-format string
    #[cfg(feature = "tmux_1_2")]
    pub window_status_current_format: Option<String>,
    //window-status-last-attr attributes
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_attr: Option<String>,
    //window-status-last-bg colour
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_bg: Option<String>,
    //window-status-last-fg colour
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub window_status_last_fg: Option<String>,
    //window-status-current-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_current_style: Option<String>,
    //window-status-format string
    #[cfg(feature = "tmux_1_2")]
    pub window_status_format: Option<String>,
    //window-status-last-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_last_style: Option<String>,
    //window-status-separator string
    #[cfg(feature = "tmux_1_7")]
    pub window_status_separator: Option<String>,
    //window-status-style style
    #[cfg(feature = "tmux_1_9")]
    pub window_status_style: Option<String>,
    //window-size largest | smallest | manual | latest
    #[cfg(feature = "tmux_2_9")]
    pub window_size: Option<WindowSize>,
    //word-separators string
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    pub word_separators: Option<String>,
    //window-style style
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub window_style: Option<String>,
    //wrap-search [on | off]
    #[cfg(feature = "tmux_1_7")]
    pub wrap_search: Option<Switch>,
    //xterm-keys [on | off]
    #[cfg(feature = "tmux_1_0")]
    pub xterm_keys: Option<Switch>,
    // XXX: user options?
    //pub user_options: Option<HashMap<String, String>>
}

/// ```text
/// tmux show-options -g -w
/// ```
///
/// ```text
/// aggressive-resize off
/// allow-rename off
/// alternate-screen on
/// automatic-rename on
/// automatic-rename-format "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}"
/// clock-mode-colour colour135
/// clock-mode-style 24
/// main-pane-height 24
/// main-pane-width 80
/// mode-keys vi
/// mode-style fg=colour196,bg=colour238,bright
/// monitor-activity off
/// monitor-bell on
/// monitor-silence 0
/// other-pane-height 0
/// other-pane-width 0
/// pane-active-border-style fg=colour114,bg=colour235
/// pane-base-index 0
/// pane-border-format "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\""
/// pane-border-status off
/// pane-border-style fg=colour238,bg=colour235
/// remain-on-exit off
/// synchronize-panes off
/// window-active-style fg=colour253,bg=colour235
/// window-size latest
/// window-style fg=colour247,bg=colour238
/// window-status-activity-style reverse
/// window-status-bell-style fg=colour253,bg=colour1,bright
/// window-status-current-format " #I: #W #F "
/// window-status-current-style fg=colour22,bg=colour114
/// window-status-format " #I: #W #F "
/// window-status-last-style default
/// window-status-separator " "
/// window-status-style fg=colour247,bg=#282c34
/// wrap-search on
/// xterm-keys on
/// ```
impl Default for WindowOptions {
    fn default() -> Self {
        let options = WindowOptions::new();

        #[cfg(feature = "tmux_1_0")]
        let options = options.aggressive_resize(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
        let options = options.allow_rename(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
        let options = options.alternate_screen(Some(Switch::On));
        #[cfg(feature = "tmux_1_0")] // 0.8
        let options = options.automatic_rename(Some(Switch::On));
        #[cfg(feature = "tmux_1_9")]
        let options = options.automatic_rename_format(Some(String::from(
            "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}",
        )));
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let options = options.c0_change_interval(None);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
        let options = options.c0_change_trigger(None);
        #[cfg(feature = "tmux_1_0")]
        let options = options.clock_mode_colour(Some(String::from("colour135")));
        #[cfg(feature = "tmux_1_0")]
        let options = options.clock_mode_style(Some(ClockModeStyle::_24));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let options = options.force_height(None);
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
        let options = options.force_width(None);
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
        let options = options.layout_history_limit(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.main_pane_height(Some(24));
        #[cfg(feature = "tmux_1_0")]
        let options = options.main_pane_width(Some(80));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.mode_fg(Some());
        #[cfg(feature = "tmux_1_0")]
        let options = options.mode_keys(Some(StatusKeys::Vi));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
        let options = options.mode_mouse(Some());
        #[cfg(feature = "tmux_1_9")]
        let options = options.mode_style(Some(String::from("fg=colour196,bg=colour238,bright")));
        #[cfg(feature = "tmux_1_0")]
        let options = options.monitor_activity(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
        let options = options.monitor_content(Some());
        #[cfg(feature = "tmux_2_6")]
        let options = options.monitor_bell(Some(Switch::On));
        #[cfg(feature = "tmux_1_4")]
        let options = options.monitor_silence(Some(0));
        #[cfg(feature = "tmux_1_4")]
        let options = options.other_pane_height(Some(0));
        #[cfg(feature = "tmux_1_4")]
        let options = options.other_pane_width(Some(0));
        #[cfg(feature = "tmux_2_0")]
        let options =
            options.pane_active_border_style(Some(String::from("fg=colour114,bg=colour235")));
        #[cfg(feature = "tmux_1_6")]
        let options = options.pane_base_index(Some(0));
        #[cfg(feature = "tmux_2_3")]
        let options = options.pane_border_format(Some(String::from(
            "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
        )));
        #[cfg(feature = "tmux_2_3")]
        let options = options.pane_border_status(Some(PaneBorderStatus::Off));
        #[cfg(feature = "tmux_2_0")]
        let options = options.pane_border_style(Some(String::from("fg=colour238,bg=colour235")));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
        let options = options.remain_on_exit(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
        let options = options.synchronize_panes(Some(Switch::Off));
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
        let options = options.utf8(Some());
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let options = options.window_active_style(Some(String::from("fg=colour253,bg=colour235")));
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_bell_fg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_content_fg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_attr(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_bg(Some());
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        let options = options.window_status_activity_fg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_fg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_attr(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_bg(Some());
        #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
        let options = options.window_status_current_fg(Some());
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_attr(Some());
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_bg(Some());
        #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
        let options = options.window_status_alert_fg(Some());
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_activity_style(Some(String::from("Reverse")));
        #[cfg(feature = "tmux_1_9")]
        let options =
            options.window_status_bell_style(Some(String::from("fg=colour253,bg=colour1,bright")));
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        let options = options.window_status_content_style(Some());
        #[cfg(feature = "tmux_1_2")]
        let options = options.window_status_current_format(Some(String::from(" #I: #W #F ")));
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_attr(Some());
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_bg(Some());
        #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
        let options = options.window_status_last_fg(Some());
        #[cfg(feature = "tmux_1_9")]
        let options =
            options.window_status_current_style(Some(String::from("fg=colour22,bg=colour114")));
        #[cfg(feature = "tmux_1_2")]
        let options = options.window_status_format(Some(String::from(" #I: #W #F ")));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_last_style(Some(String::from("default")));
        #[cfg(feature = "tmux_1_7")]
        let options = options.window_status_separator(Some(String::from(" ")));
        #[cfg(feature = "tmux_1_9")]
        let options = options.window_status_style(Some(String::from("fg=colour247,bg=#282c34")));
        #[cfg(feature = "tmux_2_9")]
        let options = options.window_size(Some(Latest));
        #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
        let options = options.word_separators(Some());
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
        let options = options.window_style(Some(String::from("fg=colour247,bg=colour238")));
        #[cfg(feature = "tmux_1_7")]
        let options = options.wrap_search(Some(Switch::On));
        #[cfg(feature = "tmux_1_0")]
        let options = options.xterm_keys(Some(Switch::On));
        options
    }
}

impl WindowOptions {
    pub fn new() -> Self {
        WindowOptions {
            #[cfg(feature = "tmux_1_0")]
            aggressive_resize: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
            allow_rename:None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            alternate_screen: None,
            #[cfg(feature = "tmux_1_0")] // 0.8
            automatic_rename: None,
            #[cfg(feature = "tmux_1_9")]
            automatic_rename_format: None,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            c0_change_interval: None,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
            c0_change_trigger: None,
            #[cfg(feature = "tmux_1_0")]
            clock_mode_colour: None,
            #[cfg(feature = "tmux_1_0")]
            clock_mode_style: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
            force_height: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
            force_width: None,
            #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
            layout_history_limit: None,
            #[cfg(feature = "tmux_1_0")]
            main_pane_height: None,
            #[cfg(feature = "tmux_1_0")]
            main_pane_width: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            mode_fg: None,
            #[cfg(feature = "tmux_1_0")]
            mode_keys: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
            mode_mouse: None,
            #[cfg(feature = "tmux_1_9")]
            mode_style: None,
            #[cfg(feature = "tmux_1_0")]
            monitor_activity: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
            monitor_content: None,
            #[cfg(feature = "tmux_2_6")]
            monitor_bell: None,
            #[cfg(feature = "tmux_1_4")]
            monitor_silence: None,
            #[cfg(feature = "tmux_1_4")]
            other_pane_height: None,
            #[cfg(feature = "tmux_1_4")]
            other_pane_width: None,
            #[cfg(feature = "tmux_2_0")]
            pane_active_border_style: None,
            #[cfg(feature = "tmux_1_6")]
            pane_base_index: None,
            #[cfg(feature = "tmux_2_3")]
            pane_border_format: None,
            #[cfg(feature = "tmux_2_3")]
            pane_border_status: None,
            #[cfg(feature = "tmux_2_0")]
            pane_border_style: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
            remain_on_exit: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
            synchronize_panes: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
            utf8: None,
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
            window_active_style: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_bell_fg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_content_fg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_attr: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_bg: None,
            #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
            window_status_activity_fg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_fg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_attr: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_bg: None,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
            window_status_current_fg: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_attr: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_bg: None,
            #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
            window_status_alert_fg: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_activity_style: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_bell_style: None,
            #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
            window_status_content_style: None,
            #[cfg(feature = "tmux_1_2")]
            window_status_current_format: None,
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_attr: None,
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_bg: None,
            #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
            window_status_last_fg: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_current_style: None,
            #[cfg(feature = "tmux_1_2")]
            window_status_format: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_last_style: None,
            #[cfg(feature = "tmux_1_7")]
            window_status_separator: None,
            #[cfg(feature = "tmux_1_9")]
            window_status_style: None,
            #[cfg(feature = "tmux_2_9")]
            window_size: None,
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
            word_separators: None,
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
            window_style: None,
            #[cfg(feature = "tmux_1_7")]
            wrap_search: None,
            #[cfg(feature = "tmux_1_0")]
            xterm_keys: None,
            // XXX: user options?
            //pub user_options: Option<HashMap<String, String>>
        }
    }

    //pub fn get_all() -> Result<Self, Error> {
    //ShowOptions::new()
    //.global()
    //.window()
    //.build()
    ////.into_tmux_bin_command()
    ////.output()?
    //.to_string()
    //.parse()
    //}

    // XXX: bitmask is overkill now, mb later use for multiple select
    // NOTE: not allows selective get by bitmask
    //#[cfg(feature = "tmux_1_7")]
    //pub fn get(bitflags: u128) -> Result<Self, Error> {
    //let selected_option = WINDOW_OPTIONS
    //.iter()
    //.filter(|t| bitflags == t.3)
    //.map(|t| t.0.to_string())
    //.collect::<Vec<String>>()
    //.join(" ");
    //ShowOptions::new()
    //.server()
    //.option(&selected_option)
    //.build()
    ////.into_tmux_bin_command()
    ////.output()?
    //.to_string()
    //.parse()
    //}

    // allows selective set by bitmask
    //pub fn set(&self, bitflags: u128) -> Result<(), Error> {
    //for selected_option in WINDOW_OPTIONS.iter().filter(|t| bitflags & t.3 == t.3) {
    //if let Some(selected_value) = selected_option.2(&self) {
    //SetOption::new()
    //.server()
    //.option(selected_option.0)
    //.value(&selected_value)
    //.build();
    ////.into_tmux_bin_command()
    ////.output()?;
    //}
    //}
    //Ok(())
    //}
}

// command_alias[0] = "alias1" => command_alias["alias1"]
// command_alias[1] = "alias2" => command_alias["alias2"]
// ...
// command_alias[n] = "aliasN" => command_alias["aliasN"]
// TODO: optimization, merge server, session, window, pane?
//impl FromStr for WindowOptions {
//type Err = Error;

//fn from_str(options: &str) -> Result<Self, Self::Err> {
//let mut window_options: WindowOptions = Default::default();
//let mut v: Vec<&str>;
//let mut arr: Vec<&str>;
//for option in options.lines() {
//v = option.trim().splitn(2, ' ').collect();
//arr = v[0].split(|c| c == '[' || c == ']').collect();
//for window_var in WINDOW_OPTIONS.iter() {
//if window_var.0 == arr[0] {
//window_var.1(
//&mut window_options,
//arr.get(1).and_then(|i| i.parse::<usize>().ok()),
//v.get(1).unwrap_or(&""),
//)
//}
//}
//}
//Ok(window_options)
//}
//}

//impl fmt::Display for WindowOptions {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//// pane option
//for var in WINDOW_OPTIONS.iter() {
//// if is set some - extract
//if let Some(ref v) = var.2(self) {
//writeln!(f, "{} {}", var.0, v)?;
//}
//}
//Ok(())
//}
//}

//#[derive(Default, Debug)]
//pub struct WindowOptionsBuilder<'a> {
//#[cfg(feature = "tmux_1_0")]
//pub aggressive_resize: Option<Switch>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
//pub allow_rename: Option<Switch>,
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
//pub alternate_screen: Option<Switch>,
//#[cfg(feature = "tmux_1_0")] // 0.8
//pub automatic_rename: Option<Switch>,
//#[cfg(feature = "tmux_1_9")]
//pub automatic_rename_format: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
//pub c0_change_interval: Option<usize>,
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
//pub c0_change_trigger: Option<usize>,
//#[cfg(feature = "tmux_1_0")]
//pub clock_mode_colour: Option<&'a str>,
//#[cfg(feature = "tmux_1_0")]
//pub clock_mode_style: Option<ClockModeStyle>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
//pub force_height: Option<usize>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
//pub force_width: Option<usize>,
//#[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
//pub layout_history_limit: Option<usize>,
//#[cfg(feature = "tmux_1_0")]
//pub main_pane_height: Option<usize>,
//#[cfg(feature = "tmux_1_0")]
//pub main_pane_width: Option<usize>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub mode_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub mode_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub mode_fg: Option<&'a str>,
//#[cfg(feature = "tmux_1_0")]
//pub mode_keys: Option<StatusKeys>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
//pub mode_mouse: Option<Switch>,
//#[cfg(feature = "tmux_1_9")]
//pub mode_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_0")]
//pub monitor_activity: Option<Switch>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
//pub monitor_content: Option<&'a str>,
//#[cfg(feature = "tmux_2_6")]
//pub monitor_bell: Option<Switch>,
//#[cfg(feature = "tmux_1_4")]
//pub monitor_silence: Option<usize>,
//#[cfg(feature = "tmux_1_4")]
//pub other_pane_height: Option<usize>,
//#[cfg(feature = "tmux_1_4")]
//pub other_pane_width: Option<usize>,
//#[cfg(feature = "tmux_2_0")]
//pub pane_active_border_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_6")]
//pub pane_base_index: Option<usize>,
//#[cfg(feature = "tmux_2_3")]
//pub pane_border_format: Option<&'a str>,
//#[cfg(feature = "tmux_2_3")]
//pub pane_border_status: Option<PaneBorderStatus>,
//#[cfg(feature = "tmux_2_0")]
//pub pane_border_style: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
//pub remain_on_exit: Option<Switch>,
//#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
//pub synchronize_panes: Option<Switch>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
//pub utf8: Option<Switch>,
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
//pub window_active_style: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_bell_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_bell_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_bell_fg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_content_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_content_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_content_fg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_activity_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_activity_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
//pub window_status_activity_fg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub window_status_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub window_status_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub window_status_fg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub window_status_current_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub window_status_current_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
//pub window_status_current_fg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
//pub window_status_alert_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
//pub window_status_alert_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
//pub window_status_alert_fg: Option<&'a str>,
//#[cfg(feature = "tmux_1_9")]
//pub window_status_activity_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_9")]
//pub window_status_bell_style: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
//pub window_status_content_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_2")]
//pub window_status_current_format: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//pub window_status_last_attr: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//pub window_status_last_bg: Option<&'a str>,
//#[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
//pub window_status_last_fg: Option<&'a str>,
//#[cfg(feature = "tmux_1_9")]
//pub window_status_current_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_2")]
//pub window_status_format: Option<&'a str>,
//#[cfg(feature = "tmux_1_9")]
//pub window_status_last_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_7")]
//pub window_status_separator: Option<&'a str>,
//#[cfg(feature = "tmux_1_9")]
//pub window_status_style: Option<&'a str>,
//#[cfg(feature = "tmux_2_9")]
//pub window_size: Option<WindowSize>,
//#[cfg(feature = "tmux_1_2")]
//pub word_separators: Option<&'a str>,
//#[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
//pub window_style: Option<&'a str>,
//#[cfg(feature = "tmux_1_7")]
//pub wrap_search: Option<Switch>,
//#[cfg(feature = "tmux_1_0")]
//pub xterm_keys: Option<Switch>,
//}

impl WindowOptions {
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
    pub fn automatic_rename_format(mut self, automatic_rename_format: Option<String>) -> Self {
        self.automatic_rename_format = automatic_rename_format;
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_interval(mut self, c0_change_interval: Option<String>) -> Self {
        self.c0_change_interval = c0_change_interval;
        self
    }

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    pub fn c0_change_trigger(mut self, c0_change_trigger: Option<String>) -> Self {
        self.c0_change_trigger = c0_change_trigger;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn clock_mode_colour(mut self, clock_mode_colour: Option<String>) -> Self {
        self.clock_mode_colour = clock_mode_colour;
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

    #[cfg(feature = "tmux_1_9")]
    pub fn main_pane_height(mut self, main_pane_height: Option<usize>) -> Self {
        self.main_pane_height = main_pane_height;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn main_pane_width(mut self, main_pane_width: Option<usize>) -> Self {
        self.main_pane_width = main_pane_width;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_attr(mut self, mode_attr: Option<String>) -> Self {
        self.mode_attr = mode_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_bg(mut self, mode_bg: Option<String>) -> Self {
        self.mode_bg = mode_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    pub fn mode_fg(mut self, mode_fg: Option<String>) -> Self {
        self.mode_fg = mode_fg;
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn mode_keys(mut self, mode_keys: Option<StatusKeys>) -> Self {
        self.mode_keys = mode_keys;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    pub fn mode_mouse(mut self, mode_mouse: Option<Switch>) -> Self {
        self.mode_mouse = mode_mouse;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn mode_style(mut self, mode_style: Option<String>) -> Self {
        self.mode_style = mode_style;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn monitor_activity(mut self, monitor_activity: Option<Switch>) -> Self {
        self.monitor_activity = monitor_activity;
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    pub fn monitor_content(mut self, monitor_content: Option<usize>) -> Self {
        self.monitor_content = monitor_content;
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

    #[cfg(feature = "tmux_2_0")]
    pub fn pane_active_border_style(mut self, pane_active_border_style: Option<String>) -> Self {
        self.pane_active_border_style = pane_active_border_style;
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn pane_base_index(mut self, pane_base_index: Option<usize>) -> Self {
        self.pane_base_index = pane_base_index;
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_format(mut self, pane_border_format: Option<String>) -> Self {
        self.pane_border_format = pane_border_format;
        self
    }

    #[cfg(feature = "tmux_2_3")]
    pub fn pane_border_status(mut self, pane_border_status: Option<PaneBorderStatus>) -> Self {
        self.pane_border_status = pane_border_status;
        self
    }

    #[cfg(feature = "tmux_2_0")]
    pub fn pane_border_style(mut self, pane_border_style: Option<String>) -> Self {
        self.pane_border_style = pane_border_style;
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
    pub fn utf8(mut self, utf8: Option<String>) -> Self {
        self.utf8 = utf8;
        self
    }

    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    pub fn window_active_style(mut self, window_active_style: Option<String>) -> Self {
        self.window_active_style = window_active_style;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_attr(mut self, window_status_bell_attr: Option<Switch>) -> Self {
        self.window_status_bell_attr = window_status_bell_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_bg(mut self, window_status_bell_bg: Option<String>) -> Self {
        self.window_status_bell_bg = window_status_bell_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bell_fg(mut self, window_status_bell_fg: Option<String>) -> Self {
        self.window_status_bell_fg = window_status_bell_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_attr(
        mut self,
        window_status_content_attr: Option<String>,
    ) -> Self {
        self.window_status_content_attr = window_status_content_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_bg(mut self, window_status_content_bg: Option<String>) -> Self {
        self.window_status_content_bg = window_status_content_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_content_fg(mut self, window_status_content_fg: Option<String>) -> Self {
        self.window_status_content_fg = window_status_content_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_attr(
        mut self,
        window_status_activity_attr: Option<String>,
    ) -> Self {
        self.window_status_activity_attr = window_status_activity_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_bg(mut self, window_status_activity_bg: Option<String>) -> Self {
        self.window_status_activity_bg = window_status_activity_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_activity_fg(mut self, window_status_activity_fg: Option<String>) -> Self {
        self.window_status_activity_fg = window_status_activity_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_attr(mut self, window_status_attr: Option<String>) -> Self {
        self.window_status_attr = window_status_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_bg(mut self, window_status_bg: Option<String>) -> Self {
        self.window_status_bg = window_status_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_fg(mut self, window_status_fg: Option<String>) -> Self {
        self.window_status_fg = window_status_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_attr(
        mut self,
        window_status_current_attr: Option<String>,
    ) -> Self {
        self.window_status_current_attr = window_status_current_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_bg(mut self, window_status_current_bg: Option<String>) -> Self {
        self.window_status_current_bg = window_status_current_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_current_fg(mut self, window_status_current_fg: Option<String>) -> Self {
        self.window_status_current_fg = window_status_current_fg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_alert_attr(mut self, window_status_alert_attr: Option<String>) -> Self {
        self.window_status_alert_attr = window_status_alert_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_alert_bg(mut self, window_status_alert_bg: Option<String>) -> Self {
        self.window_status_alert_bg = window_status_alert_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    pub fn window_status_alert_fg(mut self, window_status_alert_fg: Option<String>) -> Self {
        self.window_status_alert_fg = window_status_alert_fg;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_activity_style(
        mut self,
        window_status_activity_style: Option<String>,
    ) -> Self {
        self.window_status_activity_style = window_status_activity_style;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_bell_style(mut self, window_status_bell_style: Option<String>) -> Self {
        self.window_status_bell_style = window_status_bell_style;
        self
    }

    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    pub fn window_status_content_style(
        mut self,
        window_status_content_style: Option<String>,
    ) -> Self {
        self.window_status_content_style = window_status_content_style;
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_current_format(
        mut self,
        window_status_current_format: Option<String>,
    ) -> Self {
        self.window_status_current_format = window_status_current_format;
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_attr(mut self, window_status_last_attr: Option<String>) -> Self {
        self.window_status_last_attr = window_status_last_attr;
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_bg(mut self, window_status_last_bg: Option<String>) -> Self {
        self.window_status_last_bg = window_status_last_bg;
        self
    }

    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    pub fn window_status_last_fg(mut self, window_status_last_fg: Option<String>) -> Self {
        self.window_status_last_fg = window_status_last_fg;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_current_style(
        mut self,
        window_status_current_style: Option<String>,
    ) -> Self {
        self.window_status_current_style = window_status_current_style;
        self
    }
    #[cfg(feature = "tmux_1_2")]
    pub fn window_status_format(mut self, window_status_format: Option<String>) -> Self {
        self.window_status_format = window_status_format;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_last_style(mut self, window_status_last_style: Option<String>) -> Self {
        self.window_status_last_style = window_status_last_style;
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn window_status_separator(mut self, window_status_separator: Option<String>) -> Self {
        self.window_status_separator = window_status_separator;
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn window_status_style(mut self, window_status_style: Option<String>) -> Self {
        self.window_status_style = window_status_style;
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
    pub fn window_style(mut self, window_style: Option<String>) -> Self {
        self.window_style = window_style;
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
