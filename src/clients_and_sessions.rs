use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// Structure for attaching client to already existing session
///
/// # Manual
///
/// ```text
/// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
#[derive(Default, Debug)]
pub struct AttachSession<'a> {
    /// [-d] - any other clients attached to the session are detached
    pub detach_other: Option<bool>,
    /// [-E] - `update-environment` option will not be applied
    pub not_update_env: Option<bool>,
    /// [-r] - signifies the client is read-only
    pub read_only: Option<bool>,
    /// [-x] - send SIGHUP to the parent process, detaching the client
    pub parent_sighup: Option<bool>,
    /// [-c working-directory] - specify starting directory
    pub cwd: Option<&'a str>,
    /// [-t target-session] - specify target session name
    pub target_session: Option<&'a str>,
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure for detaching the current client
///
/// # Manual
///
/// ```text
/// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
#[derive(Default, Debug)]
pub struct DetachClient<'a> {
    /// [-a] - kill all but the client client given with `-t`
    pub all: Option<bool>,
    /// [-P] - send SIGHUP to the parent process of the client, typically causing it to exit
    pub parent_sighup: Option<bool>,
    /// [-E shell-command] - run shell-command to replace the client
    pub shell_command: Option<&'a str>,
    /// [-s target-session] - specify the session, all clients currently attached
    pub target_session: Option<&'a str>,
    /// [-t target-client] - specify the client
    pub target_client: Option<&'a str>,
}

impl<'a> DetachClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure for creating a new session
///
/// # Manual
///
/// ```text
/// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name]
/// [-s session-name] [-t group-name] [-x width] [-y height]
/// [shell-command]
/// (alias: new)
/// ```
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

impl<'a> NewSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure for refreshing the current client
///
/// # Manual
///
/// ```text
/// tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
/// [adjustment]
/// (alias: refresh)
/// ```
#[derive(Default, Debug)]
pub struct RefreshClient<'a> {
    /// [-c] - return to tracking the cursor automatically
    pub tracking_cursor: Option<bool>,
    /// [-D] - move the visible part of a window down by `adjustment` rows
    pub down: Option<bool>,
    /// [-l] - request the clipboard from the client using the xterm(1) escape sequence
    pub request_clipboard: Option<bool>,
    /// [-L] - move the visible part of a window left by `adjustment` columns
    pub left: Option<bool>,
    /// [-R] - move the visible part of a window right by `adjustment` columns
    pub right: Option<bool>,
    /// [-S] - only update the client's status line
    pub status_line: Option<bool>,
    /// [-U] - move the visible part of a window up by `adjustment` rows
    pub up: Option<bool>,
    /// [-C XxY] - set the width and height of a control client
    pub size: Option<(usize, usize)>,
    /// [-F flags] - set a comma-separated list of flags
    pub flags: Option<&'a str>,
    /// [-t target-client] - specify the client
    pub target_client: Option<&'a str>,
    /// [adjustment] - moves the visible part up/down left/right by adjustment rows/columns
    pub adjustment: Option<usize>,
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure to switch the current session for client `target-client` to `target-session`
///
/// # Manual
///
/// ```text
/// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
#[derive(Default, Debug)]
pub struct SwitchClient<'a> {
    /// [-E] - update-environment option will not be applied
    pub not_update_env: Option<bool>,
    /// [-l] - move to the last session
    pub last: Option<bool>,
    /// [-n] - move to the next session
    pub next: Option<bool>,
    /// [-p] - move to the previous session
    pub previous: Option<bool>,
    /// [-r] - toggle whether a client is read-only
    pub read_only: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    pub keep_zoomed: Option<bool>,
    /// [-c target-client] - specify the target-client
    pub target_client: Option<&'a str>,
    /// [-t target-session] - specify the target session
    pub target_session: Option<&'a str>,
    /// [-T key-table] - set the client's key table
    pub key_table: Option<&'a str>,
}

impl<'a> SwitchClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const ATTACH_SESSION: &'static str = "attach-session";
    const DETACH_CLIENT: &'static str = "detach-client";
    const HAS_SESSION: &'static str = "has-session";
    const KILL_SERVER: &'static str = "kill-server";
    const KILL_SESSION: &'static str = "kill-session";
    const LIST_CLIENTS: &'static str = "list-clients";
    const LIST_COMMANDS: &'static str = "list-commands";
    const LIST_SESSIONS: &'static str = "list-sessions";
    const LOCK_CLIENT: &'static str = "lock-client";
    const LOCK_SESSION: &'static str = "lock-session";
    const NEW_SESSION: &'static str = "new-session";
    const REFRESH_CLIENT: &'static str = "refresh-client";
    const RENAME_SESSION: &'static str = "rename-session";
    const SHOW_MESSAGES: &'static str = "show-messages";
    const SOURCE_FILE: &'static str = "source-file";
    const START_SERVER: &'static str = "start-server";
    const SUSPEND_CLIENT: &'static str = "suspend-client";
    const SWITCH_CLIENT: &'static str = "switch-client";

    /// Create a new client in the current terminal and attach it to `target-session`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    pub fn attach_session(
        &mut self,
        attach_session: Option<&AttachSession>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(attach_session) = attach_session {
            if attach_session.detach_other.unwrap_or(false) {
                args.push(d_KEY);
            }
            if attach_session.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            if attach_session.read_only.unwrap_or(false) {
                args.push(r_KEY);
            }
            if attach_session.parent_sighup.unwrap_or(false) {
                args.push(x_KEY);
            }
            if let Some(s) = attach_session.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = attach_session.target_session {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::ATTACH_SESSION, &args)?;
        Ok(output)
    }

    /// Detach the current client
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
    /// (alias: detach)
    /// ```
    pub fn detach_client(&mut self, detach_client: Option<&DetachClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(detach_client) = detach_client {
            if detach_client.all.unwrap_or(false) {
                args.push(a_KEY);
            }
            if detach_client.parent_sighup.unwrap_or(false) {
                args.push(P_KEY);
            }
            if let Some(s) = detach_client.shell_command {
                args.extend_from_slice(&[E_KEY, &s])
            }
            if let Some(s) = detach_client.target_session {
                args.extend_from_slice(&[s_KEY, &s])
            }
            if let Some(s) = detach_client.target_client {
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::DETACH_CLIENT, &args)?;
        Ok(output)
    }

    // XXX: better result return?
    /// Report if the specified session exist
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux has-session [-t target-session]
    /// (alias: has)
    /// ```
    pub fn has_session(&mut self, target_session: Option<&str>) -> Result<bool, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::HAS_SESSION, &args)?;
        Ok(output.status.success())
    }

    /// Kill the tmux server and clients and destroy all sessions
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-server
    /// ```
    pub fn kill_server(&mut self) -> Result<Output, Error> {
        let output = self.subcommand(TmuxInterface::KILL_SERVER, &[""])?;
        Ok(output)
    }

    /// Destroy the given session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-session [-aC] [-t target-session]
    /// ```
    pub fn kill_session(
        &mut self,
        all: Option<bool>,
        clear_alerts: Option<bool>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        if clear_alerts.unwrap_or(false) {
            args.push(C_KEY);
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::KILL_SESSION, &args)?;
        Ok(output)
    }

    /// List all clients attached to the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-clients [-F format] [-t target-session]
    /// (alias: lsc)
    /// ```
    pub fn list_clients(
        &mut self,
        format: Option<&str>,
        target_session: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_CLIENTS, &args)?;
        Ok(output)
    }

    /// List the syntax of all commands supported by tmux
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-commands [-F format]
    /// (alias: lscm)
    /// ```
    pub fn list_commands(&mut self, format: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::LIST_COMMANDS, &args)?;
        Ok(output)
    }

    // XXX: better result return?
    /// List all sessions managed by the server
    /// # Manual
    ///
    /// ```text
    /// tmux list-sessions [-F format]
    /// (alias: ls)
    /// ```
    pub fn list_sessions(&mut self, format: Option<&str>) -> Result<String, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = format {
            args.extend_from_slice(&[F_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LIST_SESSIONS, &args)?;
        let stdout = String::from_utf8_lossy(&output.stdout.as_slice());
        Ok(stdout.to_string())
    }

    /// Lock `target-client`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux lock-client [-t target-client]
    /// (alias: lockc)
    /// ```
    pub fn lock_client(&mut self, target_client: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LOCK_CLIENT, &args)?;
        Ok(output)
    }

    /// Lock all clients attached to `target-session`
    /// # Manual
    ///
    /// ```text
    /// tmux lock-session [-t target-session]
    /// (alias: locks)
    /// ```
    pub fn lock_session(&mut self, target_session: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, s])
        }
        let output = self.subcommand(TmuxInterface::LOCK_SESSION, &[""])?;
        Ok(output)
    }

    /// Create a new session with name `session-name`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux new-session [-AdDEPX] [-c start-directory] [-F format] [-n window-name]
    /// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    /// (alias: new)
    /// ```
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

    /// Refresh the current client
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux refresh-client [-cDlLRSU] [-C XxY] [-F flags] [-t target-client]
    /// [adjustment]
    /// (alias: refresh)
    /// ```
    pub fn refresh_client(
        &mut self,
        refresh_client: Option<&RefreshClient>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        let n;
        if let Some(refresh_client) = refresh_client {
            if refresh_client.tracking_cursor.unwrap_or(false) {
                args.push(c_KEY);
            }
            if refresh_client.down.unwrap_or(false) {
                args.push(D_KEY);
            }
            if refresh_client.request_clipboard.unwrap_or(false) {
                args.push(l_KEY);
            }
            if refresh_client.left.unwrap_or(false) {
                args.push(L_KEY);
            }
            if refresh_client.right.unwrap_or(false) {
                args.push(R_KEY);
            }
            if refresh_client.status_line.unwrap_or(false) {
                args.push(S_KEY);
            }
            if let Some(size) = refresh_client.size {
                s = format!("{}x{}", size.0, size.1);
                args.extend_from_slice(&[C_KEY, &s]);
            }
            if let Some(s) = refresh_client.flags {
                args.extend_from_slice(&[F_KEY, &s])
            }
            if let Some(s) = refresh_client.target_client {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(adjustment) = refresh_client.adjustment {
                n = adjustment.to_string();
                args.push(&n);
            }
        }
        let output = self.subcommand(TmuxInterface::REFRESH_CLIENT, &args)?;
        Ok(output)
    }

    /// Rename the session to `new-name`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rename-session [-t target-session] new-name
    /// (alias: rename)
    /// ```
    pub fn rename_session(
        &mut self,
        target_session: Option<&str>,
        new_name: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_session {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_SESSION, &args)?;
        Ok(output)
    }

    /// Show client messages or server information
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux show-messages [-JT] [-t target-client]
    /// (alias: showmsgs)
    /// ```
    pub fn show_messages(
        &mut self,
        jobs: Option<bool>,
        terminal: Option<bool>,
        target_client: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if jobs.unwrap_or(false) {
            args.push(J_KEY);
        }
        if terminal.unwrap_or(false) {
            args.push(T_KEY);
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SHOW_MESSAGES, &args)?;
        Ok(output)
    }

    /// Execute commands from path
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux source-file [-nqv] path
    /// (alias: source)
    /// ```
    pub fn source_file(
        &mut self,
        not_execute: Option<bool>,
        quite: Option<bool>,
        show_parsed: Option<bool>,
        path: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if not_execute.unwrap_or(false) {
            args.push(n_KEY);
        }
        if quite.unwrap_or(false) {
            args.push(q_KEY);
        }
        if show_parsed.unwrap_or(false) {
            args.push(v_KEY);
        }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SOURCE_FILE, &args)?;
        Ok(output)
    }

    /// Start the tmux server, if not already running, without creating any sessions
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux start-server
    /// (alias: start)
    /// ```
    pub fn start_server(&mut self) -> Result<Output, Error> {
        let output = self.subcommand(TmuxInterface::START_SERVER, &[""])?;
        Ok(output)
    }

    /// Suspend a client by sending SIGTSTP (tty stop)
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux suspend-client [-t target-client]
    /// (alias: suspendc)
    /// ```
    pub fn suspend_client(&mut self, target_client: Option<&str>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        let output = self.subcommand(TmuxInterface::SUSPEND_CLIENT, &args)?;
        Ok(output)
    }

    /// Switch the current session for client `target-client` to `target-session`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    pub fn switch_client(&mut self, switch_client: Option<&SwitchClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(switch_client) = switch_client {
            if switch_client.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            if switch_client.last.unwrap_or(false) {
                args.push(l_KEY);
            }
            if switch_client.next.unwrap_or(false) {
                args.push(n_KEY);
            }
            if switch_client.previous.unwrap_or(false) {
                args.push(p_KEY);
            }
            if switch_client.read_only.unwrap_or(false) {
                args.push(r_KEY);
            }
            if switch_client.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
            }
            if let Some(s) = switch_client.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = switch_client.target_session {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = switch_client.key_table {
                args.extend_from_slice(&[T_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SWITCH_CLIENT, &args)?;
        Ok(output)
    }
}
