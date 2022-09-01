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
pub const MESSAGE_COMMAND_ATTR: &str = "message-command-style";
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

//pub user_options: Option<HashMap<String, String>>
