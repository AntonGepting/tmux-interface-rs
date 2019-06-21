use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;


/// Session for attaching client to already existing session
///
/// # Manual
///
/// ```text
/// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
#[derive(Default)]
pub struct AttachSession<'a> {
    /// any other clients attached to the session are detached
    pub detach_other: Option<bool>,             // [-d]
    /// `update-environment` option will not be applied
    pub not_update_env: Option<bool>,           // [-E]
    /// signifies the client is read-only
    pub read_only: Option<bool>,                // [-r]
    /// specify starting directory
    pub cwd: Option<&'a str>,              // [-c working-directory]
    /// specify target session name
    pub target_session: Option<&'a str>,   // [-t target-session]
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Detach the current client
///
/// # Manual
///
/// ```text
/// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
#[derive(Default)]
pub struct DetachClient<'a> {
    pub all: Option<bool>,                      // [-a]
    pub parent_sighup: Option<bool>,            // [-P]
    pub shell_command: Option<&'a str>,         // [-E shell-command]
    pub target_session: Option<&'a str>,        // [-s target-session]
    pub target_client: Option<&'a str>          // [-t target-client]
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
/// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name]
/// [-s session-name] [-t group-name] [-x width] [-y height]
/// [shell-command]
/// (alias: new)
/// ```
#[derive(Default)]
pub struct NewSession<'a> {
    /// behave like `attach-session` if `session-name` already exists
    pub attach: Option<bool>,                   // [-A]
    /// new session is not attached to the current terminal
    pub detached: Option<bool>,                 // [-d]
    /// any other clients attached to the session are detached
    pub detach_other: Option<bool>,             // [-D]
    /// `update-environment` option will not be applied
    pub not_update_env: Option<bool>,           // [-E]
    /// print information about the new session after it has been created
    pub print: Option<bool>,                    // [-P]
    /// specify starting directory
    pub cwd: Option<&'a str>,              // [-c start-directory]
    /// specify different format
    pub format: Option<&'a str>,           // [-F format]
    /// window name of the initial window
    pub window_name: Option<&'a str>,      // [-n window-name]
    /// specify a session name
    pub session_name: Option<&'a str>,     // [-s session-name]
    /// specify a session group
    pub group_name: Option<&'a str>,       // [-t group-name]
    /// specify a different width
    pub width: Option<usize>,                   // [-x width]
    /// specify a different height
    pub height: Option<usize>,                  // [-y height]
    /// shell command to execute in the initial window
    pub shell_command: Option<&'a str>     // [shell-command]
}

impl<'a> NewSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Refresh the current client
///
/// # Manual
///
/// ```text
/// tmux refresh-client [-cDlLRSU] [-C width,height] [-t target-client] [adjustment]
/// (alias: refresh)
/// ```
#[derive(Default)]
pub struct RefreshClient<'a> {
    pub tracking_cursor: Option<bool>,          // [-c]
    pub down: Option<bool>,                     // [-D]
    pub request_clipboard: Option<bool>,        // [-l]
    pub left: Option<bool>,                     // [-L]
    pub right: Option<bool>,                    // [-R]
    pub status_line: Option<bool>,              // [-S]
    pub up: Option<bool>,                       // [-U]
    pub size: Option<(usize, usize)>,           // [-C width,height]
    pub target_client: Option<&'a str>,         // [-t target-client]
    pub adjustment: Option<usize>               // [adjustment]
}

impl<'a> RefreshClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// Switch the current session for client `target-client` to `target-session`
///
/// # Manual
///
/// ```text
/// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
#[derive(Default)]
pub struct SwitchClient<'a> {
    pub not_update_env: Option<bool>,           // [-E]
    pub last: Option<bool>,                     // [-l]
    pub next: Option<bool>,                     // [-n]
    pub previous: Option<bool>,                 // [-p]
    pub read_only: Option<bool>,                // [-r]
    pub target_client: Option<&'a str>,         // [-c target-client]
    pub target_session: Option<&'a str>,        // [-t target-session]
    pub key_table: Option<&'a str>,             // [-T key-table]
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
    /// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    pub fn attach_session(&self, attach_session: &AttachSession) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if attach_session.detach_other.unwrap_or(false) { args.push(d_KEY); }
        if attach_session.not_update_env.unwrap_or(false) { args.push(E_KEY); }
        if attach_session.read_only.unwrap_or(false) { args.push(r_KEY); }
        attach_session.cwd.and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        attach_session.target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::ATTACH_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// Detach the current client
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
    /// (alias: detach)
    /// ```
    pub fn detach_client(&self, detach_client: &DetachClient) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if detach_client.all.unwrap_or(false) { args.push(a_KEY); }
        if detach_client.parent_sighup.unwrap_or(false) { args.push(P_KEY); }
        detach_client.shell_command.and_then(|s| Some(args.extend_from_slice(&[E_KEY, &s])));
        detach_client.target_session.and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        detach_client.target_client.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::DETACH_CLIENT, &args)?;
        Ok(output.status.success())
    }


    /// Report if the specified session exist
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux has-session [-t target-session]
    /// (alias: has)
    /// ```
    pub fn has_session(&self, target_session: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
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
    pub fn kill_server(&self) -> Result<bool, TmuxInterfaceError> {
        let output = self.subcommand(TmuxInterface::KILL_SERVER, &[""])?;
        Ok(output.status.success())
    }


    /// Destroy the given session
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux kill-session [-aC] [-t target-session]
    /// ```
    pub fn kill_session(&self, all: Option<bool>, clear_alerts: Option<bool>, target_session: Option<&str>)
        -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all.unwrap_or(false) { args.push(a_KEY); }
        if clear_alerts.unwrap_or(false) { args.push(C_KEY); }
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::KILL_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// List all clients attached to the server
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-clients [-F format] [-t target-session]
    /// (alias: lsc)
    /// ```
    pub fn list_clients(&self, format: Option<&str>, target_session: Option<&str>) ->
        Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LIST_CLIENTS, &args)?;
        Ok(output.status.success())
    }


    /// List the syntax of all commands supported by tmux
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux list-commands [-F format]
    /// (alias: lscm)
    /// ```
    pub fn list_commands(&self, format: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        let output = self.subcommand(TmuxInterface::LIST_COMMANDS, &args)?;
        Ok(output.status.success())
    }


    /// List all sessions managed by the server
    /// # Manual
    ///
    /// ```text
    /// tmux list-sessions [-F format]
    /// (alias: ls)
    /// default response form: 0: 4 windows (created Mon Apr 21 22:51:13 2019) [177x64] (attached)
    /// ```
    pub fn list_sessions(&self, format: Option<&str>) -> Result<String, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, s])));
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
    pub fn lock_client(&self, target_client: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_client.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::LOCK_CLIENT, &args)?;
        Ok(output.status.success())
    }


    /// Lock all clients attached to `target-session`
    /// # Manual
    ///
    /// ```text
    /// tmux lock-session [-t target-session]
    /// (alias: locks)
    /// ```
    pub fn lock_session(&self, target_session: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, s])));
        let output = self.subcommand(TmuxInterface::LOCK_SESSION, &[""])?;
        Ok(output.status.success())
    }


    /// Create a new session with name `session-name`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux new-session [-AdDEP] [-c start-directory] [-F format] [-n window-name]
    /// [-s session-name] [-t group-name] [-x width] [-y height] [shell-command]
    /// (alias: new)
    /// ```
    pub fn new_session(&self, new_session: &NewSession) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if new_session.attach.unwrap_or(false) { args.push(A_KEY); }
        if new_session.detached.unwrap_or(false) { args.push(d_KEY); }
        if new_session.detach_other.unwrap_or(false) { args.push(D_KEY); }
        if new_session.not_update_env.unwrap_or(false) { args.push(E_KEY); }
        if new_session.print.unwrap_or(false) { args.push(P_KEY); }
        new_session.cwd.and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        new_session.format.and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        new_session.window_name.and_then(|s| Some(args.extend_from_slice(&[n_KEY, &s])));
        new_session.session_name.and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        new_session.group_name.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        //new_session.width.and_then(|n| Some(args.extend_from_slice(&[x_KEY, &n.to_string()])));
        let x;
        if let Some(width) = new_session.width {
            x = width.to_string();
            args.extend_from_slice(&[x_KEY, &x]);
        }
        let y;
        if let Some(height) = new_session.height {
            y = height.to_string();
            args.extend_from_slice(&[y_KEY, &y]);
        }
        new_session.shell_command.and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::NEW_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// Refresh the current client
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux refresh-client [-cDlLRSU] [-C width,height] [-t target-client] [adjustment]
    /// (alias: refresh)
    /// ```
    pub fn refresh_client(&self, refresh_client: &RefreshClient) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if refresh_client.tracking_cursor.unwrap_or(false) { args.push(c_KEY); }
        if refresh_client.down.unwrap_or(false) { args.push(D_KEY); }
        if refresh_client.request_clipboard.unwrap_or(false) { args.push(l_KEY); }
        if refresh_client.left.unwrap_or(false) { args.push(L_KEY); }
        if refresh_client.right.unwrap_or(false) { args.push(R_KEY); }
        if refresh_client.status_line.unwrap_or(false) { args.push(S_KEY); }
        let s;
        if let Some(size) = refresh_client.size {
            s = format!("{},{}", size.0, size.1);
            args.extend_from_slice(&[C_KEY, &s]);
        }
        refresh_client.target_client.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let n;
        if let Some(adjustment) = refresh_client.adjustment {
            n = adjustment.to_string();
            args.push(&n);
        }
        let output = self.subcommand(TmuxInterface::REFRESH_CLIENT, &args)?;
        Ok(output.status.success())
    }


    /// Rename the session to `new-name`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux rename-session [-t target-session] new-name
    /// (alias: rename)
    /// ```
    pub fn rename_session(&self, target_session: Option<&str>, new_name: &str) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        args.push(new_name);
        let output = self.subcommand(TmuxInterface::RENAME_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// Show client messages or server information
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux show-messages [-JT] [-t target-client]
    /// (alias: showmsgs)
    /// ```
    pub fn show_messages(&self, jobs: Option<bool>, terminal: Option<bool>,
                         target_client: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if jobs.unwrap_or(false) { args.push(J_KEY); }
        if terminal.unwrap_or(false) { args.push(T_KEY); }
        target_client.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SHOW_MESSAGES, &args)?;
        Ok(output.status.success())
    }


    /// Execute commands from path
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux source-file [-q] path
    /// (alias: source)
    /// ```
    pub fn source_file(&self, quite: Option<bool>, path: &str) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if quite.unwrap_or(false) { args.push(q_KEY); }
        args.push(path);
        let output = self.subcommand(TmuxInterface::SOURCE_FILE, &args)?;
        Ok(output.status.success())
    }


    /// Start the tmux server, if not already running, without creating any sessions
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux start-server
    /// (alias: start)
    /// ```
    pub fn start_server(&self) -> Result<bool, TmuxInterfaceError> {
        let output = self.subcommand(TmuxInterface::START_SERVER, &[""])?;
        Ok(output.status.success())
    }


    /// Suspend a client by sending SIGTSTP (tty stop)
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux suspend-client [-t target-client]
    /// (alias: suspendc)
    /// ```
    pub fn suspend_client(&self, target_client: Option<&str>) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        target_client.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SUSPEND_CLIENT, &args)?;
        Ok(output.status.success())
    }


    /// Switch the current session for client `target-client` to `target-session`
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    pub fn switch_client(&self, switch_client: &SwitchClient) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if switch_client.not_update_env.unwrap_or(false) { args.push(E_KEY); }
        if switch_client.last.unwrap_or(false) { args.push(l_KEY); }
        if switch_client.next.unwrap_or(false) { args.push(n_KEY); }
        if switch_client.previous.unwrap_or(false) { args.push(p_KEY); }
        if switch_client.read_only.unwrap_or(false) { args.push(r_KEY); }
        switch_client.target_client.and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        switch_client.target_session.and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        switch_client.key_table.and_then(|s| Some(args.extend_from_slice(&[T_KEY, &s])));
        let output = self.subcommand(TmuxInterface::SWITCH_CLIENT, &args)?;
        Ok(output.status.success())
    }


}
