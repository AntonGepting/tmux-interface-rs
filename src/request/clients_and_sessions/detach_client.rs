use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

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

impl<'a> TmuxInterface<'a> {
    const DETACH_CLIENT: &'static str = "detach-client";

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
}
