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
