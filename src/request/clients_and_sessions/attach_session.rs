use crate::error::Error;
use crate::tmux_interface::*;
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

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const ATTACH_SESSION: &'static str = "attach-session";

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
}
