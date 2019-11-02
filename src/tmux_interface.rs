#![allow(non_upper_case_globals)]

use crate::Error;
use crate::Version;
use std::process::{Command, Output};
use std::str;

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

// XXX: callback hooks, before, after execution of the command thx for an idea to Jezza [Jezza's Fork](https://github.com/Jezza/tmux-interface-rs)
// XXX: mb also add_env, clear_env, remove_env for std::process::Command?
/// This structure is used to store execution parameters of `tmux`, including binary
/// name. Full description of fields can be found using `man tmux`.
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
#[derive(Default)]
pub struct TmuxInterface<'a> {
    /// Environment variables for tmux
    pub environment: Option<&'a str>, //
    /// Tmux binary name (default: `tmux`, can be set as `tmux_mock.sh` for "sniffing")
    pub tmux: Option<&'a str>, // tmux (or tmux_mock.sh)
    /// Force tmux to assume the terminal supports 256 colours
    pub colours256: Option<bool>, // -2
    /// Start in control mode
    pub control_mode: Option<bool>, // -C
    /// Disables echo
    pub disable_echo: Option<bool>, // -CC
    /// Execute shell-command using the default shell
    pub shell_cmd: Option<&'a str>, // -c shell-command
    /// Specify an alternative configuration file
    pub file: Option<&'a str>, // -f file
    /// Allows a different socket name to be specified
    pub socket_name: Option<&'a str>, // -L socket-name
    /// Behave as a login shell
    pub login_shell: Option<bool>, // -l
    /// Specify a full alternative path to the server socket
    pub socket_path: Option<&'a str>, // -S socket-path
    /// Write UTF-8 output to the terminal
    pub force_utf8: Option<bool>, // -u
    /// Request verbose logging
    pub verbose_logging: Option<bool>, // -v
    /// Report the tmux version
    pub version: Option<bool>, // -V
    //
    pub pre_hook: Option<Box<dyn FnMut(&mut String, &Vec<&str>, &Vec<&str>) -> Result<(), Error>>>,
}

/// Common `TmuxInterface` functions
impl<'a> TmuxInterface<'a> {
    const TMUX: &'static str = "tmux";

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
    /// let mut tmux = TmuxInterface::new();
    /// tmux.subcommand("has-session", &["-t", "session_name"]).unwrap();
    /// ```
    pub fn subcommand(&mut self, subcmd: &str, args: &[&str]) -> Result<Output, Error> {
        let mut options: Vec<&str> = Vec::new();
        options.push(subcmd);
        options.extend_from_slice(args);
        self.exec(&mut options)
    }

    pub fn exec(&mut self, mut args: &Vec<&str>) -> Result<Output, Error> {
        let mut options: Vec<&str> = Vec::new();
        let mut bin = self.tmux.unwrap_or(TmuxInterface::TMUX).to_string();
        // XXX: using environment vars
        //self.environment.and_then(|s| Some(envs.push(s)));
        if self.colours256.unwrap_or(false) {
            options.push(_2_KEY);
        };
        if self.control_mode.unwrap_or(false) {
            options.push(C_KEY);
        };
        if self.disable_echo.unwrap_or(false) {
            options.push(CC_KEY);
        };
        if self.login_shell.unwrap_or(false) {
            options.push(l_KEY)
        };
        if self.force_utf8.unwrap_or(false) {
            options.push(u_KEY)
        }
        if self.verbose_logging.unwrap_or(false) {
            options.push(v_KEY)
        }
        if let Some(s) = self.shell_cmd {
            options.extend_from_slice(&[c_KEY, &s])
        }
        if let Some(s) = self.file {
            options.extend_from_slice(&[f_KEY, &s])
        }
        if let Some(s) = self.socket_name {
            options.extend_from_slice(&[L_KEY, &s])
        }
        if let Some(s) = self.socket_path {
            options.extend_from_slice(&[S_KEY, &s])
        }

        // pre hook callback
        // XXX: check argumets, mb separate subcmd too
        if let Some(callback) = self.pre_hook.as_mut() {
            callback(&mut bin, &mut options, &mut args)?;
        }

        let mut cmd = Command::new(&bin);
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
    pub fn version(&mut self) -> Result<Version, Error> {
        let mut args: Vec<&str> = Vec::new();
        args.push(V_KEY);
        let output = self.exec(&args)?;
        let version_str = String::from_utf8_lossy(&output.stdout).to_string();
        let version = version_str.parse()?;
        Ok(version)
    }
}
