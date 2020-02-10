use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

/// Structure for attaching client to already existing session
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
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
    pub target_session: Option<&'a TargetSession<'a>>,
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct AttachSessionBuilder<'a> {
    pub detach_other: Option<bool>,
    pub not_update_env: Option<bool>,
    pub read_only: Option<bool>,
    pub parent_sighup: Option<bool>,
    pub cwd: Option<&'a str>,
    pub target_session: Option<&'a TargetSession<'a>>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a> AttachSessionBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn detach_other(&mut self) -> &mut Self {
        self.detach_other = Some(true);
        self
    }

    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = Some(true);
        self
    }

    pub fn parent_sighup(&mut self) -> &mut Self {
        self.parent_sighup = Some(true);
        self
    }

    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(&target_session);
        self
    }

    pub fn build(&self) -> AttachSession<'a> {
        AttachSession {
            detach_other: self.detach_other,
            not_update_env: self.not_update_env,
            read_only: self.read_only,
            parent_sighup: self.parent_sighup,
            cwd: self.cwd,
            target_session: self.target_session,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct AttachSession<'a> {
    /// [-d] - any other clients attached to the session are detached
    pub detach_other: Option<bool>,
    /// [-E] - `update-environment` option will not be applied
    pub not_update_env: Option<bool>,
    /// [-r] - signifies the client is read-only
    pub read_only: Option<bool>,
    /// [-c working-directory] - specify starting directory
    pub cwd: Option<&'a str>,
    /// [-t target-session] - specify target session name
    pub target_session: Option<&'a TargetSession<'a>>,
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct AttachSessionBuilder<'a> {
    pub detach_other: Option<bool>,
    pub not_update_env: Option<bool>,
    pub read_only: Option<bool>,
    pub cwd: Option<&'a str>,
    pub target_session: Option<&'a TargetSession<'a>>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a> AttachSessionBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn detach_other(&mut self) -> &mut Self {
        self.detach_other = Some(true);
        self
    }

    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = Some(true);
        self
    }

    pub fn cwd(&mut self, cwd: &'a str) -> &mut Self {
        self.cwd = Some(cwd);
        self
    }

    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(&target_session);
        self
    }

    pub fn build(&self) -> AttachSession<'a> {
        AttachSession {
            detach_other: self.detach_other,
            not_update_env: self.not_update_env,
            read_only: self.read_only,
            cwd: self.cwd,
            target_session: self.target_session,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const ATTACH_SESSION: &'static str = "attach-session";

    /// Create a new client in the current terminal and attach it to `target-session`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn attach_session(
        &mut self,
        attach_session: Option<&AttachSession>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(ref target_session) = attach_session.target_session {
                s = target_session.to_string();
                args.extend_from_slice(&[t_KEY, &s]);
            }
        }
        let output = self.subcommand(TmuxInterface::ATTACH_SESSION, &args)?;
        Ok(output)
    }

    /// Create a new client in the current terminal and attach it to `target-session`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn attach_session(
        &mut self,
        attach_session: Option<&AttachSession>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(s) = attach_session.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(attach_session) = attach_session.target_session {
                s = attach_session.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::ATTACH_SESSION, &args)?;
        Ok(output)
    }
}
