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
