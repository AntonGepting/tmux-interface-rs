use crate::tmux_interface::*;
use crate::{Tmux, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

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
#[derive(Debug)]
pub struct NewSession<'a>(TmuxCommand<'a>);

impl<'a> Default for NewSession<'a> {
    fn default() -> Self {
        NewSession(TmuxCommand {
            bin: None,
            bin_args: None,
            cmd: Some(NewSession::NEW_SESSION.into()),
            cmd_args: None,
        })
    }
}

impl<'a> NewSession<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const NEW_SESSION: &'static str = "new-session";
    #[cfg(feature = "use_cmd_alias")]
    const NEW_SESSION: &'static str = "new";

    pub fn new() -> Self {
        Default::default()
    }

    /// [-A] - behave like `attach-session` if `session-name` already exists
    #[cfg(feature = "tmux_1_8")]
    pub fn attach(&mut self) -> &mut Self {
        self.0.push_flag(A_KEY);
        self
    }

    /// [-d] - new session is not attached to the current terminal
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(d_KEY);
        self
    }

    /// [-D] - any other clients attached to the session are detached
    #[cfg(feature = "tmux_1_8")]
    pub fn detach_other(&mut self) -> &mut Self {
        self.0.push_flag(D_KEY);
        self
    }

    /// [-E] - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.0.push_flag(E_KEY);
        self
    }

    /// [-P] - print information about the new session after it has been created
    #[cfg(feature = "tmux_1_8")]
    pub fn print(&mut self) -> &mut Self {
        self.0.push_flag(P_KEY);
        self
    }

    /// [-X] - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.0.push_flag(X_KEY);
        self
    }

    /// [-c start-directory] - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(&mut self, start_directory: S) -> &mut Self {
        self.0.push_option(c_KEY, start_directory);
        self
    }

    /// [-F format] - specify different format
    #[cfg(feature = "tmux_1_8")]
    pub fn format<S: Into<Cow<'a, str>>>(&mut self, format: S) -> &mut Self {
        self.0.push_option(F_KEY, format);
        self
    }

    /// [-n window-name] - window name of the initial window
    #[cfg(feature = "tmux_0_8")]
    pub fn window_name<S: Into<Cow<'a, str>>>(&mut self, window_name: S) -> &mut Self {
        self.0.push_option(n_KEY, window_name);
        self
    }

    /// [-s session-name] - specify a session name
    #[cfg(feature = "tmux_0_8")]
    pub fn session_name<S: Into<Cow<'a, str>>>(&mut self, session_name: S) -> &mut Self {
        self.0.push_option(s_KEY, session_name);
        self
    }

    /// [-t group-name] - specify a session group
    #[cfg(feature = "tmux_2_4")]
    pub fn group_name<S: Into<Cow<'a, str>>>(&mut self, group_name: S) -> &mut Self {
        self.0.push_option(t_KEY, group_name);
        self
    }

    /// [-x width] - specify a different width
    #[cfg(feature = "tmux_1_6")]
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.0.push_option(x_KEY, width.to_string());
        self
    }

    /// [-y height] - specify a different height
    #[cfg(feature = "tmux_1_6")]
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.0.push_option(y_KEY, height.to_string());
        self
    }

    /// [shell-command] - shell command to execute in the initial window
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    /// run command
    pub fn exec(&self) -> TmuxOutput {
        self.0.exec()
    }
}

impl<'a> From<NewSession<'a>> for TmuxCommand<'a> {
    fn from(item: NewSession<'a>) -> Self {
        item.0
    }
}

impl<'a> From<Tmux<'a>> for NewSession<'a> {
    fn from(item: Tmux<'a>) -> Self {
        NewSession(TmuxCommand {
            bin: item.0.bin,
            cmd: Some(NewSession::NEW_SESSION.into()),
            ..Default::default()
        })
    }
}

impl<'a> From<&Tmux<'a>> for NewSession<'a> {
    fn from(item: &Tmux<'a>) -> Self {
        NewSession(TmuxCommand {
            bin: item.0.bin.clone(),
            cmd: Some(NewSession::NEW_SESSION.into()),
            ..Default::default()
        })
    }
}

impl<'a> From<&mut Tmux<'a>> for NewSession<'a> {
    fn from(item: &mut Tmux<'a>) -> Self {
        NewSession(TmuxCommand {
            bin: item.0.bin.clone(),
            cmd: Some(NewSession::NEW_SESSION.into()),
            ..Default::default()
        })
    }
}

impl<'a> From<TmuxCommand<'a>> for NewSession<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        let mut command: TmuxCommand = item.into();
        command.cmd = Some(NewSession::NEW_SESSION.into());
        NewSession(command)
    }
}

//impl<'a> From<&TmuxCommand<'a>> for &NewSession<'a> {
//fn from(item: &TmuxCommand<'a>) -> Self {
//item.cmd = Some(NewSession::NEW_SESSION.into());
//&NewSession(item.to_owned())
//}
//}
