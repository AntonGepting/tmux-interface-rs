use crate::error::Error;
use crate::tmux_interface::*;

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
#[derive(Default, Debug)]
pub struct NewSession<'a> {
    /// [-A] - behave like `attach-session` if `session-name` already exists
    #[cfg(feature = "tmux_1_8")]
    pub attach: Option<bool>,
    /// [-d] - new session is not attached to the current terminal
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    /// [-D] - any other clients attached to the session are detached
    #[cfg(feature = "tmux_1_8")]
    pub detach_other: Option<bool>,
    /// [-E] - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: Option<bool>,
    /// [-P] - print information about the new session after it has been created
    #[cfg(feature = "tmux_1_8")]
    pub print: Option<bool>,
    /// [-X] - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub parent_sighup: Option<bool>,
    /// [-c start-directory] - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub cwd: Option<&'a str>,
    /// [-F format] - specify different format
    #[cfg(feature = "tmux_1_8")]
    pub format: Option<&'a str>,
    /// [-n window-name] - window name of the initial window
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<&'a str>,
    /// [-s session-name] - specify a session name
    #[cfg(feature = "tmux_0_8")]
    pub session_name: Option<&'a str>,
    /// [-t group-name] - specify a session group
    #[cfg(feature = "tmux_2_4")]
    pub group_name: Option<&'a str>,
    /// [-x width] - specify a different width
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
    /// [-y height] - specify a different height
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    /// [shell-command] - shell command to execute in the initial window
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct NewSessionBuilder<'a> {
    #[cfg(feature = "tmux_1_8")]
    pub attach: Option<bool>,
    #[cfg(feature = "tmux_0_8")]
    pub detached: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub detach_other: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: Option<bool>,
    #[cfg(feature = "tmux_1_8")]
    pub print: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub parent_sighup: Option<bool>,
    #[cfg(feature = "tmux_1_9")]
    pub cwd: Option<&'a str>,
    #[cfg(feature = "tmux_1_8")]
    pub format: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub window_name: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub session_name: Option<&'a str>,
    #[cfg(feature = "tmux_2_4")]
    pub group_name: Option<&'a str>,
    #[cfg(feature = "tmux_1_6")]
    pub width: Option<usize>,
    #[cfg(feature = "tmux_1_6")]
    pub height: Option<usize>,
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a> NewSessionBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn attach(&mut self) -> &mut Self {
        self.attach = Some(true);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn detach_other(&mut self) -> &mut Self {
        self.detach_other = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn print(&mut self) -> &mut Self {
        self.print = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.parent_sighup = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_9")]
    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn window_name(&mut self, window_name: &'a str) -> &mut Self {
        self.window_name = Some(window_name);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn session_name(&mut self, session_name: &'a str) -> &mut Self {
        self.session_name = Some(session_name);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn group_name(&mut self, group_name: &'a str) -> &mut Self {
        self.group_name = Some(group_name);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = Some(height);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> NewSession<'a> {
        NewSession {
            #[cfg(feature = "tmux_1_8")]
            attach: self.attach,
            #[cfg(feature = "tmux_0_8")]
            detached: self.detached,
            #[cfg(feature = "tmux_1_8")]
            detach_other: self.detach_other,
            #[cfg(feature = "tmux_2_1")]
            not_update_env: self.not_update_env,
            #[cfg(feature = "tmux_1_8")]
            print: self.print,
            #[cfg(feature = "tmux_3_0")]
            parent_sighup: self.parent_sighup,
            #[cfg(feature = "tmux_1_9")]
            cwd: self.cwd,
            #[cfg(feature = "tmux_1_8")]
            format: self.format,
            #[cfg(feature = "tmux_0_8")]
            window_name: self.window_name,
            #[cfg(feature = "tmux_0_8")]
            session_name: self.session_name,
            #[cfg(feature = "tmux_2_4")]
            group_name: self.group_name,
            #[cfg(feature = "tmux_1_6")]
            width: self.width,
            #[cfg(feature = "tmux_1_6")]
            height: self.height,
            #[cfg(feature = "tmux_1_2")]
            shell_command: self.shell_command,
        }
    }
}

impl<'a> NewSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> TmuxInterface<'a> {
    const NEW_SESSION: &'static str = "new-session";

    /// Create a new session with name `session-name`
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
    pub fn new_session(&mut self, new_session: Option<&NewSession>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
            #[cfg(feature = "tmux_1_6")]
        let x;
            #[cfg(feature = "tmux_1_6")]
        let y;

        if let Some(new_session) = new_session {
            #[cfg(feature = "tmux_1_8")]
            if new_session.attach.unwrap_or(false) {
                args.push(A_KEY);
            }
            #[cfg(feature = "tmux_0_8")]
            if new_session.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if new_session.detach_other.unwrap_or(false) {
                args.push(D_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if new_session.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            #[cfg(feature = "tmux_1_8")]
            if new_session.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if new_session.parent_sighup.unwrap_or(false) {
                args.push(X_KEY);
            }
            #[cfg(feature = "tmux_1_9")]
            if let Some(s) = new_session.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_1_8")]
            if let Some(s) = new_session.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(s) = new_session.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(s) = new_session.session_name {
                args.extend_from_slice(&[s_KEY, &s])
            }
            #[cfg(feature = "tmux_2_4")]
            if let Some(s) = new_session.group_name {
                args.extend_from_slice(&[t_KEY, &s])
            }
            //new_session.width.map(|n| args.extend_from_slice(&[x_KEY, &n.to_string()]));
            #[cfg(feature = "tmux_1_6")]
            if let Some(width) = new_session.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            #[cfg(feature = "tmux_1_6")]
            if let Some(height) = new_session.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            #[cfg(feature = "tmux_1_2")]
            if let Some(s) = new_session.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::NEW_SESSION, &args)?;
        let stdout = String::from_utf8_lossy(&output.stderr.as_slice());
        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            Err(Error::Tmux(stdout.to_string()))
        }
    }
}
