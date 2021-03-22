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

// Default tmux binary name
pub const TMUX: &'static str = "tmux";

#[cfg(not(feature = "use_cmd_alias"))]
pub const ATTACH_SESSION: &'static str = "attach-session";
#[cfg(feature = "use_cmd_alias")]
pub const ATTACH_SESSION: &'static str = "attach";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DETACH_CLIENT: &'static str = "detach-client";
#[cfg(feature = "use_cmd_alias")]
pub const DETACH_CLIENT: &'static str = "detach";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_CLIENTS: &'static str = "list-clients";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_CLIENTS: &'static str = "lsc";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_COMMANDS: &'static str = "list-commands";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_COMMANDS: &'static str = "lscm";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOCK_CLIENT: &'static str = "lock-client";
#[cfg(feature = "use_cmd_alias")]
pub const LOCK_CLIENT: &'static str = "lockc";

#[cfg(not(feature = "use_cmd_alias"))]
pub const NEW_SESSION: &'static str = "new-session";
#[cfg(feature = "use_cmd_alias")]
pub const NEW_SESSION: &'static str = "new";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_SESSIONS: &'static str = "list-sessions";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_SESSIONS: &'static str = "ls";

#[cfg(not(feature = "use_cmd_alias"))]
pub const REFRESH_CLIENT: &'static str = "refresh-client";
#[cfg(feature = "use_cmd_alias")]
pub const REFRESH_CLIENT: &'static str = "refresh";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RENAME_SESSION: &'static str = "rename-session";
#[cfg(feature = "use_cmd_alias")]
pub const RENAME_SESSION: &'static str = "rename";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_MESSAGES: &'static str = "show-messages";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_MESSAGES: &'static str = "showmsgs";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SOURCE_FILE: &'static str = "source-file";
#[cfg(feature = "use_cmd_alias")]
pub const SOURCE_FILE: &'static str = "source";

#[cfg(not(feature = "use_cmd_alias"))]
pub const START_SERVER: &'static str = "start-server";
#[cfg(feature = "use_cmd_alias")]
pub const START_SERVER: &'static str = "start";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SUSPEND_CLIENT: &'static str = "suspend-client";
#[cfg(feature = "use_cmd_alias")]
pub const SUSPEND_CLIENT: &'static str = "suspendc";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SWITCH_CLIENT: &'static str = "switch-client";
#[cfg(feature = "use_cmd_alias")]
pub const SWITCH_CLIENT: &'static str = "switchc";

pub const KILL_SERVER: &'static str = "kill-server";
pub const KILL_SESSION: &'static str = "kill-session";

#[cfg(not(feature = "use_cmd_alias"))]
pub const HAS_SESSION: &'static str = "has-session";
#[cfg(feature = "use_cmd_alias")]
pub const HAS_SESSION: &'static str = "has";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOCK_SESSION: &'static str = "lock-session";
#[cfg(feature = "use_cmd_alias")]
pub const LOCK_SESSION: &'static str = "locks";

// Buffers

pub const CHOOSE_BUFFER: &'static str = "choose-buffer";

#[cfg(not(feature = "use_cmd_alias"))]
pub const CLEAR_HISTORY: &'static str = "clear-history";
#[cfg(feature = "use_cmd_alias")]
pub const CLEAR_HISTORY: &'static str = "clearhist";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DELETE_BUFFER: &'static str = "delete-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const DELETE_BUFFER: &'static str = "deleteb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_BUFFERS: &'static str = "list-buffers";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_BUFFERS: &'static str = "lsb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOAD_BUFFER: &'static str = "load-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const LOAD_BUFFER: &'static str = "loadb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const PASTE_BUFFER: &'static str = "paste-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const PASTE_BUFFER: &'static str = "pasteb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SAVE_BUFFER: &'static str = "save-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const SAVE_BUFFER: &'static str = "saveb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_BUFFER: &'static str = "set-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const SET_BUFFER: &'static str = "setb";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_BUFFER: &'static str = "show-buffer";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_BUFFER: &'static str = "showb";

// Environment

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_ENVIRONMENT: &'static str = "set-environment";
#[cfg(feature = "use_cmd_alias")]
pub const SET_ENVIRONMENT: &'static str = "setenv";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_ENVIRONMENT: &'static str = "show-environment";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_ENVIRONMENT: &'static str = "showenv";

// Hooks

pub const SET_HOOK: &'static str = "set-hook";

pub const SHOW_HOOKS: &'static str = "show-hooks";

// Keys

#[cfg(not(feature = "use_cmd_alias"))]
pub const BIND_KEY: &'static str = "bind-key";
#[cfg(feature = "use_cmd_alias")]
pub const BIND_KEY: &'static str = "bind";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LIST_KEYS: &'static str = "list-keys";
#[cfg(feature = "use_cmd_alias")]
pub const LIST_KEYS: &'static str = "lsk";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SEND_KEYS: &'static str = "send-keys";
#[cfg(feature = "use_cmd_alias")]
pub const SEND_KEYS: &'static str = "send";

pub const SEND_PREFIX: &'static str = "send-prefix";

#[cfg(not(feature = "use_cmd_alias"))]
pub const UNBIND_KEY: &'static str = "unbind-key";
#[cfg(feature = "use_cmd_alias")]
pub const UNBIND_KEY: &'static str = "unbind";

// miscellaneous

pub const CLOCK_MODE: &'static str = "clock-mode";

#[cfg(not(feature = "use_cmd_alias"))]
pub const LOCK_SERVER: &'static str = "lock-server";
#[cfg(feature = "use_cmd_alias")]
pub const LOCK_SERVER: &'static str = "lock";

#[cfg(not(feature = "use_cmd_alias"))]
pub const RUN_SHELL: &'static str = "run-shell";
#[cfg(feature = "use_cmd_alias")]
pub const RUN_SHELL: &'static str = "run";

#[cfg(not(feature = "use_cmd_alias"))]
pub const WAIT_FOR: &'static str = "wait-for";
#[cfg(feature = "use_cmd_alias")]
pub const WAIT_FOR: &'static str = "wait";

#[cfg(not(feature = "use_cmd_alias"))]
pub const IF_SHELL: &'static str = "if-shell";
#[cfg(feature = "use_cmd_alias")]
pub const IF_SHELL: &'static str = "if";

// options

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_OPTION: &'static str = "set-option";
#[cfg(feature = "use_cmd_alias")]
pub const SET_OPTION: &'static str = "set";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SET_WINDOW_OPTION: &'static str = "set-window-option";
#[cfg(feature = "use_cmd_alias")]
pub const SET_WINDOW_OPTION: &'static str = "setw";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_OPTIONS: &'static str = "show-options";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_OPTIONS: &'static str = "show";

#[cfg(not(feature = "use_cmd_alias"))]
pub const SHOW_WINDOW_OPTIONS: &'static str = "show-window-options";
#[cfg(feature = "use_cmd_alias")]
pub const SHOW_WINDOW_OPTIONS: &'static str = "showw";

// status line

pub const COMMAND_PROMPT: &'static str = "command-prompt";

#[cfg(not(feature = "use_cmd_alias"))]
pub const CONFIRM_BEFORE: &'static str = "confirm-before";
#[cfg(feature = "use_cmd_alias")]
pub const CONFIRM_BEFORE: &'static str = "confirm";

pub const DISPLAY_MENU: &'static str = "display-menu";

#[cfg(not(feature = "use_cmd_alias"))]
pub const DISPLAY_MESSAGE: &'static str = "display-message";
#[cfg(feature = "use_cmd_alias")]
pub const DISPLAY_MESSAGE: &'static str = "display";
