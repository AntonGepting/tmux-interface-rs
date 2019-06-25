#![allow(non_upper_case_globals)]

extern crate regex;


use std::process::{Command, Output};
use std::str;
use regex::Regex;
use super::tmux_interface_error::TmuxInterfaceError;


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


pub const CC_KEY: &str = "-C";


// XXX: mb also add_env, clear_env, remove_env for std::process::Command?
/// This structure is used to store execution parameters of `tmux`, including binary
/// name. Full description of fields can be found using `man tmux`.
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
#[derive(Default)]
pub struct TmuxInterface<'a> {
    /// Environment variables for tmux
    pub environment: Option<&'a str>,                   //
    /// Tmux binary name (default: `tmux`, can be set as `tmux_mock.sh` for "sniffing")
    pub tmux: Option<&'a str>,                          // tmux (or tmux_mock.sh)
    /// Force tmux to assume the terminal supports 256 colours
    pub colours256: Option<bool>,                       // -2
    /// Start in control mode
    pub control_mode: Option<bool>,                     // -C
    /// Disables echo
    pub disable_echo: Option<bool>,                     // -CC
    /// Execute shell-command using the default shell
    pub shell_cmd: Option<&'a str>,                     // -c shell-command
    /// Specify an alternative configuration file
    pub file: Option<&'a str>,                          // -f file
    /// Allows a different socket name to be specified
    pub socket_name: Option<&'a str>,                   // -L socket-name
    /// Behave as a login shell
    pub login_shell: Option<bool>,                      // -l
    /// Specify a full alternative path to the server socket
    pub socket_path: Option<&'a str>,                   // -S socket-path
    /// Write UTF-8 output to the terminal
    pub force_utf8: Option<bool>,                       // -u
    /// Request verbose logging
    pub verbose_logging: Option<bool>,                  // -v
    /// Report the tmux version
    pub version: Option<bool>                           // -V
}


/// Common `TmuxInterface` functions
impl<'a> TmuxInterface<'a> {

    const TMUX: &'static str = "tmux";
    const VERSION_STR_REGEX: &'static str = r"^tmux\s(\d+).(\d+)\n$";

    /// Create new `TmuxInterface` instance initialized with default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Execute subcommand of tmux
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path]
    /// [command [flags]]
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::tmux_interface::TmuxInterface;
    ///
    /// let tmux = TmuxInterface::new();
    /// tmux.subcommand("has-session", &["-t", "session_name"]).unwrap();
    /// ```
    pub fn subcommand(&self, subcmd: &str, args: &[&str]) -> Result<Output, TmuxInterfaceError> {
        let mut options: Vec<&str> = Vec::new();
        options.push(subcmd);
        options.extend_from_slice(args);
        self.exec(&options)
    }


    pub fn exec(&self, args: &[&str]) -> Result<Output, TmuxInterfaceError> {
        let mut options: Vec<&str> = Vec::new();
        let mut cmd = Command::new(self.tmux.unwrap_or(TmuxInterface::TMUX));
        // XXX: using environment vars
        //self.environment.and_then(|s| Some(envs.push(s)));
        if self.colours256.unwrap_or(false) { options.push(_2_KEY); };
        if self.control_mode.unwrap_or(false) { options.push(C_KEY); };
        if self.disable_echo.unwrap_or(false) { options.push(CC_KEY); };
        if self.login_shell.unwrap_or(false) { options.push(l_KEY) };
        if self.force_utf8.unwrap_or(false) { options.push(u_KEY) }
        if self.verbose_logging.unwrap_or(false) { options.push(v_KEY) }
        self.shell_cmd.and_then(|s| Some(options.extend_from_slice(&[c_KEY, &s])));
        self.file.and_then(|s| Some(options.extend_from_slice(&[f_KEY, &s])));
        self.socket_name.and_then(|s| Some(options.extend_from_slice(&[L_KEY, &s])));
        self.socket_path.and_then(|s| Some(options.extend_from_slice(&[S_KEY, &s])));
        cmd.args(options);
        let output = cmd.args(args).output()?;
        Ok(output)
    }


    // XXX: refactor: move
    /// # Manual
    ///
    /// ```text
    /// tmux -V
    /// ```
    pub fn version(&self) -> Result<(usize, usize), TmuxInterfaceError> {
        let mut tmux = Command::new(self.tmux.unwrap_or(TmuxInterface::TMUX));
        let output = tmux.arg(V_KEY).output()?;
        let version_str = String::from_utf8_lossy(&output.stdout).to_string();
        let regex = Regex::new(TmuxInterface::VERSION_STR_REGEX)?;
        if let Some(cap) = regex.captures(&version_str) {
            let major = cap[1].parse::<usize>()?;
            let minor = cap[2].parse::<usize>()?;
            Ok((major, minor))
        } else {
            Err(TmuxInterfaceError::new("cap"))
        }
    }
}
