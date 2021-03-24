#![allow(non_upper_case_globals)]
pub const _1_KEY: &str = "-1";
pub const _2_KEY: &str = "-2";

// constants for use as keys for subcommands
pub const a_KEY: &str = "-a";
pub const b_KEY: &str = "-b";
pub const c_KEY: &str = "-c";
pub const d_KEY: &str = "-d";
pub const e_KEY: &str = "-e";
pub const f_KEY: &str = "-f";
pub const g_KEY: &str = "-g";
pub const h_KEY: &str = "-h";
pub const i_KEY: &str = "-i";
//pub const j_KEY: &str = "-j";
pub const k_KEY: &str = "-k";
pub const l_KEY: &str = "-l";
pub const m_KEY: &str = "-m";
pub const n_KEY: &str = "-n";
pub const o_KEY: &str = "-o";
pub const p_KEY: &str = "-p";
pub const q_KEY: &str = "-q";
pub const r_KEY: &str = "-r";
pub const s_KEY: &str = "-s";
pub const t_KEY: &str = "-t";
pub const u_KEY: &str = "-u";
pub const v_KEY: &str = "-v";
pub const w_KEY: &str = "-w";
pub const x_KEY: &str = "-x";
pub const y_KEY: &str = "-y";
//pub const z_KEY: &str = "-z";

#[allow(non_upper_case_globals)]
pub const A_KEY: &str = "-A";
//pub const B_KEY: &str = "-B";
pub const C_KEY: &str = "-C";
pub const D_KEY: &str = "-D";
pub const E_KEY: &str = "-E";
pub const F_KEY: &str = "-F";
pub const G_KEY: &str = "-G";
pub const H_KEY: &str = "-H";
pub const I_KEY: &str = "-I";
pub const J_KEY: &str = "-J";
//pub const K_KEY: &str = "-K";
pub const L_KEY: &str = "-L";
pub const M_KEY: &str = "-M";
pub const N_KEY: &str = "-N";
pub const O_KEY: &str = "-O";
pub const P_KEY: &str = "-P";
//pub const Q_KEY: &str = "-Q";
pub const R_KEY: &str = "-R";
pub const S_KEY: &str = "-S";
pub const T_KEY: &str = "-T";
pub const U_KEY: &str = "-U";
pub const V_KEY: &str = "-V";
//pub const W_KEY: &str = "-W";
pub const X_KEY: &str = "-X";
//pub const Y_KEY: &str = "-Y";
pub const Z_KEY: &str = "-Z";

pub const CC_KEY: &str = "-CC";

/// Default tmux binary name
pub const TMUX: &str = "tmux";

// XXX: mb CMD_ALIAS const

// Buffers

pub const CHOOSE_BUFFER: &str = "choose-buffer";

#[cfg(not(feature = "use_cmd_alias"))]
pub const CLEAR_HISTORY: &str = "clear-history";
#[cfg(feature = "use_cmd_alias")]
pub const CLEAR_HISTORY: &str = "clearhist";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DELETE_BUFFER: &str = "delete-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const DELETE_BUFFER: &str = "deleteb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_BUFFERS: &str = "list-buffers";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_BUFFERS: &str = "lsb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOAD_BUFFER: &str = "load-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const LOAD_BUFFER: &str = "loadb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const PASTE_BUFFER: &str = "paste-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const PASTE_BUFFER: &str = "pasteb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SAVE_BUFFER: &str = "save-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const SAVE_BUFFER: &str = "saveb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_BUFFER: &str = "set-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const SET_BUFFER: &str = "setb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_BUFFER: &str = "show-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_BUFFER: &str = "showb";

// Clients and sessions

#[cfg(not(feature = "use_cmd_alias"))]
pub const ATTACH_SESSION: &str = "attach-session";
#[cfg(feature = "use_cmd_alias")]
pub const ATTACH_SESSION: &str = "attach";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DETACH_CLIENT: &str = "detach-client";
#[cfg(feature = "use_cmd_alias")]
pub const DETACH_CLIENT: &str = "detach";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_CLIENTS: &str = "list-clients";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_CLIENTS: &str = "lsc";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_COMMANDS: &str = "list-commands";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_COMMANDS: &str = "lscm";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOCK_CLIENT: &str = "lock-client";
#[cfg(feature = "use_cmd_alias")]
pub const LOCK_CLIENT: &str = "lockc";

#[cfg(not(feature = "use_cmd_alias"))]
pub const NEW_SESSION: &str = "new-session";
#[cfg(feature = "use_cmd_alias")]
pub const NEW_SESSION: &str = "new";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_SESSIONS: &str = "list-sessions";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_SESSIONS: &str = "ls";

#[cfg(not(feature = "use_cmd_alias"))]
pub const REFRESH_CLIENT: &str = "refresh-client";
#[cfg(feature = "use_cmd_alias")]
pub const REFRESH_CLIENT: &str = "refresh";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RENAME_SESSION: &str = "rename-session";
#[cfg(feature = "use_cmd_alias")]
pub const RENAME_SESSION: &str = "rename";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_MESSAGES: &str = "show-messages";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_MESSAGES: &str = "showmsgs";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SOURCE_FILE: &str = "source-file";
#[cfg(feature = "use_cmd_alias")]
pub const SOURCE_FILE: &str = "source";

#[cfg(not(feature = "use_cmd_alias"))]
pub const START_SERVER: &str = "start-server";
#[cfg(feature = "use_cmd_alias")]
pub const START_SERVER: &str = "start";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SUSPEND_CLIENT: &str = "suspend-client";
#[cfg(feature = "use_cmd_alias")]
pub const SUSPEND_CLIENT: &str = "suspendc";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SWITCH_CLIENT: &str = "switch-client";
#[cfg(feature = "use_cmd_alias")]
pub const SWITCH_CLIENT: &str = "switchc";

pub const KILL_SERVER: &str = "kill-server";
pub const KILL_SESSION: &str = "kill-session";

#[cfg(not(feature = "use_cmd_alias"))]
pub const HAS_SESSION: &str = "has-session";
#[cfg(feature = "use_cmd_alias")]
pub const HAS_SESSION: &str = "has";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOCK_SESSION: &str = "lock-session";
#[cfg(feature = "use_cmd_alias")]
pub const LOCK_SESSION: &str = "locks";

// Global and session environment

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_ENVIRONMENT: &str = "set-environment";
#[cfg(feature = "use_cmd_alias")]
pub const SET_ENVIRONMENT: &str = "setenv";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_ENVIRONMENT: &str = "show-environment";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_ENVIRONMENT: &str = "showenv";

// Hooks

pub const SET_HOOK: &str = "set-hook";

pub const SHOW_HOOKS: &str = "show-hooks";

// Key Bindings

#[cfg(not(feature = "use_cmd_alias"))]
pub const BIND_KEY: &str = "bind-key";
#[cfg(feature = "use_cmd_alias")]
pub const BIND_KEY: &str = "bind";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_KEYS: &str = "list-keys";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_KEYS: &str = "lsk";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SEND_KEYS: &str = "send-keys";
#[cfg(feature = "use_cmd_alias")]
pub const SEND_KEYS: &str = "send";

pub const SEND_PREFIX: &str = "send-prefix";

#[cfg(not(feature = "use_cmd_alias"))]
pub const UNBIND_KEY: &str = "unbind-key";
#[cfg(feature = "use_cmd_alias")]
pub const UNBIND_KEY: &str = "unbind";

// Miscellaneous

pub const CLOCK_MODE: &str = "clock-mode";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOCK_SERVER: &str = "lock-server";
#[cfg(feature = "use_cmd_alias")]
pub const LOCK_SERVER: &str = "lock";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RUN_SHELL: &str = "run-shell";
#[cfg(feature = "use_cmd_alias")]
pub const RUN_SHELL: &str = "run";

#[cfg(not(feature = "use_cmd_alias"))]
pub const WAIT_FOR: &str = "wait-for";
#[cfg(feature = "use_cmd_alias")]
pub const WAIT_FOR: &str = "wait";

#[cfg(not(feature = "use_cmd_alias"))]
pub const IF_SHELL: &str = "if-shell";
#[cfg(feature = "use_cmd_alias")]
pub const IF_SHELL: &str = "if";

// Options

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_OPTION: &str = "set-option";
#[cfg(feature = "use_cmd_alias")]
pub const SET_OPTION: &str = "set";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_WINDOW_OPTION: &str = "set-window-option";
#[cfg(feature = "use_cmd_alias")]
pub const SET_WINDOW_OPTION: &str = "setw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_OPTIONS: &str = "show-options";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_OPTIONS: &str = "show";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_WINDOW_OPTIONS: &str = "show-window-options";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_WINDOW_OPTIONS: &str = "showw";

// Status Line

pub const COMMAND_PROMPT: &str = "command-prompt";

#[cfg(not(feature = "use_cmd_alias"))]
pub const CONFIRM_BEFORE: &str = "confirm-before";
#[cfg(feature = "use_cmd_alias")]
pub const CONFIRM_BEFORE: &str = "confirm";

pub const DISPLAY_MENU: &str = "display-menu";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DISPLAY_MESSAGE: &str = "display-message";
#[cfg(feature = "use_cmd_alias")]
pub const DISPLAY_MESSAGE: &str = "display";

// Windows and panes

#[cfg(not(feature = "use_cmd_alias"))]
pub const BREAK_PANE: &str = "break-pane";
#[cfg(feature = "use_cmd_alias")]
pub const BREAK_PANE: &str = "breakp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const CAPTURE_PANE: &str = "capture-pane";
#[cfg(feature = "use_cmd_alias")]
pub const CAPTURE_PANE: &str = "capturep";

pub const CHOOSE_CLIENT: &str = "choose-client";

pub const CHOOSE_TREE: &str = "choose-tree";

pub const COPY_MODE: &str = "copy-mode";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DISPLAY_PANES: &str = "display-panes";
#[cfg(feature = "use_cmd_alias")]
pub const DISPLAY_PANES: &str = "displayp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const FIND_WINDOW: &str = "find-window";
#[cfg(feature = "use_cmd_alias")]
pub const FIND_WINDOW: &str = "findw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const JOIN_PANE: &str = "join-pane";
#[cfg(feature = "use_cmd_alias")]
pub const JOIN_PANE: &str = "joinp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const KILL_PANE: &str = "kill-pane";
#[cfg(feature = "use_cmd_alias")]
pub const KILL_PANE: &str = "killp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const KILL_WINDOW: &str = "kill-window";
#[cfg(feature = "use_cmd_alias")]
pub const KILL_WINDOW: &str = "killw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LAST_PANE: &str = "last-pane";
#[cfg(feature = "use_cmd_alias")]
pub const LAST_PANE: &str = "lastp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LAST_WINDOW: &str = "last-window";
#[cfg(feature = "use_cmd_alias")]
pub const LAST_WINDOW: &str = "last";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LINK_WINDOW: &str = "link-window";
#[cfg(feature = "use_cmd_alias")]
pub const LINK_WINDOW: &str = "linkw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_PANES: &str = "list-panes";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_PANES: &str = "lsp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_WINDOWS: &str = "list-windows";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_WINDOWS: &str = "lsw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const MOVE_PANE: &str = "move-pane";
#[cfg(feature = "use_cmd_alias")]
pub const MOVE_PANE: &str = "movep";

#[cfg(not(feature = "use_cmd_alias"))]
pub const MOVE_WINDOW: &str = "move-window";
#[cfg(feature = "use_cmd_alias")]
pub const MOVE_WINDOW: &str = "movew";

#[cfg(not(feature = "use_cmd_alias"))]
pub const NEW_WINDOW: &str = "new-window";
#[cfg(feature = "use_cmd_alias")]
pub const NEW_WINDOW: &str = "neww";

#[cfg(not(feature = "use_cmd_alias"))]
pub const NEXT_LAYOUT: &str = "next-layout";
#[cfg(feature = "use_cmd_alias")]
pub const NEXT_LAYOUT: &str = "nextl";

#[cfg(not(feature = "use_cmd_alias"))]
pub const NEXT_WINDOW: &str = "next-window";
#[cfg(feature = "use_cmd_alias")]
pub const NEXT_WINDOW: &str = "next";

#[cfg(not(feature = "use_cmd_alias"))]
pub const PIPE_PANE: &str = "pipe-pane";
#[cfg(feature = "use_cmd_alias")]
pub const PIPE_PANE: &str = "pipep";

#[cfg(not(feature = "use_cmd_alias"))]
pub const PREVIOUS_LAYOUT: &str = "previous-layout";
#[cfg(feature = "use_cmd_alias")]
pub const PREVIOUS_LAYOUT: &str = "prevl";

#[cfg(not(feature = "use_cmd_alias"))]
pub const PREVIOUS_WINDOW: &str = "previous-window";
#[cfg(feature = "use_cmd_alias")]
pub const PREVIOUS_WINDOW: &str = "prev";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RENAME_WINDOW: &str = "rename-window";
#[cfg(feature = "use_cmd_alias")]
pub const RENAME_WINDOW: &str = "renamew";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RESIZE_PANE: &str = "resize-pane";
#[cfg(feature = "use_cmd_alias")]
pub const RESIZE_PANE: &str = "resizep";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RESIZE_WINDOW: &str = "resize-window";
#[cfg(feature = "use_cmd_alias")]
pub const RESIZE_WINDOW: &str = "resizew";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RESPAWN_PANE: &str = "respawn-pane";
#[cfg(feature = "use_cmd_alias")]
pub const RESPAWN_PANE: &str = "respawnp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RESPAWN_WINDOW: &str = "respawn-window";
#[cfg(feature = "use_cmd_alias")]
pub const RESPAWN_WINDOW: &str = "respawnw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const ROTATE_WINDOW: &str = "rotate-window";
#[cfg(feature = "use_cmd_alias")]
pub const ROTATE_WINDOW: &str = "rotatew";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SELECT_LAYOUT: &str = "select-layout";
#[cfg(feature = "use_cmd_alias")]
pub const SELECT_LAYOUT: &str = "selectl";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SELECT_PANE: &str = "select-pane";
#[cfg(feature = "use_cmd_alias")]
pub const SELECT_PANE: &str = "selectp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SELECT_WINDOW: &str = "select-window";
#[cfg(feature = "use_cmd_alias")]
pub const SELECT_WINDOW: &str = "selectw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SPLIT_WINDOW: &str = "split-window";
#[cfg(feature = "use_cmd_alias")]
pub const SPLIT_WINDOW: &str = "splitw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SWAP_PANE: &str = "swap-pane";
#[cfg(feature = "use_cmd_alias")]
pub const SWAP_PANE: &str = "swapp";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SWAP_WINDOW: &str = "swap-window";
#[cfg(feature = "use_cmd_alias")]
pub const SWAP_WINDOW: &str = "swapw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const UNLINK_WINDOW: &str = "unlink-window";
#[cfg(feature = "use_cmd_alias")]
pub const UNLINK_WINDOW: &str = "unlinkw";
