use crate::{
    Action, Activity, DetachOnDestroy, Status, StatusJustify, StatusKeys, StatusPosition, Switch,
};

//activity-action [any | none | current | other]
#[cfg(feature = "tmux_2_6")]
pub const ACTIVITY_ACTION: &str = "activity-action";
//assume-paste-time milliseconds
#[cfg(feature = "tmux_1_8")]
pub const ASSUME_PASTE_TIME: &str = "assume-paste-time";
//base-index index
#[cfg(feature = "tmux_1_0")]
pub const BASE_INDEX: &str = "base-index";
//bell-action [any | none | current | other]
// tmux 1.0: bell-action [any | none | other]
#[cfg(feature = "tmux_1_0")]
pub const BELL_ACTION: &str = "bell-action";
//bell-on-alert [on | off]
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
pub const BELL_ON_ALERT: &str = "bell-on-alert";
//buffer-limit limit
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
pub const BUFFER_LIMIT: &str = "buffer-limit";
//default-command shell-command
#[cfg(feature = "tmux_1_0")]
pub const DEFAULT_COMMAND: &str = "default-command";
//default-shell path
#[cfg(feature = "tmux_1_0")]
pub const DEFAULT_SHELL: &str = "default-shell";
//default-path path
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const DEFAULT_PATH: &str = "default-path";
// default-terminal terminal
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const DEFAULT_TERMINAL: &str = "default-terminal";
//default-size XxY
#[cfg(feature = "tmux_2_9")]
pub const DEFAULT_SIZE: &str = "default-size";
//destroy-unattached [on | off]
#[cfg(feature = "tmux_1_4")]
pub const DESTROY_UNATTACHED: &str = "destroy-unattached";
//detach-on-destroy [on | off]
// tmux ^3.2 detach-on-destroy [on | off | no-detached]
#[cfg(feature = "tmux_1_4")]
pub const DETACH_ON_DESTROY: &str = "detach-on-destroy";
//display-panes-active-colour colour
#[cfg(feature = "tmux_1_2")]
pub const DISPLAY_PANES_ACTIVE_COLOUR: &str = "display-panes-active-colour";
//display-panes-colour colour
#[cfg(feature = "tmux_1_0")]
pub const DISPLAY_PANES_COLOUR: &str = "display-panes-colour";
//display-panes-time time
#[cfg(feature = "tmux_1_0")]
pub const DISPLAY_PANES_TIME: &str = "display-panes-time";
//display-time time
#[cfg(feature = "tmux_1_0")]
pub const DISPLAY_TIME: &str = "display-time";
//history-limit lines
#[cfg(feature = "tmux_1_0")]
pub const HISTORY_LIMIT: &str = "history-limit";
//key-table key-table
#[cfg(feature = "tmux_2_2")]
pub const KEY_TABLE: &str = "key-table";
//lock-after-time number
#[cfg(feature = "tmux_1_0")]
pub const LOCK_AFTER_TIME: &str = "lock-after-time";
//lock-command shell-command
#[cfg(feature = "tmux_1_1")]
pub const LOCK_COMMAND: &str = "lock-command";
//lock-server [on | off]
#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
pub const LOCK_SERVER: &str = "lock-server";

//message-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_ATTR: &str = "message-attr";
//message-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_BG: &str = "message-bg";
//message-command-attr attributes
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const MESSAGE_COMMAND_ATTR: &str = "message-command-attr";
//message-command-bg colour
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const MESSAGE_COMMAND_BG: &str = "message-command-bg";
//message-command-fg colour
#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
pub const MESSAGE_COMMAND_FG: &str = "message-command-fg";
//message-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_FG: &str = "message-fg";
//message-command-style style
#[cfg(feature = "tmux_1_9")]
pub const MESSAGE_COMMAND_STYLE: &str = "message-command-style";
//message-limit number
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub const MESSAGE_LIMIT: &str = "message-limit";
//message-style style
#[cfg(feature = "tmux_1_9")]
pub const MESSAGE_STYLE: &str = "message-style";
//mouse-resize-pane [on | off]
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub const MOUSE_RESIZE_PANE: &str = "mouse-resize-pane";
//mouse-select-pane [on | off]
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub const MOUSE_SELECT_PANE: &str = "mouse-select-pane";
//mouse-select-window [on | off]
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub const MOUSE_SELECT_WINDOW: &str = "mouse-select-window";
//mouse [on | off]
#[cfg(feature = "tmux_2_1")]
pub const MOUSE: &str = "mouse";
// mouse-utf8 [on | off]
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
pub const MOUSE_UTF8: &str = "mouse-utf8";
//pane-active-border-bg colour
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_ACTIVE_BORDER_BG: &str = "pane-active-border-bg";
//pane-active-border-fg colour
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_ACTIVE_BORDER_FG: &str = "pane-active-border-fg";
//pane-border-bg colour
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_BORDER_BG: &str = "pane-border-bg";
//pane-border-fg colour
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_BORDER_FG: &str = "pane-border-fg";
//pane-active-border-style style
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const PANE_ACTIVE_BORDER_STYLE: &str = "pane-active-border-style";
//pane-border-style style
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const PANE_BORDER_STYLE: &str = "pane-border-style";
//prefix key
#[cfg(feature = "tmux_1_0")]
pub const PREFIX: &str = "prefix";
//prefix2 key
#[cfg(feature = "tmux_1_6")]
pub const PREFIX2: &str = "prefix2";
//renumber-windows [on | off]
#[cfg(feature = "tmux_1_7")]
pub const RENUMBER_WINDOWS: &str = "renumber-windows";
//repeat-time time
#[cfg(feature = "tmux_1_0")]
pub const REPEAT_TIME: &str = "repeat-time";
//set-remain-on-exit [on | off]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
pub const SET_REMAIN_ON_EXIT: &str = "set-remain-on-exit";
//set-titles [on | off]
#[cfg(feature = "tmux_1_0")]
pub const SET_TITLES: &str = "set-titles";
//set-titles-string string
#[cfg(feature = "tmux_1_0")]
pub const SET_TITLES_STRING: &str = "set-titles-string";
//silence-action [any | none | current | other]
#[cfg(feature = "tmux_2_6")]
pub const SILENCE_ACTION: &str = "silence-action";
//status [off | on | 2 | 3 | 4 | 5]
//tmux 1.0: status [off | on]
#[cfg(feature = "tmux_1_0")]
pub const STATUS: &str = "status";
//status-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_ATTR: &str = "status-attr";
//status-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_BG: &str = "status-bg";
//status-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_FG: &str = "status-fg";
//status-format[] format
#[cfg(feature = "tmux_2_9")]
pub const STATUS_FORMAT: &str = "status-format";
//status-interval interval
#[cfg(feature = "tmux_1_0")]
pub const STATUS_INTERVAL: &str = "status-interval";
//status-justify [left | centre | right]
#[cfg(feature = "tmux_1_0")]
pub const STATUS_JUSTIFY: &str = "status-justify";
//status-keys [vi | emacs]
#[cfg(feature = "tmux_1_0")]
pub const STATUS_KEYS: &str = "status-keys";

//status-left string
#[cfg(feature = "tmux_1_0")]
pub const STATUS_LEFT: &str = "status-left";
//status-left-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_LEFT_ATTR: &str = "status-left-attr";
//status-left-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_LEFT_BG: &str = "status-left-bg";
//status-left-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_LEFT_FG: &str = "status-left-fg";
//status-left-length length
#[cfg(feature = "tmux_1_0")]
pub const STATUS_LEFT_LENGTH: &str = "status-left-length";
//status-left-style style
#[cfg(feature = "tmux_1_9")]
pub const STATUS_LEFT_STYLE: &str = "status-left-style";
//status-position [top | bottom]
#[cfg(feature = "tmux_1_7")]
pub const STATUS_POSITION: &str = "status-position";
//status-right string
#[cfg(feature = "tmux_1_0")]
pub const STATUS_RIGHT: &str = "status-right";
//status-right-attr attributes
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_RIGHT_ATTR: &str = "status-right-attr";
//status-right-bg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_RIGHT_BG: &str = "status-right-bg";
//status-right-fg colour
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_RIGHT_FG: &str = "status-right-fg";
//status-right-length length
#[cfg(feature = "tmux_1_0")]
pub const STATUS_RIGHT_LENGTH: &str = "status-right-length";
//status-right-style style
#[cfg(feature = "tmux_1_9")]
pub const STATUS_RIGHT_STYLE: &str = "status-right-style";
//status-style style
#[cfg(feature = "tmux_1_9")]
pub const STATUS_STYLE: &str = "status-style";
//status-utf8 [on | off]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub const STATUS_UTF8: &str = "status-utf8";
//terminal-overrides string
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const TERMINAL_OVERRIDES: &str = "terminal-overrides";
//update-environment[] variable
#[cfg(feature = "tmux_1_0")]
pub const UPDATE_ENVIRONMENT: &str = "update-environment";
//user-keys
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
pub const USER_KEYS: &str = "user-keys";
//visual-activity [on | off | both]
//tmux 1.0: visual-activity [on | off]
#[cfg(feature = "tmux_1_0")]
pub const VISUAL_ACTIVITY: &str = "visual-activity";
//visual-bell [on | off | both]
//tmux 1.0: visual-bell [on | off]
#[cfg(feature = "tmux_1_0")]
pub const VISUAL_BELL: &str = "visual-bell";
//visual-content [on | off]
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const VISUAL_CONTENT: &str = "visual-content";
//visual-silence [on | off | both]
#[cfg(feature = "tmux_1_4")]
pub const VISUAL_SILENCE: &str = "visual-silence";
//word-separators string
#[cfg(feature = "tmux_1_6")]
pub const WORD_SEPARATORS: &str = "word-separators";


/// tmux ^2.6:
/// ```text
/// activity-action other
/// ```
#[cfg(feature = "tmux_2_6")]
pub const ACTIVITY_ACTION_DEFAULT: Action = Action::Other;

/// tmux ^1.8:
/// ```text
/// assume-paste-time 1
/// ```
#[cfg(feature = "tmux_1_8")]
pub const ASSUME_PASTE_TIME_DEFAULT: usize = 1;

/// tmux ^1.0:
/// ```text
/// base-index 0
/// ```
#[cfg(feature = "tmux_1_0")]
pub const BASE_INDEX_DEFAULT: usize = 0;

/// tmux ^1.0:
/// ```text
/// bell-action none
/// ```
#[cfg(feature = "tmux_1_0")]
pub const BELL_ACTION_DEFAULT: Action = Action::Any;

/// tmux ^1.5 v2.6:
/// ```text
/// bell-on-alert off
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
pub const BELL_ON_ALERT_DEFAULT: Switch = Switch::Off;

/// tmux ^1.0 v1.4:
/// ```text
/// buffer-limit 20
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
pub const BUFFER_LIMIT_DEFAULT: usize = 20;

/// tmux ^1.0:
/// ```text
/// default-command
/// ```
#[cfg(feature = "tmux_1_0")]
pub const DEFAULT_COMMAND_DEFAULT: &str = "";

/// tmux ^1.0:
/// ```text
/// default-shell /bin/bash
/// ```
// _PATH_BSHELL
#[cfg(feature = "tmux_1_0")]
pub const DEFAULT_SHELL_DEFAULT: &str = "/bin/bash";

/// tmux ^1.0 v2.1:
/// ```text
/// default-path ""
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const DEFAULT_PATH_DEFAULT: &str = "";

/// tmux ^2.9:
/// ```text
/// default-size 80x24
/// ```
#[cfg(feature = "tmux_2_9")]
pub const DEFAULT_SIZE_DEFAULT: (usize, usize) = (80, 24);

/// tmux ^1.0 v2.1:
/// ```text
/// default-terminal screen
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
pub const DEFAULT_TERMINAL_DEFAULT: &str = "screen";

/// tmux ^1.4:
/// ```text
/// destroy-unattached off
/// ```
#[cfg(feature = "tmux_1_4")]
pub const DESTROY_UNATTACHED_DEFAULT: Switch = Switch::Off;

/// tmux ^1.4:
/// ```text
/// detach-on-destroy on
/// ```
#[cfg(feature = "tmux_1_4")]
pub const DETACH_ON_DESTROY_DEFAULT: DetachOnDestroy = DetachOnDestroy::On;

/// tmux ^1.2:
/// ```text
/// display-panes-active-colour red
/// ```
// 1
#[cfg(feature = "tmux_1_2")]
pub const DISPLAY_PANES_ACTIVE_COLOUR_DEFAULT: &str = "red";

/// tmux ^1.0:
/// ```text
/// display-panes-colour blue
/// ```
// 4
#[cfg(feature = "tmux_1_0")]
pub const DISPLAY_PANES_COLOUR_DEFAULT: &str = "blue";

/// tmux ^1.0:
/// ```text
/// display-panes-time 1000
/// ```
#[cfg(feature = "tmux_1_0")]
pub const DISPLAY_PANES_TIME_DEFAULT: usize = 1000;

/// tmux ^1.0:
/// ```text
/// display-time 750
/// ```
#[cfg(feature = "tmux_1_0")]
pub const DISPLAY_TIME_DEFAULT: usize = 750;

/// tmux ^1.0:
/// ```text
/// history-limit 2000
/// ```
#[cfg(feature = "tmux_1_0")]
pub const HISTORY_LIMIT_DEFAULT: usize = 2000;

/// tmux ^2.2:
/// ```text
/// key-table root
/// ```
#[cfg(feature = "tmux_2_2")]
pub const KEY_TABLE_DEFAULT: &str = "root";

/// tmux ^1.0:
/// ```text
/// lock-after-time 0
/// ```
#[cfg(feature = "tmux_1_0")]
pub const LOCK_AFTER_TIME_DEFAULT: usize = 0;

/// tmux ^1.1:
/// ```text
/// lock-command "lock -np"
/// ```
#[cfg(feature = "tmux_1_1")]
pub const LOCK_COMMAND_DEFAULT: &str = "lock -np";

/// tmux ^1.1 v2.1:
/// ```text
/// lock-server on
/// ```
#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
pub const LOCK_SERVER_DEFAULT: Switch = Switch::On;

/// tmux ^1.0 v1.9:
/// ```text
/// message-attr none
/// ```
// 0
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// message-bg yellow
/// ```
// 3
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_BG_DEFAULT: &str = "yellow";

/// tmux ^1.0 v1.9:
/// ```text
/// message-command-attr none
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_COMMAND_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// message-command-bg black
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_COMMAND_BG_DEFAULT: &str = "black";

/// tmux ^1.0 v1.9:
/// ```text
/// message-command-fg yellow
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_COMMAND_FG_DEFAULT: &str = "yellow";

/// tmux ^1.0 v1.9:
/// ```text
/// message-fg black
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const MESSAGE_FG_DEFAULT: &str = "black";

/// tmux ^1.9:
/// ```text
/// message-command-style bg=black,fg=yellow
/// ```
#[cfg(feature = "tmux_1_9")]
pub const MESSAGE_COMMAND_STYLE_DEFAULT: &str = "bg=black,fg=yellow";

/// tmux ^1.2 v2.0:
/// ```text
/// message-limit number
/// ```
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
pub const MESSAGE_LIMIT_DEFAULT: usize = 20;

/// tmux ^1.9:
/// ```text
/// message-style bg=yellow,fg=black
/// ```
#[cfg(feature = "tmux_1_9")]
pub const MESSAGE_STYLE_DEFAULT: &str = "bg=yellow,fg=black";

/// tmux ^1.5 v2.1:
/// ```text
/// mouse-resize-pane off
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub const MOUSE_RESIZE_PANE_DEFAULT: Switch = Switch::Off;

/// tmux ^1.5 v2.1:
/// ```text
/// mouse-select-pane off
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub const MOUSE_SELECT_PANE_DEFAULT: Switch = Switch::Off;

/// tmux ^1.5 v2.1:
/// ```text
/// mouse-select-window off
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
pub const MOUSE_SELECT_WINDOW_DEFAULT: Switch = Switch::Off;

/// tmux ^2.1:
/// ```text
/// mouse on
/// ```
#[cfg(feature = "tmux_2_1")]
pub const MOUSE_DEFAULT: Switch = Switch::Off;

/// tmux ^1.5 v2.2:
/// ```text
/// mouse-utf8 off
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
pub const MOUSE_UTF8_DEFAULT: Switch = Switch::Off;

/// tmux ^1.2 v1.9:
/// ```text
/// pane-active-border-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_ACTIVE_BORDER_BG_DEFAULT: &str = "default";

/// tmux ^1.2 v1.9:
/// ```text
/// pane-active-border-fg green
/// ```
// 2
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_ACTIVE_BORDER_FG_DEFAULT: &str = "green";

/// tmux ^1.2 v1.9:
/// ```text
/// pane-border-bg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_BORDER_BG_DEFAULT: &str = "default";

/// tmux ^1.2 v1.9:
/// ```text
/// pane-border-fg default
/// ```
// 8
#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
pub const PANE_BORDER_FG_DEFAULT: &str = "default";

/// tmux ^1.9 v2.0:
/// ```text
/// pane-active-border-style fg=green
/// ```
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const PANE_ACTIVE_BORDER_STYLE_DEFAULT: &str = "fg=green";

/// tmux ^1.9 v2.0:
/// ```text
/// pane-border-style default
/// ```
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const PANE_BORDER_STYLE_DEFAULT: &str = "default";

/// tmux ^1.0:
/// ```text
/// prefix C-b
/// ```
// FIXME
// \002
// key_string_lookup_string
#[cfg(feature = "tmux_1_0")]
pub const PREFIX_DEFAULT: &str = "C-b";

/// tmux ^1.6:
/// ```text
/// prefix2 Invalid#1fff00000000
/// ```
// FIXME
// KEYC_NONE
// 0xfff
#[cfg(feature = "tmux_1_6")]
pub const PREFIX2_DEFAULT: &str = "Invalid#1fff00000000";

/// tmux ^1.7:
/// ```text
/// renumber-windows off
/// ```
#[cfg(feature = "tmux_1_7")]
pub const RENUMBER_WINDOWS_DEFAULT: Switch = Switch::Off;

/// tmux ^1.0:
/// ```text
/// repeat-time 500
/// ```
#[cfg(feature = "tmux_1_0")]
pub const REPEAT_TIME_DEFAULT: usize = 500;

/// tmux ^1.0 v2.4:
/// ```text
/// set-remain-on-exit off
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
pub const SET_REMAIN_ON_EXIT_DEFAULT: Switch = Switch::Off;

/// tmux ^1.0:
/// ```text
/// set-titles off
/// ```
#[cfg(feature = "tmux_1_0")]
pub const SET_TITLES_DEFAULT: Switch = Switch::Off;

/// tmux ^1.0:
/// ```text
/// set-titles-string "#S:#I:#W - \"#T\" #{session_alerts}"
/// ```
#[cfg(feature = "tmux_1_0")]
pub const SET_TITLES_STRING_DEFAULT: &str = "#S:#I:#W - \"#T\" #{session_alerts}";

/// tmux ^2.6:
/// ```text
/// silence-action other
/// ```
#[cfg(feature = "tmux_2_6")]
pub const SILENCE_ACTION_DEFAULT: Action = Action::Other;

/// tmux ^1.0:
/// ```text
/// status on
/// ```
#[cfg(feature = "tmux_1_0")]
pub const STATUS_DEFAULT: Status = Status::On;

/// tmux ^1.0 v1.9:
/// ```text
/// status-attr none
/// ```
// 0
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// status-bg green
/// ```
// 2
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_BG_DEFAULT: &str = "green";

/// tmux ^1.0 v1.9:
/// ```text
/// status-fg black
/// ```
// 0
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_FG_DEFAULT: &str = "black";

/// tmux ^2.9:
/// ```text
/// status-format[0] "#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]"
/// status-format[1] "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"
/// ```
#[cfg(feature = "tmux_2_9")]
pub const STATUS_FORMAT_DEFAULT: [&str; 2] = [ 
    "#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]",
    "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"
];

/// tmux ^1.0:
/// ```text
/// status-interval 15
/// ```
#[cfg(feature = "tmux_1_0")]
pub const STATUS_INTERVAL_DEFAULT: usize = 15;

/// tmux ^1.0:
/// ```text
/// status-justify left
/// ```
// 0
#[cfg(feature = "tmux_1_0")]
pub const STATUS_JUSTIFY_DEFAULT: StatusJustify = StatusJustify::Left;

/// tmux ^1.0:
/// ```text
/// status-keys emacs
/// ```
#[cfg(feature = "tmux_1_0")]
pub const STATUS_KEYS_DEFAULT: StatusKeys = StatusKeys::Emacs;

/// tmux ^3.2:
/// ```text
/// status-left "[#{session_name}] "
/// ```
/// 
/// tmux ^1.0 v3.2:
/// ```text
/// status-left "[#S] "
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_2")))]
pub const STATUS_LEFT_DEFAULT: &str = "[#S] ";
#[cfg(feature = "tmux_3_2")]
pub const STATUS_LEFT_DEFAULT: &str = "[#{session_name}] ";

/// tmux ^1.0 v1.9:
/// ```text
/// status-left none
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_LEFT_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// status-left default
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_LEFT_BG_DEFAULT: &str = "default";

/// ```text
/// tmux ^1.0 v1.9:
/// status-left default
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_LEFT_FG_DEFAULT: &str = "default";

/// tmux ^1.0:
/// ```text
/// status-left-length 10
/// ```
#[cfg(feature = "tmux_1_0")]
pub const STATUS_LEFT_LENGTH_DEFAULT: usize = 10;

/// tmux ^1.9:
/// ```text
/// status-left-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const STATUS_LEFT_STYLE_DEFAULT: &str = "default";

/// tmux ^1.7:
/// ```text
/// status-position bottom
/// ```
// 1
#[cfg(feature = "tmux_1_7")]
pub const STATUS_POSITION_DEFAULT: StatusPosition = StatusPosition::Bottom;

/// tmux ^1.0:
/// ```text
/// status-right "#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\"#{=21:pane_title}\" %H:%M %d-%b-%y"
/// ```
#[cfg(feature = "tmux_1_0")]
pub const STATUS_RIGHT_DEFAULT: &str = "#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\"#{=21:pane_title}\" %H:%M %d-%b-%y";

/// tmux ^1.0 v1.9:
/// ```text
/// status-right-attr none
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_RIGHT_ATTR_DEFAULT: &str = "none";

/// tmux ^1.0 v1.9:
/// ```text
/// status-right-bg default
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_RIGHT_BG_DEFAULT: &str = "default";

/// tmux ^1.0 v1.9:
/// ```text
/// status-right-fg default
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
pub const STATUS_RIGHT_FG_DEFAULT: &str = "default";

/// tmux ^1.0:
/// ```text
/// status-right-length 40
/// ```
#[cfg(feature = "tmux_1_0")]
pub const STATUS_RIGHT_LENGTH_DEFAULT: usize = 40;

/// tmux ^1.9:
/// ```text
/// status-right-style default
/// ```
#[cfg(feature = "tmux_1_9")]
pub const STATUS_RIGHT_STYLE_DEFAULT: &str = "default";

/// tmux ^1.9:
/// ```text
/// status-style fg=colour247,bg=#282c34
/// ```
#[cfg(feature = "tmux_1_9")]
pub const STATUS_STYLE_DEFAULT: &str = "bg=green,fg=black";


/// tmux ^1.0 v2.2:
/// ```text
/// status-utf8 off 
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
pub const STATUS_UTF8_DEFAULT: Switch = Switch::Off;


/// tmux ^1.0 v2.0:
/// ```text
/// terminal-overrides "*256col*:colors=256,xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT" 
/// ```
#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_9")))]
pub const TERMINAL_OVERRIDES_DEFAULT: &str = "*88col*:colors=88,*256col*:colors=256xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cc=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Cs=\\E[%p1%d q:Csr=\\E[2 q";
#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
pub const TERMINAL_OVERRIDES_DEFAULT: &str = "*256col*:colors=256,xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT";
#[cfg(feature = "tmux_2_0")]
pub const TERMINAL_OVERRIDES_DEFAULT: &str = "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT";

/// tmux ^1.0:
/// ```text
/// update-environment[0] DISPLAY
/// update-environment[1] KRB5CCNAME
/// update-environment[2] SSH_ASKPASS
/// update-environment[3] SSH_AUTH_SOCK
/// update-environment[4] SSH_AGENT_PID
/// update-environment[5] SSH_CONNECTION
/// update-environment[6] WINDOWID
/// update-environment[7] XAUTHORITY
/// ```
#[cfg(feature = "tmux_1_0")]
pub const UPDATE_ENVIRONMENT_DEFAULT: [&str; 8] = [
        "DISPLAY",
        "KRB5CCNAME",
        "SSH_ASKPASS",
        "SSH_AUTH_SOCK",
        "SSH_AGENT_PID",
        "SSH_CONNECTION",
        "WINDOWID",
        "XAUTHORITY",
];

/// tmux ^2.6 v3.0:
/// ```text
/// user-keys[]
/// ```
#[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
pub const USER_KEYS_DEFAULT: [&str; 1] = [""];

/// tmux ^1.0:
/// ```text
/// visual-activity off
/// ```
#[cfg(feature = "tmux_1_0")]
pub const VISUAL_ACTIVITY_DEFAULT: Activity = Activity::Off;

/// tmux ^1.0:
/// ```text
/// visual-bell off
/// ```
// XXX: rename Activity = Visual? acc to tmux sources
#[cfg(feature = "tmux_1_0")]
pub const VISUAL_BELL_DEFAULT: Activity = Activity::Off;

/// tmux ^1.0 v2.0:
/// ```text
/// visual-content off
/// ```
#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
pub const VISUAL_CONTENT_DEFAULT: Switch = Switch::Off;

/// tmux ^1.4:
/// ```text
/// visual-silence off
/// ```
#[cfg(feature = "tmux_1_4")]
pub const VISUAL_SILENCE_DEFAULT: Activity = Activity::Off;

/// tmux ^1.6:
/// ```text
/// word-separators " "
/// ```
#[cfg(feature = "tmux_1_6")]
pub const WORD_SEPARATORS_DEFAULT: &str = " ";
