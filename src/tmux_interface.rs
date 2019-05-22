#![allow(non_upper_case_globals)]

extern crate regex;


use std::process::{Command, Output};
use std::str;
use regex::Regex;
use super::tmux_interface_error::TmuxInterfaceError;


// constants for use as keys for subcommands
pub const a_KEY: &str = "-a";
pub const b_KEY: &str = "-b";
pub const c_KEY: &str = "-c";
pub const d_KEY: &str = "-d";
pub const e_KEY: &str = "-e";
pub const f_KEY: &str = "-f";
pub const g_KEY: &str = "-g";
pub const h_KEY: &str = "-h";
//pub const i_KEY: &str = "-i";
//pub const j_KEY: &str = "-j";
pub const k_KEY: &str = "-k";
pub const l_KEY: &str = "-l";
pub const m_KEY: &str = "-m";
pub const n_KEY: &str = "-n";
//pub const o_KEY: &str = "-o";
pub const p_KEY: &str = "-p";
//pub const Q_KEY: &str = "-q";
pub const r_KEY: &str = "-r";
pub const s_KEY: &str = "-s";
pub const t_KEY: &str = "-t";
//pub const u_KEY: &str = "-u";
pub const v_KEY: &str = "-v";
//pub const w_KEY: &str = "-w";
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
//pub const G_KEY: &str = "-G";
//pub const H_KEY: &str = "-H";
//pub const I_KEY: &str = "-I";
//pub const J_KEY: &str = "-J";
//pub const K_KEY: &str = "-K";
pub const L_KEY: &str = "-L";
pub const M_KEY: &str = "-M";
pub const N_KEY: &str = "-N";
//pub const O_KEY: &str = "-O";
pub const P_KEY: &str = "-P";
//pub const Q_KEY: &str = "-Q";
pub const R_KEY: &str = "-R";
//pub const S_KEY: &str = "-S";
pub const T_KEY: &str = "-T";
pub const U_KEY: &str = "-U";
pub const V_KEY: &str = "-V";
//pub const W_KEY: &str = "-W";
pub const X_KEY: &str = "-X";
//pub const Y_KEY: &str = "-Y";
//pub const Z_KEY: &str = "-Z";




// XXX: mb also add_env, clear_env, remove_env for std::process::Command?
pub struct TmuxInterface<'a> {
    /// Tmux binary name (default: `tmux`, can be set as `tmux_mock.sh` for "sniffing")
    pub tmux: &'a str
}


impl<'a> TmuxInterface<'a> {

    const TMUX: &'static str = "tmux";
    const VERSION_STR_REGEX: &'static str = r"^tmux\s(\d+).(\d+)\n$";


    pub fn new(tmux: Option<&'a str>) -> Self {
        let tmux_cmd = tmux.unwrap_or(TmuxInterface::TMUX);
        TmuxInterface {
            tmux: tmux_cmd
        }
    }


    pub fn subcommand(&self, subcmd: &str, args: &[&str]) -> Result<Output, TmuxInterfaceError> {
        let mut tmux = Command::new(self.tmux);
        tmux.arg(subcmd);
        let output = tmux.args(args).output()?;
        Ok(output)
    }


    //pub fn exec(&self, subcmd: &str, args: &[String]) -> Result<ExitStatus, MyError> {
        //let mut tmux = Command::new(self.tmux);
        //tmux.arg(subcmd);
        //let status = tmux.args(args).status()?;
        ////println!("{} {} {:?}", Tmux::TMUX, subcmd, args);
        ////println!("[exec output]: {} {} {:?} {:?}", self.tmux, subcmd, args, status);
        ////if !status.success() {
            ////println!("[exec status error]: {}", status);
        ////}
        //Ok(status)
    //}


    // tmux parameter
    // ===========================================================================================

    /// ```text
    /// tmux -V
    /// ```
    pub fn version(&self) -> Result<(usize, usize), TmuxInterfaceError> {
        let mut tmux = Command::new(self.tmux);
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


