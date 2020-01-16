use crate::error::Error;
use crate::tmux_interface::*;

/// Structure for creating a new session
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name]
/// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name]
/// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
/// (alias: new)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct NewSession<'a> {
    /// [-A] - behave like `attach-session` if `session-name` already exists
    pub attach: Option<bool>,
    /// [-d] - new session is not attached to the current terminal
    pub detached: Option<bool>,
    /// [-D] - any other clients attached to the session are detached
    pub detach_other: Option<bool>,
    /// [-E] - `update-environment` option will not be applied
    pub not_update_env: Option<bool>,
    /// [-P] - print information about the new session after it has been created
    pub print: Option<bool>,
    /// [-X] - send SIGHUP to the parent process, detaching the client
    pub parent_sighup: Option<bool>,
    /// [-c start-directory] - specify starting directory
    pub cwd: Option<&'a str>,
    /// [-F format] - specify different format
    pub format: Option<&'a str>,
    /// [-n window-name] - window name of the initial window
    pub window_name: Option<&'a str>,
    /// [-s session-name] - specify a session name
    pub session_name: Option<&'a str>,
    /// [-t group-name] - specify a session group
    pub group_name: Option<&'a str>,
    /// [-x width] - specify a different width
    pub width: Option<usize>,
    /// [-y height] - specify a different height
    pub height: Option<usize>,
    /// [shell-command] - shell command to execute in the initial window
    pub shell_command: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct NewSession<'a> {
    /// [-A] - behave like `attach-session` if `session-name` already exists
    pub attach: Option<bool>,
    /// [-d] - new session is not attached to the current terminal
    pub detached: Option<bool>,
    /// [-D] - any other clients attached to the session are detached
    pub detach_other: Option<bool>,
    /// [-E] - `update-environment` option will not be applied
    pub not_update_env: Option<bool>,
    /// [-P] - print information about the new session after it has been created
    pub print: Option<bool>,
    /// [-c start-directory] - specify starting directory
    pub cwd: Option<&'a str>,
    /// [-F format] - specify different format
    pub format: Option<&'a str>,
    /// [-n window-name] - window name of the initial window
    pub window_name: Option<&'a str>,
    /// [-s session-name] - specify a session name
    pub session_name: Option<&'a str>,
    /// [-t group-name] - specify a session group
    pub group_name: Option<&'a str>,
    /// [-x width] - specify a different width
    pub width: Option<usize>,
    /// [-y height] - specify a different height
    pub height: Option<usize>,
    /// [shell-command] - shell command to execute in the initial window
    pub shell_command: Option<&'a str>,
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
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name]
    /// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    /// (alias: new)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name]
    /// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    /// (alias: new)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn new_session(&mut self, new_session: Option<&NewSession>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(new_session) = new_session {
            if new_session.attach.unwrap_or(false) {
                args.push(A_KEY);
            }
            if new_session.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if new_session.detach_other.unwrap_or(false) {
                args.push(D_KEY);
            }
            if new_session.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            if new_session.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if new_session.parent_sighup.unwrap_or(false) {
                args.push(X_KEY);
            }
            if let Some(s) = new_session.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = new_session.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_session.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(s) = new_session.session_name {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = new_session.group_name {
                args.extend_from_slice(&[t_KEY, &s])
            }
            //new_session.width.map(|n| args.extend_from_slice(&[x_KEY, &n.to_string()]));
            if let Some(width) = new_session.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = new_session.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(s) = new_session.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::NEW_SESSION, &args)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
            Ok(stdout.to_string())
        } else {
            let stdout = String::from_utf8_lossy(&output.stderr.as_slice());
            Err(Error::new(&stdout))
        }
    }

    /// Create a new session with name `session-name`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name]
    /// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    /// (alias: new)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name]
    /// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    /// (alias: new)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn new_session(&mut self, new_session: Option<&NewSession>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(new_session) = new_session {
            if new_session.attach.unwrap_or(false) {
                args.push(A_KEY);
            }
            if new_session.detached.unwrap_or(false) {
                args.push(d_KEY);
            }
            if new_session.detach_other.unwrap_or(false) {
                args.push(D_KEY);
            }
            if new_session.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            if new_session.print.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = new_session.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = new_session.format {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = new_session.window_name {
                args.extend_from_slice(&[n_KEY, &s])
            }
            if let Some(s) = new_session.session_name {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = new_session.group_name {
                args.extend_from_slice(&[t_KEY, &s])
            }
            //new_session.width.map(|n| args.extend_from_slice(&[x_KEY, &n.to_string()]));
            if let Some(width) = new_session.width {
                x = width.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(height) = new_session.height {
                y = height.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
            if let Some(s) = new_session.shell_command {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::NEW_SESSION, &args)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
            Ok(stdout.to_string())
        } else {
            let stdout = String::from_utf8_lossy(&output.stderr.as_slice());
            Err(Error::new(&stdout))
        }
    }
}
