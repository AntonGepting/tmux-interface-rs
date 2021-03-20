#![allow(non_upper_case_globals)]
use crate::commands::constants::*;
use crate::Error;
use crate::Version;
use std::process::{Command, Output, Stdio};
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
///
/// # Manual
///
/// tmux ^2.1:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux [-2lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux [-28lCquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux [-28lquvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.1:
/// ```text
/// tmux [-28lquv] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux [-28dlqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux [-28dqUuv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux [-28dqUuVv] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
#[derive(Default)]
pub struct TmuxInterface<'a> {
    /// Tmux options fields
    ///
    // XXX: combine into a struct and separate from other options?
    /// [-2] - Force tmux to assume the terminal supports 256 colours
    #[cfg(feature = "tmux_0_8")]
    pub colours256: Option<bool>,
    /// [-8] - indicates that tmux supports 88 colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub colours88: Option<bool>,
    /// [-d] - indicates that tmux supports defaults colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub default_colours: Option<bool>,
    /// [-q] - prevent the server sending various information messages
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub prevent_msg: Option<bool>,
    /// [-C] - Start in control mode
    #[cfg(feature = "tmux_1_8")]
    pub control_mode: Option<bool>,
    /// [-l] - Behave as a login shell
    #[cfg(feature = "tmux_1_0")]
    pub login_shell: Option<bool>,
    /// [-U] - Unlock the server
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub unlock: Option<bool>,
    /// [-u] - Write UTF-8 output to the terminal
    #[cfg(feature = "tmux_0_8")]
    pub force_utf8: Option<bool>,
    /// [-v] - Request verbose logging
    #[cfg(feature = "tmux_0_8")]
    pub verbose_logging: Option<bool>,
    /// [-V] - Report the tmux version
    #[cfg(feature = "tmux_1_4")]
    pub version: Option<bool>,
    /// [-c shell-command] - Execute shell-command using the default shell
    #[cfg(feature = "tmux_1_1")]
    pub shell_cmd: Option<&'a str>,
    /// [-f file] - Specify an alternative configuration file
    #[cfg(feature = "tmux_0_8")]
    pub file: Option<&'a str>,
    /// [-L socket-name] - Allow a different socket name to be specified
    #[cfg(feature = "tmux_0_8")]
    pub socket_name: Option<&'a str>,
    /// [-S socket-path] - Specify a full alternative path to the server socket
    #[cfg(feature = "tmux_0_8")]
    pub socket_path: Option<&'a str>,
    /// [-CC] - Disable echo
    pub disable_echo: Option<bool>,
    ///// [command [flags]]
    //#[cfg(feature = "tmux_0_8")]
    //pub command
    /// non tmux options fields
    ///
    /// Tmux binary name (default: `tmux`, can be set as `tmux_mock.sh` for "sniffing")
    pub tmux: Option<&'a str>, // tmux (or tmux_mock.sh)

    /// Environment variables for tmux (currently not used)
    pub environment: Option<&'a str>, //

    /// define a callback function for displaying or editing all command's arguments:
    /// callback(bin: &mut String, options: &Vec<&str>, subcmd_with_args: &Vec<&str>) -> Result
    /// this function will be called before command.output();
    // XXX: return?
    pub pre_hook: Option<
        Box<dyn FnMut(&mut String, &Vec<&str>, &Vec<&str>) -> Result<Option<Output>, Error>>,
    >,

    /// define a callback function for displaying or editing command's output
    /// callback(output: &mut Output) -> Result
    /// this function will be called after command.output();
    // XXX: return?
    pub post_hook: Option<Box<dyn FnMut(&mut Output) -> Result<Option<Output>, Error>>>,
}

#[derive(Default)]
pub struct TmuxInterfaceBuilder<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub colours256: Option<bool>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub colours88: Option<bool>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub default_colours: Option<bool>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub prevent_msg: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub control_mode: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub login_shell: Option<bool>,
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub unlock: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub force_utf8: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub verbose_logging: Option<bool>,
    #[cfg(feature = "tmux_1_4")]
    pub version: Option<bool>,
    #[cfg(feature = "tmux_1_1")]
    pub shell_cmd: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub file: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub socket_name: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub socket_path: Option<&'a str>,
    pub disable_echo: Option<bool>,

    pub tmux: Option<&'a str>,
    pub environment: Option<&'a str>,

    pub pre_hook: Option<
        Box<dyn FnMut(&mut String, &Vec<&str>, &Vec<&str>) -> Result<Option<Output>, Error>>,
    >,
    pub post_hook: Option<Box<dyn FnMut(&mut Output) -> Result<Option<Output>, Error>>>,
}

impl<'a> TmuxInterfaceBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn colours256(&mut self) -> &mut Self {
        self.colours256 = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn control_mode(&mut self) -> &mut Self {
        self.control_mode = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn login_shell(&mut self) -> &mut Self {
        self.login_shell = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn force_utf8(&mut self) -> &mut Self {
        self.force_utf8 = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn verbose_logging(&mut self) -> &mut Self {
        self.verbose_logging = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn version(&mut self) -> &mut Self {
        self.verbose_logging = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn shell_cmd(&mut self) -> &mut Self {
        self.verbose_logging = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn file(&mut self, file: &'a str) -> &mut Self {
        self.file = Some(file);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_name(&mut self, socket_name: &'a str) -> &mut Self {
        self.socket_name = Some(socket_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn socket_path(&mut self, socket_path: &'a str) -> &mut Self {
        self.socket_path = Some(socket_path);
        self
    }

    pub fn disable_echo(&mut self) -> &mut Self {
        self.disable_echo = Some(true);
        self
    }

    pub fn tmux(&mut self, tmux: &'a str) -> &mut Self {
        self.tmux = Some(tmux);
        self
    }

    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    pub fn pre_hook(
        &mut self,
        pre_hook: Box<
            dyn FnMut(&mut String, &Vec<&str>, &Vec<&str>) -> Result<Option<Output>, Error>,
        >,
    ) -> &mut Self {
        self.pre_hook = Some(pre_hook);
        self
    }

    pub fn post_hook(
        &mut self,
        post_hook: Box<dyn FnMut(&mut Output) -> Result<Option<Output>, Error>>,
    ) -> &mut Self {
        self.post_hook = Some(post_hook);
        self
    }

    pub fn build(&self) -> TmuxInterface<'a> {
        TmuxInterface {
            colours256: self.colours256,
            control_mode: self.control_mode,
            login_shell: self.login_shell,
            force_utf8: self.force_utf8,
            verbose_logging: self.verbose_logging,
            version: self.version,
            shell_cmd: self.shell_cmd,
            file: self.file,
            socket_name: self.socket_name,
            socket_path: self.socket_path,
            disable_echo: self.disable_echo,
            tmux: self.tmux,
            environment: self.environment,
            // TODO: fix errors
            pre_hook: None,
            post_hook: None,
        }
    }
}

/// Common `TmuxInterface` functions
impl<'a> TmuxInterface<'a> {
    const TMUX: &'static str = "tmux";

    /// Create new `TmuxInterface` instance initialized with default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Execute tmux command
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
    /// tmux.command("has-session", &["-t", "session_name"]).unwrap();
    /// ```
    // XXX: rename to command?
    pub fn command(&mut self, cmd: &str, args: &[&str]) -> Result<Output, Error> {
        let mut options: Vec<&str> = Vec::new();
        options.push(cmd);
        options.extend_from_slice(args);
        self.exec(&mut options)
    }

    pub fn exec(&mut self, mut args: &Vec<&str>) -> Result<Output, Error> {
        let mut options: Vec<&str> = Vec::new();
        let mut bin = self.tmux.unwrap_or(TmuxInterface::TMUX).to_string();
        // XXX: using environment vars
        //self.environment.and_then(|s| Some(envs.push(s)));
        #[cfg(feature = "tmux_0_8")]
        if self.colours256.unwrap_or(false) {
            options.push(_2_KEY);
        };
        #[cfg(feature = "tmux_1_8")]
        if self.control_mode.unwrap_or(false) {
            options.push(C_KEY);
        };
        if self.disable_echo.unwrap_or(false) {
            options.push(CC_KEY);
        };
        #[cfg(feature = "tmux_1_0")]
        if self.login_shell.unwrap_or(false) {
            options.push(l_KEY)
        };
        #[cfg(feature = "tmux_0_8")]
        if self.force_utf8.unwrap_or(false) {
            options.push(u_KEY)
        }
        #[cfg(feature = "tmux_0_8")]
        if self.verbose_logging.unwrap_or(false) {
            options.push(v_KEY)
        }
        #[cfg(feature = "tmux_1_1")]
        if let Some(s) = self.shell_cmd {
            options.extend_from_slice(&[c_KEY, &s])
        }
        #[cfg(feature = "tmux_0_8")]
        if let Some(s) = self.file {
            options.extend_from_slice(&[f_KEY, &s])
        }
        #[cfg(feature = "tmux_0_8")]
        if let Some(s) = self.socket_name {
            options.extend_from_slice(&[L_KEY, &s])
        }
        #[cfg(feature = "tmux_0_8")]
        if let Some(s) = self.socket_path {
            options.extend_from_slice(&[S_KEY, &s])
        }

        // pre hook callback
        // XXX: check argumets, mb separate cmd too
        if let Some(pre_hook) = self.pre_hook.as_mut() {
            let result = pre_hook(&mut bin, &mut options, &mut args)?;
            if let Some(mut output) = result {
                // post hook callback
                if let Some(post_hook) = self.post_hook.as_mut() {
                    let result = post_hook(&mut output)?;
                    if let Some(output) = result {
                        return Ok(output);
                    }
                }
                return Ok(output);
            }
        }

        let mut cmd = Command::new(&bin);
        // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
        cmd.stdin(Stdio::inherit());
        cmd.args(options);
        let mut output = cmd.args(args).output()?;

        // post hook callback
        if let Some(post_hook) = self.post_hook.as_mut() {
            let result = post_hook(&mut output)?;
            if let Some(output) = result {
                return Ok(output);
            }
        }

        Ok(output)
    }

    // XXX: allow custom function
    // pub fn custom_subcommand() -> Result<(), ()> {
    // }

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
