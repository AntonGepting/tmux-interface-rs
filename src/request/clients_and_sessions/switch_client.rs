use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

/// Structure to switch the current session for client `target-client` to `target-session`
///
/// # Manual
///
/// tmux X.X:
/// ```text
/// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
///
/// tmux 2.6:
/// ```text
/// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
#[cfg(not(feature = "tmux_2_6"))]
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
    pub target_session: Option<&'a TargetSession<'a>>,
    /// [-T key-table] - set the client's key table
    pub key_table: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
#[derive(Default, Debug)]
pub struct SwitchClientBuilder<'a> {
    pub not_update_env: Option<bool>,
    pub last: Option<bool>,
    pub next: Option<bool>,
    pub previous: Option<bool>,
    pub read_only: Option<bool>,
    pub keep_zoomed: Option<bool>,
    pub target_client: Option<&'a str>,
    pub target_session: Option<&'a TargetSession<'a>>,
    pub key_table: Option<&'a str>,
}

#[cfg(not(feature = "tmux_2_6"))]
impl<'a> SwitchClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = Some(true);
        self
    }

    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = Some(true);
        self
    }

    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    pub fn key_table(&mut self, key_table: &'a str) -> &mut Self {
        self.key_table = Some(key_table);
        self
    }

    pub fn build(&self) -> SwitchClient<'a> {
        SwitchClient {
            not_update_env: self.not_update_env,
            last: self.last,
            next: self.next,
            previous: self.previous,
            read_only: self.read_only,
            keep_zoomed: self.keep_zoomed,
            target_client: self.target_client,
            target_session: self.target_session,
            key_table: self.key_table,
        }
    }
}

#[cfg(feature = "tmux_2_6")]
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
    /// [-c target-client] - specify the target-client
    pub target_client: Option<&'a str>,
    /// [-t target-session] - specify the target session
    pub target_session: Option<&'a TargetSession<'a>>,
    /// [-T key-table] - set the client's key table
    pub key_table: Option<&'a str>,
}

impl<'a> SwitchClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(feature = "tmux_2_6")]
#[derive(Default, Debug)]
pub struct SwitchClientBuilder<'a> {
    pub not_update_env: Option<bool>,
    pub last: Option<bool>,
    pub next: Option<bool>,
    pub previous: Option<bool>,
    pub read_only: Option<bool>,
    pub target_client: Option<&'a str>,
    pub target_session: Option<&'a TargetSession<'a>>,
    pub key_table: Option<&'a str>,
}

#[cfg(feature = "tmux_2_6")]
impl<'a> SwitchClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    pub fn last(&mut self) -> &mut Self {
        self.last = Some(true);
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.next = Some(true);
        self
    }

    pub fn previous(&mut self) -> &mut Self {
        self.previous = Some(true);
        self
    }

    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = Some(true);
        self
    }

    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    pub fn key_table(&mut self, key_table: &'a str) -> &mut Self {
        self.key_table = Some(key_table);
        self
    }

    pub fn build(&self) -> SwitchClient<'a> {
        SwitchClient {
            not_update_env: self.not_update_env,
            last: self.last,
            next: self.next,
            previous: self.previous,
            read_only: self.read_only,
            target_client: self.target_client,
            target_session: self.target_session,
            key_table: self.key_table,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const SWITCH_CLIENT: &'static str = "switch-client";

    /// Switch the current session for client `target-client` to `target-session`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    #[cfg(not(feature = "tmux_2_6"))]
    pub fn switch_client(&mut self, switch_client: Option<&SwitchClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(target_session) = switch_client.target_session {
                s = target_session.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = switch_client.key_table {
                args.extend_from_slice(&[T_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SWITCH_CLIENT, &args)?;
        Ok(output)
    }

    /// Switch the current session for client `target-client` to `target-session`
    ///
    /// # Manual
    ///
    /// tmux X.X:
    /// ```text
    /// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux 2.6:
    /// ```text
    /// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    #[cfg(feature = "tmux_2_6")]
    pub fn switch_client(&mut self, switch_client: Option<&SwitchClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
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
            if let Some(s) = switch_client.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(target_session) = switch_client.target_session {
                s = target_session.to_string();
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
