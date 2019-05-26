use super::tmux_interface::*;
use super::tmux_interface_error::TmuxInterfaceError;
use std::borrow::Cow;


pub struct NewSession<'a> {
    pub attach: Option<bool>,                   // [-A]
    pub detached: Option<bool>,                 // [-d]
    pub detach_other: Option<bool>,             // [-D]
    pub not_update_env: Option<bool>,           // [-E]
    pub print: Option<bool>,                    // [-P]
    pub cwd: Option<Cow<'a, str>>,              // [-c start-directory]
    pub format: Option<Cow<'a, str>>,           // [-F format]
    pub window_name: Option<Cow<'a, str>>,      // [-n window-name]
    pub session_name: Option<Cow<'a, str>>,     // [-s session-name]
    pub group_name: Option<Cow<'a, str>>,       // [-t group-name]
    pub width: Option<usize>,                   // [-x width]
    pub height: Option<usize>,                  // [-y height]
    pub shell_command: Option<Cow<'a, str>>     // [shell-command]
}


impl<'a> Default for NewSession<'a> {
    fn default() -> Self {
        NewSession {
            attach: None,
            detached: None,
            detach_other: None,
            not_update_env: None,
            print: None,
            cwd: None,
            format: None,
            window_name: None,
            session_name: None,
            group_name: None,
            width: None,
            height: None,
            shell_command: None
        }
    }
}


impl<'a> NewSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


pub struct AttachSession<'a> {
    pub detach_other: Option<bool>,             // [-d]
    pub not_update_env: Option<bool>,           // [-E]
    pub read_only: Option<bool>,                // [-r]
    pub cwd: Option<Cow<'a, str>>,              // [-c working-directory]
    pub target_session: Option<Cow<'a, str>>,   // [-t target-session]
}


impl<'a> Default for AttachSession<'a> {
    fn default() -> Self {
        AttachSession {
            detach_other: None,
            not_update_env: None,
            read_only: None,
            cwd: None,
            target_session: None,
        }
    }
}


impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}


/// All functions from man tmux "Clients and Sessions" listed below
impl<'a> TmuxInterface<'a> {

    const NEW_SESSION: &'static str = "new-session";
    const ATTACH_SESSION: &'static str = "attach-session";
    const HAS_SESSION: &'static str = "has-session";
    const KILL_SESSION: &'static str = "kill-session";
    const LIST_SESSIONS: &'static str = "list-sessions";
    const RENAME_SESSION: &'static str = "rename-session";

    // Clients and Sessions
    // ===========================================================================================
    //
    /// ```text
    /// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    pub fn attach_session(&self, attach_session: &AttachSession) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if attach_session.detach_other.unwrap_or(false) { args.push(d_KEY); }
        if attach_session.not_update_env.unwrap_or(false) { args.push(E_KEY); }
        if attach_session.read_only.unwrap_or(false) { args.push(r_KEY); }
        attach_session.cwd.as_ref().and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        attach_session.target_session.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        let output = self.subcommand(TmuxInterface::ATTACH_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
    /// (alias: detach)
    /// ```
    pub fn detach_client() {
        unimplemented!();
    }


    /// ```text
    /// tmux has-session [-t target-session]
    /// (alias: has)
    /// ```
    pub fn has_session(&self, target_session: &str) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        args.extend_from_slice(&[t_KEY, target_session]);
        let output = self.subcommand(TmuxInterface::HAS_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// tmux kill-server
    /// ```
    pub fn kill_server() {
        unimplemented!();
    }


    /// ```text
    /// tmux kill-session [-aC] [-t target-session]
    /// ```
    pub fn kill_session(&self, name: &str, all: bool, clear_alerts: bool) -> Result<bool, TmuxInterfaceError> {
        let mut args: Vec<&str> = Vec::new();
        if all { args.push(a_KEY); }
        if clear_alerts { args.push(C_KEY); }
        args.extend_from_slice(&[t_KEY, name]);
        let output = self.subcommand(TmuxInterface::KILL_SESSION, &args)?;
        Ok(output.status.success())
    }


    /// ```text
    /// tmux list-clients [-F format] [-t target-session]
    /// (alias: lsc)
    /// ```
    pub fn list_clients() {
        unimplemented!();
    }


    /// ```text
    /// tmux list-commands [-F format]
    /// (alias: lscm)
    /// ```
    pub fn list_commands() {
        unimplemented!();
    }


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

    /// ```text
    /// tmux lock-client [-t target-client]
    /// (alias: lockc)
    /// ```
    pub fn lock_client() {
        unimplemented!();
    }

    /// ```text
    /// tmux lock-session [-t target-session]
    /// (alias: locks)
    /// ```
    pub fn lock_session() {
        unimplemented!();
    }

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
        new_session.cwd.as_ref().and_then(|s| Some(args.extend_from_slice(&[c_KEY, &s])));
        new_session.format.as_ref().and_then(|s| Some(args.extend_from_slice(&[F_KEY, &s])));
        new_session.window_name.as_ref().and_then(|s| Some(args.extend_from_slice(&[n_KEY, &s])));
        new_session.session_name.as_ref().and_then(|s| Some(args.extend_from_slice(&[s_KEY, &s])));
        new_session.group_name.as_ref().and_then(|s| Some(args.extend_from_slice(&[t_KEY, &s])));
        //new_session.width.and_then(|n| Some(args.extend_from_slice(&[x_KEY, &n.to_string()])));
        let mut x;
        if let Some(width) = new_session.width {
            x = width.to_string();
            args.extend_from_slice(&[x_KEY, &x]);
        }
        let mut y;
        if let Some(height) = new_session.height {
            y = height.to_string();
            args.extend_from_slice(&[y_KEY, &y]);
        }
        new_session.shell_command.as_ref().and_then(|s| Some(args.push(&s)));
        let output = self.subcommand(TmuxInterface::NEW_SESSION, &args)?;
        Ok(output.status.success())
    }

    /// ```text
    /// tmux refresh-client [-cDlLRSU] [-C width,height] [-t target-client] [adjustment]
    /// (alias: refresh)
    /// ```
    pub fn refresh_client() {
        unimplemented!();
    }

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

    /// ```text
    /// tmux show-messages [-JT] [-t target-client]
    /// (alias: showmsgs)
    /// ```
    pub fn show_messages() {
        unimplemented!();
    }

    /// ```text
    /// tmux source-file [-q] path
    /// (alias: source)
    /// ```
    pub fn source_file() {
        unimplemented!();
    }

    /// ```text
    /// tmux start-server
    /// (alias: start)
    /// ```
    pub fn start_server() {
        unimplemented!();
    }

    /// ```text
    /// tmux suspend-client [-t target-client]
    /// (alias: suspendc)
    /// ```
    pub fn suspend_client() {
        unimplemented!();
    }

    /// ```text
    /// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    pub fn switch_client() {
        unimplemented!();
    }


}
