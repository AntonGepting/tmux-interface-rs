use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

/// Structure for detaching the current client
///
/// # Manual
///
/// tmux ^2.4:
/// ```text
/// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^2.2:
/// ```text
/// tmux detach-client [-aP] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux detach-client [-P] [-s target-session] [-t target-client]
/// (alias: detach)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux detach-client [-t target-client]
/// (alias: detach)
/// ```
#[derive(Default, Debug)]
pub struct DetachClient<'a> {
    /// [-a] - kill all but the client client given with `-t`
    #[cfg(feature = "tmux_2_2")]
    pub all: Option<bool>,
    /// [-P] - send SIGHUP to the parent process of the client, typically causing it to exit
    #[cfg(feature = "tmux_1_5")]
    pub parent_sighup: Option<bool>,
    /// [-E shell-command] - run shell-command to replace the client
    #[cfg(feature = "tmux_2_4")]
    pub shell_command: Option<&'a str>,
    /// [-s target-session] - specify the session, all clients currently attached
    #[cfg(feature = "tmux_1_5")]
    pub target_session: Option<&'a TargetSession<'a>>,
    /// [-t target-client] - specify the client
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<&'a str>,
}

impl<'a> DetachClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct DetachClientBuilder<'a> {
    #[cfg(feature = "tmux_2_2")]
    pub all: Option<bool>,
    #[cfg(feature = "tmux_1_5")]
    pub parent_sighup: Option<bool>,
    #[cfg(feature = "tmux_2_4")]
    pub shell_command: Option<&'a str>,
    #[cfg(feature = "tmux_1_5")]
    pub target_session: Option<&'a TargetSession<'a>>,
    #[cfg(feature = "tmux_0_8")]
    pub target_client: Option<&'a str>,
}

impl<'a> DetachClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_2")]
    pub fn all(&mut self) -> &mut Self {
        self.all = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.parent_sighup = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_4")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    pub fn build(&self) -> DetachClient<'a> {
        DetachClient {
            #[cfg(feature = "tmux_2_2")]
            all: self.all,
            #[cfg(feature = "tmux_1_5")]
            parent_sighup: self.parent_sighup,
            #[cfg(feature = "tmux_2_4")]
            shell_command: self.shell_command,
            #[cfg(feature = "tmux_1_5")]
            target_session: self.target_session,
            #[cfg(feature = "tmux_0_8")]
            target_client: self.target_client,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const DETACH_CLIENT: &'static str = "detach-client";

    /// Detach the current client
    ///
    /// # Manual
    ///
    /// tmux ^2.4:
    /// ```text
    /// tmux detach-client [-aP] [-E shell-command] [-s target-session] [-t target-client]
    /// (alias: detach)
    /// ```
    ///
    /// tmux ^2.2:
    /// ```text
    /// tmux detach-client [-aP] [-s target-session] [-t target-client]
    /// (alias: detach)
    /// ```
    ///
    /// tmux ^1.5:
    /// ```text
    /// tmux detach-client [-P] [-s target-session] [-t target-client]
    /// (alias: detach)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux detach-client [-t target-client]
    /// (alias: detach)
    /// ```
    pub fn detach_client(&mut self, detach_client: Option<&DetachClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s: String;
        if let Some(detach_client) = detach_client {
            #[cfg(feature = "tmux_2_2")]
            {
                if detach_client.all.unwrap_or(false) {
                    args.push(a_KEY);
                }
            }
            #[cfg(feature = "tmux_1_5")]
            {
                if detach_client.parent_sighup.unwrap_or(false) {
                    args.push(P_KEY);
                }
            }
            #[cfg(feature = "tmux_2_4")]
            {
                if let Some(s) = detach_client.shell_command {
                    args.extend_from_slice(&[E_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_1_5")]
            {
                if let Some(target_session) = detach_client.target_session {
                    s = target_session.to_string();
                    args.extend_from_slice(&[s_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(s) = detach_client.target_client {
                    args.extend_from_slice(&[t_KEY, &s])
                }
            }
        }
        let output = self.subcommand(TmuxInterface::DETACH_CLIENT, &args)?;
        Ok(output)
    }
}
