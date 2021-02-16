use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Structure for attaching client to already existing session
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux attach-session [-dr] [-c working-directory] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux attach-session [-dr] [-t target-session]
/// (alias: attach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux attach-session [-d] [-t target-session]
/// (alias: attach)
/// ```
#[derive(Default, Debug)]
pub struct AttachSession<'a> {
    /// [-d] - any other clients attached to the session are detached
    #[cfg(feature = "tmux_0_8")]
    pub detach_other: Option<bool>,
    /// [-E] - `update-environment` option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: Option<bool>,
    /// [-r] - signifies the client is read-only
    #[cfg(feature = "tmux_1_2")]
    pub read_only: Option<bool>,
    /// [-x] - send SIGHUP to the parent process, detaching the client
    #[cfg(feature = "tmux_3_0")]
    pub parent_sighup: Option<bool>,
    /// [-c working-directory] - specify starting directory
    #[cfg(feature = "tmux_1_9")]
    pub cwd: Option<&'a str>,
    /// [-t target-session] - specify target session name
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<&'a str>,
}

impl<'a> AttachSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct AttachSessionBuilder<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub detach_other: Option<bool>,
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: Option<bool>,
    #[cfg(feature = "tmux_1_2")]
    pub read_only: Option<bool>,
    #[cfg(feature = "tmux_3_0")]
    pub parent_sighup: Option<bool>,
    #[cfg(feature = "tmux_1_9")]
    pub cwd: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<&'a str>,
}

impl<'a> AttachSessionBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn detach_other(&mut self) -> &mut Self {
        self.detach_other = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = Some(true);
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

    #[cfg(feature = "tmux_0_8")]
    pub fn target_session(&mut self, target_session: &'a str) -> &mut Self {
        self.target_session = Some(&target_session);
        self
    }

    pub fn build(&self) -> AttachSession<'a> {
        AttachSession {
            #[cfg(feature = "tmux_0_8")]
            detach_other: self.detach_other,
            #[cfg(feature = "tmux_2_1")]
            not_update_env: self.not_update_env,
            #[cfg(feature = "tmux_1_2")]
            read_only: self.read_only,
            #[cfg(feature = "tmux_3_0")]
            parent_sighup: self.parent_sighup,
            #[cfg(feature = "tmux_1_9")]
            cwd: self.cwd,
            #[cfg(feature = "tmux_0_8")]
            target_session: self.target_session,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const ATTACH_SESSION: &'static str = "attach-session";
    #[cfg(feature = "use_cmd_alias")]
    const ATTACH_SESSION: &'static str = "attach";

    /// # Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// tmux attach-session [-dErx] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux attach-session [-dEr] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    /// tmux ^1.9:
    /// ```text
    /// tmux attach-session [-dr] [-c working-directory] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    /// tmux ^1.2:
    /// ```text
    /// tmux attach-session [-dr] [-t target-session]
    /// (alias: attach)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux attach-session [-d] [-t target-session]
    /// (alias: attach)
    /// ```
    pub fn attach_session(
        &mut self,
        attach_session: Option<&AttachSession>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(attach_session) = attach_session {
            #[cfg(feature = "tmux_0_8")]
            if attach_session.detach_other.unwrap_or(false) {
                args.push(d_KEY);
            }
            #[cfg(feature = "tmux_2_1")]
            if attach_session.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            #[cfg(feature = "tmux_1_2")]
            if attach_session.read_only.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_3_0")]
            if attach_session.parent_sighup.unwrap_or(false) {
                args.push(x_KEY);
            }
            #[cfg(feature = "tmux_1_9")]
            if let Some(s) = attach_session.cwd {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_0_8")]
            if let Some(target_session) = attach_session.target_session {
                args.extend_from_slice(&[t_KEY, &target_session]);
            }
        }
        let output = self.command(TmuxInterface::ATTACH_SESSION, &args)?;
        Ok(output)
    }
}
