use crate::error::Error;
use crate::tmux_interface::*;
use crate::TargetSession;
use std::process::Output;

/// Structure to switch the current session for client `target-client` to `target-session`
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
/// (alias: switchc)
/// ```
///
/// tmux ^1.6:
/// ```text
/// tmux switch-client [-lnpr] [-c target-client] [-t target-session]
/// (alias: switchc)
/// ```
///
/// tmux ^1.4:
/// ```text
/// tmux switch-client [-lnp] [-c target-client] [-t target-session]
/// (alias: switchc)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux switch-client [-c target-client] [-t target-session]
/// (alias: switchc)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux switch-client [-c target-client -t target-session]
/// (alias: switchc)
/// ```
#[derive(Default, Debug)]
pub struct SwitchClient<'a> {
    /// [-E] - update-environment option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: Option<bool>,
    /// [-l] - move to the last session
    #[cfg(feature = "tmux_1_4")]
    pub last_session: Option<bool>,
    /// [-n] - move to the next session
    #[cfg(feature = "tmux_1_4")]
    pub next_session: Option<bool>,
    /// [-p] - move to the previous session
    #[cfg(feature = "tmux_1_4")]
    pub previous_session: Option<bool>,
    /// [-r] - toggle whether a client is read-only
    #[cfg(feature = "tmux_1_6")]
    pub read_only: Option<bool>,
    /// [-Z] - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    /// [-c target-client] - specify the target-client
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<&'a str>,
    /// [-t target-session] - specify the target session
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<&'a TargetSession<'a>>,
    /// [-T key-table] - set the client's key table
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct SwitchClientBuilder<'a> {
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: Option<bool>,
    #[cfg(feature = "tmux_1_4")]
    pub last_session: Option<bool>,
    #[cfg(feature = "tmux_1_4")]
    pub next_session: Option<bool>,
    #[cfg(feature = "tmux_1_4")]
    pub previous_session: Option<bool>,
    #[cfg(feature = "tmux_1_6")]
    pub read_only: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: Option<bool>,
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<&'a TargetSession<'a>>,
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<&'a str>,
}

impl<'a> SwitchClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn last_session(&mut self) -> &mut Self {
        self.last_session = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn next_session(&mut self) -> &mut Self {
        self.next_session = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn previous_session(&mut self) -> &mut Self {
        self.previous_session = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_6")]
    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn target_session(&mut self, target_session: &'a TargetSession<'a>) -> &mut Self {
        self.target_session = Some(target_session);
        self
    }

    #[cfg(feature = "tmux_2_1")]
    pub fn key_table(&mut self, key_table: &'a str) -> &mut Self {
        self.key_table = Some(key_table);
        self
    }

    pub fn build(&self) -> SwitchClient<'a> {
        SwitchClient {
            #[cfg(feature = "tmux_2_1")]
            not_update_env: self.not_update_env,
            #[cfg(feature = "tmux_1_4")]
            last_session: self.last_session,
            #[cfg(feature = "tmux_1_4")]
            next_session: self.next_session,
            #[cfg(feature = "tmux_1_4")]
            previous_session: self.previous_session,
            #[cfg(feature = "tmux_1_6")]
            read_only: self.read_only,
            #[cfg(feature = "tmux_3_1")]
            keep_zoomed: self.keep_zoomed,
            #[cfg(feature = "tmux_1_0")]
            target_client: self.target_client,
            #[cfg(feature = "tmux_1_0")]
            target_session: self.target_session,
            #[cfg(feature = "tmux_2_1")]
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
    /// tmux ^3.1:
    /// ```text
    /// tmux switch-client [-ElnprZ] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux ^2.1:
    /// ```text
    /// tmux switch-client [-Elnpr] [-c target-client] [-t target-session] [-T key-table]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux ^1.6:
    /// ```text
    /// tmux switch-client [-lnpr] [-c target-client] [-t target-session]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux ^1.4:
    /// ```text
    /// tmux switch-client [-lnp] [-c target-client] [-t target-session]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux switch-client [-c target-client] [-t target-session]
    /// (alias: switchc)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux switch-client [-c target-client -t target-session]
    /// (alias: switchc)
    /// ```
    pub fn switch_client(&mut self, switch_client: Option<&SwitchClient>) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(switch_client) = switch_client {
            #[cfg(feature = "tmux_2_1")]
            if switch_client.not_update_env.unwrap_or(false) {
                args.push(E_KEY);
            }
            #[cfg(feature = "tmux_1_4")]
            if switch_client.last_session.unwrap_or(false) {
                args.push(l_KEY);
            }
            #[cfg(feature = "tmux_1_4")]
            if switch_client.next_session.unwrap_or(false) {
                args.push(n_KEY);
            }
            #[cfg(feature = "tmux_1_4")]
            if switch_client.previous_session.unwrap_or(false) {
                args.push(p_KEY);
            }
            #[cfg(feature = "tmux_1_6")]
            if switch_client.read_only.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if switch_client.keep_zoomed.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(s) = switch_client.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(target_session) = switch_client.target_session {
                s = target_session.to_string();
                args.extend_from_slice(&[t_KEY, &s])
            }
            #[cfg(feature = "tmux_2_1")]
            if let Some(s) = switch_client.key_table {
                args.extend_from_slice(&[T_KEY, &s])
            }
        }
        let output = self.subcommand(TmuxInterface::SWITCH_CLIENT, &args)?;
        Ok(output)
    }
}
