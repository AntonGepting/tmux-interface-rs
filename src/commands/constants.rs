/// `-1`
pub const _1_KEY: &str = "-1";
/// `-2`
pub const _2_KEY: &str = "-2";
/// `-8`
pub const _8_KEY: &str = "-8";

// constants for use as keys for subcommands
/// `-a`
pub const A_LOWERCASE_KEY: &str = "-a";
/// `-b`
pub const B_LOWERCASE_KEY: &str = "-b";
/// `-c`
pub const C_LOWERCASE_KEY: &str = "-c";
/// `-d`
pub const D_LOWERCASE_KEY: &str = "-d";
/// `-e`
pub const E_LOWERCASE_KEY: &str = "-e";
/// `-f`
pub const F_LOWERCASE_KEY: &str = "-f";
/// `-g`
pub const G_LOWERCASE_KEY: &str = "-g";
/// `-h`
pub const H_LOWERCASE_KEY: &str = "-h";
/// `-i`
pub const I_LOWERCASE_KEY: &str = "-i";
// `-j]`
//pub const J_LOWERCASE_KEY: &str = "-j";
/// `-k`
pub const K_LOWERCASE_KEY: &str = "-k";
/// `-l`
pub const L_LOWERCASE_KEY: &str = "-l";
/// `-m`
pub const M_LOWERCASE_KEY: &str = "-m";
/// `-n`
pub const N_LOWERCASE_KEY: &str = "-n";
/// `-o`
pub const O_LOWERCASE_KEY: &str = "-o";
/// `-p`
pub const P_LOWERCASE_KEY: &str = "-p";
/// `-q`
pub const Q_LOWERCASE_KEY: &str = "-q";
/// `-r`
pub const R_LOWERCASE_KEY: &str = "-r";
/// `-s`
pub const S_LOWERCASE_KEY: &str = "-s";
/// `-t`
pub const T_LOWERCASE_KEY: &str = "-t";
/// `-u`
pub const U_LOWERCASE_KEY: &str = "-u";
/// `-v`
pub const V_LOWERCASE_KEY: &str = "-v";
/// `-w`
pub const W_LOWERCASE_KEY: &str = "-w";
/// `-x`
pub const X_LOWERCASE_KEY: &str = "-x";
/// `-y`
pub const Y_LOWERCASE_KEY: &str = "-y";
// `-z`
//pub const Z_LOWERCASE_KEY: &str = "-z";

/// `-A`
pub const A_UPPERCASE_KEY: &str = "-A";
/// `-B`
//pub const B_UPPERCASE_KEY: &str = "-B";
// `-C`
pub const C_UPPERCASE_KEY: &str = "-C";
/// `-D`
pub const D_UPPERCASE_KEY: &str = "-D";
/// `-E`
pub const E_UPPERCASE_KEY: &str = "-E";
/// `-F`
pub const F_UPPERCASE_KEY: &str = "-F";
/// `-G`
pub const G_UPPERCASE_KEY: &str = "-G";
/// `-H`
pub const H_UPPERCASE_KEY: &str = "-H";
/// `-I`
pub const I_UPPERCASE_KEY: &str = "-I";
/// `-J`
pub const J_UPPERCASE_KEY: &str = "-J";
// `-K`
//pub const K_UPPERCASE_KEY: &str = "-K";
/// `-L`
pub const L_UPPERCASE_KEY: &str = "-L";
/// `-M`
pub const M_UPPERCASE_KEY: &str = "-M";
/// `-N`
pub const N_UPPERCASE_KEY: &str = "-N";
/// `-O`
pub const O_UPPERCASE_KEY: &str = "-O";
/// `-P`
pub const P_UPPERCASE_KEY: &str = "-P";
// `-Q`
//pub const Q_UPPERCASE_KEY: &str = "-Q";
/// `-R`
pub const R_UPPERCASE_KEY: &str = "-R";
/// `-S`
pub const S_UPPERCASE_KEY: &str = "-S";
/// `-T`
pub const T_UPPERCASE_KEY: &str = "-T";
/// `-U`
pub const U_UPPERCASE_KEY: &str = "-U";
/// `-V`
pub const V_UPPERCASE_KEY: &str = "-V";
// `-W`
//pub const W_UPPERCASE_KEY: &str = "-W";
/// `-X`
pub const X_UPPERCASE_KEY: &str = "-X";
// `-Y`
//pub const Y_UPPERCASE_KEY: &str = "-Y";
/// `-Z`
pub const Z_UPPERCASE_KEY: &str = "-Z";
/// `-CC`
pub const CC_UPPERCASE_KEY: &str = "-CC";

/// `tmux` - Default tmux binary name
pub const TMUX: &str = "tmux";

// XXX: mb CMD_ALIAS const

// tmux commands, which have no aliases:
//
// Buffers:
// - choose-buffer
//
// Clients and Sessions:
// - kill-server
// - kill-session
//
// Global and Session Environment:
// -
//
// Hooks:
// - set-hook
// - show-hooks
//
// Key bindings:
// - send-prefix
//
// Miscellaneous:
//
// - clock-mode
//
// Options:
// -
//
// Status Line:
// - command-prompt
// - display-menu
//
// Windows and panes:
// - choose-client
// - choose-tree
// - copy-mode

// Buffers
/// `choose-buffer`
pub const CHOOSE_BUFFER: &str = "choose-buffer";

/// `clear-history`
#[cfg(not(feature = "cmd_alias"))]
pub const CLEAR_HISTORY: &str = "clear-history";
/// `clearhist`
#[cfg(feature = "cmd_alias")]
pub const CLEAR_HISTORY: &str = "clearhist";

/// `delete-buffer`
#[cfg(not(feature = "cmd_alias"))]
pub const DELETE_BUFFER: &str = "delete-buffer";
/// `deleteb`
#[cfg(feature = "cmd_alias")]
pub const DELETE_BUFFER: &str = "deleteb";

/// `list-buffers`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_BUFFERS: &str = "list-buffers";
/// `lsb`
#[cfg(feature = "cmd_alias")]
pub const LIST_BUFFERS: &str = "lsb";

/// `load-buffer`
#[cfg(not(feature = "cmd_alias"))]
pub const LOAD_BUFFER: &str = "load-buffer";
/// `loadb`
#[cfg(feature = "cmd_alias")]
pub const LOAD_BUFFER: &str = "loadb";

/// `paste-buffer`
#[cfg(not(feature = "cmd_alias"))]
pub const PASTE_BUFFER: &str = "paste-buffer";
///  `pasteb`
#[cfg(feature = "cmd_alias")]
pub const PASTE_BUFFER: &str = "pasteb";

/// `save-buffer`
#[cfg(not(feature = "cmd_alias"))]
pub const SAVE_BUFFER: &str = "save-buffer";
/// `saveeb`
#[cfg(feature = "cmd_alias")]
pub const SAVE_BUFFER: &str = "saveb";

/// `set-buffer`
#[cfg(not(feature = "cmd_alias"))]
pub const SET_BUFFER: &str = "set-buffer";
/// `setb`
#[cfg(feature = "cmd_alias")]
pub const SET_BUFFER: &str = "setb";

/// `show-buffer`
#[cfg(not(feature = "cmd_alias"))]
pub const SHOW_BUFFER: &str = "show-buffer";
/// `showb`
#[cfg(feature = "cmd_alias")]
pub const SHOW_BUFFER: &str = "showb";

// Clients and sessions

/// `attach-session`
#[cfg(not(feature = "cmd_alias"))]
pub const ATTACH_SESSION: &str = "attach-session";
/// `attach`
#[cfg(feature = "cmd_alias")]
pub const ATTACH_SESSION: &str = "attach";

/// `detach-session`
#[cfg(not(feature = "cmd_alias"))]
pub const DETACH_CLIENT: &str = "detach-client";
/// `detach`
#[cfg(feature = "cmd_alias")]
pub const DETACH_CLIENT: &str = "detach";

/// `list-clients`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_CLIENTS: &str = "list-clients";
/// `lsc`
#[cfg(feature = "cmd_alias")]
pub const LIST_CLIENTS: &str = "lsc";

/// `list-commands`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_COMMANDS: &str = "list-commands";
///  `lscm`
#[cfg(feature = "cmd_alias")]
pub const LIST_COMMANDS: &str = "lscm";

/// `lock-client`
#[cfg(not(feature = "cmd_alias"))]
pub const LOCK_CLIENT: &str = "lock-client";
/// `lockc`
#[cfg(feature = "cmd_alias")]
pub const LOCK_CLIENT: &str = "lockc";

/// `new-session`
#[cfg(not(feature = "cmd_alias"))]
pub const NEW_SESSION: &str = "new-session";
/// `new`
#[cfg(feature = "cmd_alias")]
pub const NEW_SESSION: &str = "new";

/// `list-sessions`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_SESSIONS: &str = "list-sessions";
/// `ls`
#[cfg(feature = "cmd_alias")]
pub const LIST_SESSIONS: &str = "ls";

/// `refresh-client`
#[cfg(not(feature = "cmd_alias"))]
pub const REFRESH_CLIENT: &str = "refresh-client";
/// `refresh`
#[cfg(feature = "cmd_alias")]
pub const REFRESH_CLIENT: &str = "refresh";

/// `rename-session`
#[cfg(not(feature = "cmd_alias"))]
pub const RENAME_SESSION: &str = "rename-session";
/// `rename`
#[cfg(feature = "cmd_alias")]
pub const RENAME_SESSION: &str = "rename";

/// `show-messages`
#[cfg(not(feature = "cmd_alias"))]
pub const SHOW_MESSAGES: &str = "show-messages";
/// `showmsgs`
#[cfg(feature = "cmd_alias")]
pub const SHOW_MESSAGES: &str = "showmsgs";

/// `source-file`
#[cfg(not(feature = "cmd_alias"))]
pub const SOURCE_FILE: &str = "source-file";
/// `source`
#[cfg(feature = "cmd_alias")]
pub const SOURCE_FILE: &str = "source";

/// `start-server`
#[cfg(not(feature = "cmd_alias"))]
pub const START_SERVER: &str = "start-server";
/// `start`
#[cfg(feature = "cmd_alias")]
pub const START_SERVER: &str = "start";

/// `suspend-client`
#[cfg(not(feature = "cmd_alias"))]
pub const SUSPEND_CLIENT: &str = "suspend-client";
/// `suspendc`
#[cfg(feature = "cmd_alias")]
pub const SUSPEND_CLIENT: &str = "suspendc";

/// `switch-client`
#[cfg(not(feature = "cmd_alias"))]
pub const SWITCH_CLIENT: &str = "switch-client";
/// `switchc`
#[cfg(feature = "cmd_alias")]
pub const SWITCH_CLIENT: &str = "switchc";

/// `kill-server`
pub const KILL_SERVER: &str = "kill-server";

/// `kill-session`
pub const KILL_SESSION: &str = "kill-session";

/// `has-session`
#[cfg(not(feature = "cmd_alias"))]
pub const HAS_SESSION: &str = "has-session";
/// `has`
#[cfg(feature = "cmd_alias")]
pub const HAS_SESSION: &str = "has";

/// `lock-session`
#[cfg(not(feature = "cmd_alias"))]
pub const LOCK_SESSION: &str = "lock-session";
/// `locks`
#[cfg(feature = "cmd_alias")]
pub const LOCK_SESSION: &str = "locks";

// Global and session environment

/// `set-environment`
#[cfg(not(feature = "cmd_alias"))]
pub const SET_ENVIRONMENT: &str = "set-environment";
/// `setenv`
#[cfg(feature = "cmd_alias")]
pub const SET_ENVIRONMENT: &str = "setenv";

/// `show-environment`
#[cfg(not(feature = "cmd_alias"))]
pub const SHOW_ENVIRONMENT: &str = "show-environment";
/// `showenv`
#[cfg(feature = "cmd_alias")]
pub const SHOW_ENVIRONMENT: &str = "showenv";

// Hooks
/// `set-hook`
pub const SET_HOOK: &str = "set-hook";

/// `show-hooks`
pub const SHOW_HOOKS: &str = "show-hooks";

// Key Bindings

/// `bind-key`
#[cfg(not(feature = "cmd_alias"))]
pub const BIND_KEY: &str = "bind-key";
/// `bind`
#[cfg(feature = "cmd_alias")]
pub const BIND_KEY: &str = "bind";

/// `list-keys`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_KEYS: &str = "list-keys";
/// `lsk`
#[cfg(feature = "cmd_alias")]
pub const LIST_KEYS: &str = "lsk";

/// `send-keys`
#[cfg(not(feature = "cmd_alias"))]
pub const SEND_KEYS: &str = "send-keys";
/// `send`
#[cfg(feature = "cmd_alias")]
pub const SEND_KEYS: &str = "send";

/// `send-prefix`
pub const SEND_PREFIX: &str = "send-prefix";

/// `unbind-key`
#[cfg(not(feature = "cmd_alias"))]
pub const UNBIND_KEY: &str = "unbind-key";
/// `unbind`
#[cfg(feature = "cmd_alias")]
pub const UNBIND_KEY: &str = "unbind";

// Miscellaneous

/// `clock-mode`
pub const CLOCK_MODE: &str = "clock-mode";

/// `lock-server`
#[cfg(not(feature = "cmd_alias"))]
pub const LOCK_SERVER: &str = "lock-server";
/// `lock`
#[cfg(feature = "cmd_alias")]
pub const LOCK_SERVER: &str = "lock";

/// `run-shell`
#[cfg(not(feature = "cmd_alias"))]
pub const RUN_SHELL: &str = "run-shell";
/// `run`
#[cfg(feature = "cmd_alias")]
pub const RUN_SHELL: &str = "run";

/// `wait-for`
#[cfg(not(feature = "cmd_alias"))]
pub const WAIT_FOR: &str = "wait-for";
/// `wait`
#[cfg(feature = "cmd_alias")]
pub const WAIT_FOR: &str = "wait";

/// `if-shell`
#[cfg(not(feature = "cmd_alias"))]
pub const IF_SHELL: &str = "if-shell";
/// `if`
#[cfg(feature = "cmd_alias")]
pub const IF_SHELL: &str = "if";

// Options

/// `set-option`
#[cfg(not(feature = "cmd_alias"))]
pub const SET_OPTION: &str = "set-option";
/// `set`
#[cfg(feature = "cmd_alias")]
pub const SET_OPTION: &str = "set";

/// `set-window-option`
#[cfg(not(feature = "cmd_alias"))]
pub const SET_WINDOW_OPTION: &str = "set-window-option";
/// `setw`
#[cfg(feature = "cmd_alias")]
pub const SET_WINDOW_OPTION: &str = "setw";

/// `show-options`
#[cfg(not(feature = "cmd_alias"))]
pub const SHOW_OPTIONS: &str = "show-options";
/// `show`
#[cfg(feature = "cmd_alias")]
pub const SHOW_OPTIONS: &str = "show";

/// `show-window-options`
#[cfg(not(feature = "cmd_alias"))]
pub const SHOW_WINDOW_OPTIONS: &str = "show-window-options";
/// `showw`
#[cfg(feature = "cmd_alias")]
pub const SHOW_WINDOW_OPTIONS: &str = "showw";

// Status Line

/// `command-prompt`
pub const COMMAND_PROMPT: &str = "command-prompt";

/// `confirm-before`
#[cfg(not(feature = "cmd_alias"))]
pub const CONFIRM_BEFORE: &str = "confirm-before";
/// `confirm`
#[cfg(feature = "cmd_alias")]
pub const CONFIRM_BEFORE: &str = "confirm";

/// `display-menu`
pub const DISPLAY_MENU: &str = "display-menu";

/// `display-message`
#[cfg(not(feature = "cmd_alias"))]
pub const DISPLAY_MESSAGE: &str = "display-message";
/// `display`
#[cfg(feature = "cmd_alias")]
pub const DISPLAY_MESSAGE: &str = "display";

// Windows and panes

/// `break-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const BREAK_PANE: &str = "break-pane";
/// `breakp`
#[cfg(feature = "cmd_alias")]
pub const BREAK_PANE: &str = "breakp";

/// `capture-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const CAPTURE_PANE: &str = "capture-pane";
/// `capturep`
#[cfg(feature = "cmd_alias")]
pub const CAPTURE_PANE: &str = "capturep";

/// `choose-client`
pub const CHOOSE_CLIENT: &str = "choose-client";

/// `choose-tree`
pub const CHOOSE_TREE: &str = "choose-tree";

/// `copy-mode`
pub const COPY_MODE: &str = "copy-mode";

/// `display-panes`
#[cfg(not(feature = "cmd_alias"))]
pub const DISPLAY_PANES: &str = "display-panes";
/// `displayp`
#[cfg(feature = "cmd_alias")]
pub const DISPLAY_PANES: &str = "displayp";

/// `find-window`
#[cfg(not(feature = "cmd_alias"))]
pub const FIND_WINDOW: &str = "find-window";
/// `findw`
#[cfg(feature = "cmd_alias")]
pub const FIND_WINDOW: &str = "findw";

/// `join-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const JOIN_PANE: &str = "join-pane";
/// `joinp`
#[cfg(feature = "cmd_alias")]
pub const JOIN_PANE: &str = "joinp";

/// `kill-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const KILL_PANE: &str = "kill-pane";
/// `killp`
#[cfg(feature = "cmd_alias")]
pub const KILL_PANE: &str = "killp";

/// `kill-window`
#[cfg(not(feature = "cmd_alias"))]
pub const KILL_WINDOW: &str = "kill-window";
/// `killw`
#[cfg(feature = "cmd_alias")]
pub const KILL_WINDOW: &str = "killw";

/// `last-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const LAST_PANE: &str = "last-pane";
/// `lastp`
#[cfg(feature = "cmd_alias")]
pub const LAST_PANE: &str = "lastp";

/// `last-window`
#[cfg(not(feature = "cmd_alias"))]
pub const LAST_WINDOW: &str = "last-window";
/// `last`
#[cfg(feature = "cmd_alias")]
pub const LAST_WINDOW: &str = "last";

/// `link-window`
#[cfg(not(feature = "cmd_alias"))]
pub const LINK_WINDOW: &str = "link-window";
/// `linkw`
#[cfg(feature = "cmd_alias")]
pub const LINK_WINDOW: &str = "linkw";

/// `list-panes`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_PANES: &str = "list-panes";
/// `lsp`
#[cfg(feature = "cmd_alias")]
pub const LIST_PANES: &str = "lsp";

/// `list-windows`
#[cfg(not(feature = "cmd_alias"))]
pub const LIST_WINDOWS: &str = "list-windows";
/// `lsw`
#[cfg(feature = "cmd_alias")]
pub const LIST_WINDOWS: &str = "lsw";

/// `move-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const MOVE_PANE: &str = "move-pane";
/// `movep`
#[cfg(feature = "cmd_alias")]
pub const MOVE_PANE: &str = "movep";

/// `move-window`
#[cfg(not(feature = "cmd_alias"))]
pub const MOVE_WINDOW: &str = "move-window";
/// `movew`
#[cfg(feature = "cmd_alias")]
pub const MOVE_WINDOW: &str = "movew";

/// `new-window`
#[cfg(not(feature = "cmd_alias"))]
pub const NEW_WINDOW: &str = "new-window";
/// `neww`
#[cfg(feature = "cmd_alias")]
pub const NEW_WINDOW: &str = "neww";

/// `next-layout`
#[cfg(not(feature = "cmd_alias"))]
pub const NEXT_LAYOUT: &str = "next-layout";
/// `nextl`
#[cfg(feature = "cmd_alias")]
pub const NEXT_LAYOUT: &str = "nextl";

/// `next-window`
#[cfg(not(feature = "cmd_alias"))]
pub const NEXT_WINDOW: &str = "next-window";
/// `next`
#[cfg(feature = "cmd_alias")]
pub const NEXT_WINDOW: &str = "next";

/// `pipe-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const PIPE_PANE: &str = "pipe-pane";
/// `pipep`
#[cfg(feature = "cmd_alias")]
pub const PIPE_PANE: &str = "pipep";

/// `previous-layout`
#[cfg(not(feature = "cmd_alias"))]
pub const PREVIOUS_LAYOUT: &str = "previous-layout";
/// `prevl`
#[cfg(feature = "cmd_alias")]
pub const PREVIOUS_LAYOUT: &str = "prevl";

/// `previous-window`
#[cfg(not(feature = "cmd_alias"))]
pub const PREVIOUS_WINDOW: &str = "previous-window";
/// `prev`
#[cfg(feature = "cmd_alias")]
pub const PREVIOUS_WINDOW: &str = "prev";

/// `rename-window`
#[cfg(not(feature = "cmd_alias"))]
pub const RENAME_WINDOW: &str = "rename-window";
/// `renamew`
#[cfg(feature = "cmd_alias")]
pub const RENAME_WINDOW: &str = "renamew";

/// `resize-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const RESIZE_PANE: &str = "resize-pane";
/// `resizep`
#[cfg(feature = "cmd_alias")]
pub const RESIZE_PANE: &str = "resizep";

/// `resize-window`
#[cfg(not(feature = "cmd_alias"))]
pub const RESIZE_WINDOW: &str = "resize-window";
/// `resizew`
#[cfg(feature = "cmd_alias")]
pub const RESIZE_WINDOW: &str = "resizew";

/// `respawn-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const RESPAWN_PANE: &str = "respawn-pane";
/// `respawnp`
#[cfg(feature = "cmd_alias")]
pub const RESPAWN_PANE: &str = "respawnp";

/// `respawn-window`
#[cfg(not(feature = "cmd_alias"))]
pub const RESPAWN_WINDOW: &str = "respawn-window";
/// `respawnw`
#[cfg(feature = "cmd_alias")]
pub const RESPAWN_WINDOW: &str = "respawnw";

/// `rotate-window`
#[cfg(not(feature = "cmd_alias"))]
pub const ROTATE_WINDOW: &str = "rotate-window";
/// `rotatew`
#[cfg(feature = "cmd_alias")]
pub const ROTATE_WINDOW: &str = "rotatew";

/// `select-layout`
#[cfg(not(feature = "cmd_alias"))]
pub const SELECT_LAYOUT: &str = "select-layout";
/// `selectl`
#[cfg(feature = "cmd_alias")]
pub const SELECT_LAYOUT: &str = "selectl";

/// `select-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const SELECT_PANE: &str = "select-pane";
/// `selectp`
#[cfg(feature = "cmd_alias")]
pub const SELECT_PANE: &str = "selectp";

/// `select-window`
#[cfg(not(feature = "cmd_alias"))]
pub const SELECT_WINDOW: &str = "select-window";
/// `selectw`
#[cfg(feature = "cmd_alias")]
pub const SELECT_WINDOW: &str = "selectw";

/// `split-window`
#[cfg(not(feature = "cmd_alias"))]
pub const SPLIT_WINDOW: &str = "split-window";
/// `splitw`
#[cfg(feature = "cmd_alias")]
pub const SPLIT_WINDOW: &str = "splitw";

/// `swap-pane`
#[cfg(not(feature = "cmd_alias"))]
pub const SWAP_PANE: &str = "swap-pane";
/// `swapp`
#[cfg(feature = "cmd_alias")]
pub const SWAP_PANE: &str = "swapp";

/// `swap-window`
#[cfg(not(feature = "cmd_alias"))]
pub const SWAP_WINDOW: &str = "swap-window";
/// `swapw`
#[cfg(feature = "cmd_alias")]
pub const SWAP_WINDOW: &str = "swapw";

/// `unlink-window`
#[cfg(not(feature = "cmd_alias"))]
pub const UNLINK_WINDOW: &str = "unlink-window";
/// `unlinkw`
#[cfg(feature = "cmd_alias")]
pub const UNLINK_WINDOW: &str = "unlinkw";
