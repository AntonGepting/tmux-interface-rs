use crate::commands::constants::*;
use crate::commands::tmux_bin::TmuxBin;
use crate::commands::tmux_command::Args;
use crate::TmuxCommand;
use std::borrow::Cow;

pub struct Tmux<'a>(pub TmuxBin<'a>);

// NOTE: [-N] missing in man
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#DESCRIPTION)
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux [-2CDluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [-T features] [command [flags]]
/// ```
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
// XXX: using environment vars
impl<'a> Tmux<'a> {
    pub fn new() -> Self {
        Self(TmuxBin::new())
    }

    /// `[-2]` - Force tmux to assume the terminal supports 256 colours
    #[cfg(feature = "tmux_0_8")]
    pub fn colours256(&mut self) -> &mut Self {
        self.0.args.push_flag(_2_KEY);
        self
    }

    /// `[-8]` - indicates that tmux supports 88 colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    pub fn colours88(&mut self) -> &mut Self {
        self.0.args.push_flag(_8_KEY);
        self
    }

    /// `[-d]` - indicates that tmux supports defaults colours
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub fn default_colours(&mut self) -> &mut Self {
        self.0.args.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-q]` - prevent the server sending various information messages
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    pub fn prevent_msg(&mut self) -> &mut Self {
        self.0.args.push_flag(Q_LOWERCASE_KEY);
        self
    }

    /// `[-C]` - Start in control mode
    #[cfg(feature = "tmux_1_8")]
    pub fn control_mode(&mut self) -> &mut Self {
        self.0.args.push_flag(C_UPPERCASE_KEY);
        self
    }

    /// `[-CC]` - Disable echo
    #[cfg(feature = "tmux_1_8")]
    pub fn disable_echo(&mut self) -> &mut Self {
        self.0.args.push_flag(CC_UPPERCASE_KEY);
        self
    }

    /// `[-D]` - Do not start the tmux server as a daemon. This also turns the exit-empty option off.  With -D, command may not be specified.
    #[cfg(feature = "tmux_3_2")]
    pub fn no_daemon(&mut self) -> &mut Self {
        self.0.args.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-l]` - Behave as a login shell
    #[cfg(feature = "tmux_1_0")]
    pub fn login_shell(&mut self) -> &mut Self {
        self.0.args.push_flag(L_LOWERCASE_KEY);
        self
    }

    /// `[-N]` - Do not start the server even if the command would normally do so (for example new-session or start-server).
    #[cfg(feature = "tmux_3_2")]
    pub fn no_start(&mut self) -> &mut Self {
        self.0.args.push_flag(N_UPPERCASE_KEY);
        self
    }

    /// `[-U]` - Unlock the server
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_1")))]
    pub fn unlock(&mut self) -> &mut Self {
        self.0.args.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `[-u]` - Write UTF-8 output to the terminal
    #[cfg(feature = "tmux_0_8")]
    pub fn force_utf8(&mut self) -> &mut Self {
        self.0.args.push_flag(U_LOWERCASE_KEY);
        self
    }

    /// `[-v]` - Request verbose logging
    #[cfg(feature = "tmux_0_8")]
    pub fn verbose_logging(&mut self) -> &mut Self {
        self.0.args.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `[-V]` - Report the tmux version
    #[cfg(feature = "tmux_0_8")]
    pub fn version(&mut self) -> &mut Self {
        self.0.args.push_flag(V_UPPERCASE_KEY);
        self
    }

    /// `[-c shell-command]` - Execute shell-command using the default shell
    #[cfg(feature = "tmux_1_1")]
    pub fn shell_cmd<S: Into<Cow<'a, str>>>(&mut self, shell_cmd: S) -> &mut Self {
        self.0.args.push_option(C_LOWERCASE_KEY, shell_cmd);
        self
    }

    /// `[-f file]` - Specify an alternative configuration file
    #[cfg(feature = "tmux_0_8")]
    pub fn file<S: Into<Cow<'a, str>>>(&mut self, file: S) -> &mut Self {
        self.0.args.push_option(F_LOWERCASE_KEY, file);
        self
    }

    /// `[-L socket-name]` - Allow a different socket name to be specified
    #[cfg(feature = "tmux_0_8")]
    pub fn socket_name<S: Into<Cow<'a, str>>>(&mut self, socket_name: S) -> &mut Self {
        self.0.args.push_option(L_UPPERCASE_KEY, socket_name);
        self
    }

    /// `[-S socket-path]` - Specify a full alternative path to the server socket
    #[cfg(feature = "tmux_0_8")]
    pub fn socket_path<S: Into<Cow<'a, str>>>(&mut self, socket_path: S) -> &mut Self {
        self.0.args.push_option(S_UPPERCASE_KEY, socket_path);
        self
    }

    /// `[-T features]` - Set terminal features for the client
    #[cfg(feature = "tmux_3_2")]
    pub fn features<S: Into<Cow<'a, str>>>(&mut self, features: S) -> &mut Self {
        self.0.args.push_option(T_UPPERCASE_KEY, features);
        self
    }
}
