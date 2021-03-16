use crate::commands::NEW_SESSION;
use crate::tmux_interface::*;
use crate::{TmuxCommand, TmuxCommandTrait};

/// Structure for creating a new session
///
/// # Manual
///
/// tmux 3.0:
/// ```text
/// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.4:
/// ```text
/// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.1:
/// ```text
/// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.9:
/// ```text
/// tmux new-session [-AdDP] [-c start-directory] [-F format] [-n window-name] [-s session-name]
/// [-t target-session] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.8:
/// ```text
/// tmux new-session [-AdDP] [-F format] [-n window-name] [-s session-name] [-t target-session]
/// [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.6:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [-x width]
/// [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.2:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 1.1:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [-t target-session] [command]
/// (alias: new)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux new-session [-d] [-n window-name] [-s session-name] [command]
/// (alias: new)
/// ```

//impl Default for NewSession {
//fn default() -> Self {
//NewSession(TmuxCommand {
//bin: None,
//bin_args: None,
//cmd: Some(NewSession::NEW_SESSION.into()),
//cmd_args: None,
//})
//}
//}

impl NewSession for TmuxCommand {}

pub trait NewSession: TmuxCommandTrait {
    fn new() -> TmuxCommand {
        TmuxCommand {
            bin: None,
            bin_args: None,
            cmd: Some(NEW_SESSION.to_string()),
            cmd_args: None,
        }
    }

    /// [-A] - behave like `attach-session` if `session-name` already exists
    #[cfg(feature = "tmux_1_8")]
    fn attach(&mut self) -> &mut Self {
        self.push_flag(A_KEY);
        self
    }

    /// [-d] - new session is not attached to the current terminal
    #[cfg(feature = "tmux_0_8")]
    fn detached(&mut self) -> &mut Self {
        self.push_flag(d_KEY);
        self
    }

    /// [-D] - any other clients attached to the session are detached
    #[cfg(feature = "tmux_1_8")]
    fn detach_other(&mut self) -> &mut Self {
        self.push_flag(D_KEY);
        self
    }

    /// [-E] - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    fn not_update_env(&mut self) -> &mut Self {
        self.push_flag(E_KEY);
        self
    }

    /// [-P] - print information about the new session after it has been created
    #[cfg(feature = "tmux_1_8")]
    fn print(&mut self) -> &mut Self {
        self.push_flag(P_KEY);
        self
    }

    /// [-X] - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    fn parent_sighup(&mut self) -> &mut Self {
        self.push_flag(X_KEY);
        self
    }

    /// [-c start-directory] - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    fn start_directory<S: Into<String>>(&mut self, start_directory: S) -> &mut Self {
        self.push_option(c_KEY, start_directory);
        self
    }

    /// [-F format] - specify different format
    #[cfg(feature = "tmux_1_8")]
    fn format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.push_option(F_KEY, format);
        self
    }

    /// [-n window-name] - window name of the initial window
    #[cfg(feature = "tmux_0_8")]
    fn window_name<S: Into<String>>(&mut self, window_name: S) -> &mut Self {
        self.push_option(n_KEY, window_name);
        self
    }

    /// [-s session-name] - specify a session name
    #[cfg(feature = "tmux_0_8")]
    fn session_name<S: Into<String>>(&mut self, session_name: S) -> &mut Self {
        self.push_option(s_KEY, session_name);
        self
    }

    /// [-t group-name] - specify a session group
    #[cfg(feature = "tmux_2_4")]
    fn group_name<S: Into<String>>(&mut self, group_name: S) -> &mut Self {
        self.push_option(t_KEY, group_name);
        self
    }

    /// [-x width] - specify a different width
    #[cfg(feature = "tmux_1_6")]
    fn width(&mut self, width: usize) -> &mut Self {
        self.push_option(x_KEY, width.to_string());
        self
    }

    /// [-y height] - specify a different height
    #[cfg(feature = "tmux_1_6")]
    fn height(&mut self, height: usize) -> &mut Self {
        self.push_option(y_KEY, height.to_string());
        self
    }

    /// [shell-command] - shell command to execute in the initial window
    #[cfg(feature = "tmux_1_2")]
    fn shell_command<S: Into<String>>(&mut self, shell_command: S) -> &mut Self {
        self.push_param(shell_command);
        self
    }

    //fn clone_from(orig: &TmuxCommand) -> TmuxCommand {
    //let mut tmux = <TmuxCommand as NewSession>::new();
    //tmux.bin = orig.bin.clone();
    //tmux.cmd = Some(Cow::Borrowed("new-session-from"));
    //tmux
    //}
}
