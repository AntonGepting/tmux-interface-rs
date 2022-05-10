use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SwitchClient<'a> {
    /// `[-E]` - update-environment option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub not_update_env: bool,

    /// `[-l]` - move to the last session
    #[cfg(feature = "tmux_1_4")]
    pub last_session: bool,

    /// `[-n]` - move to the next session
    #[cfg(feature = "tmux_1_4")]
    pub next_session: bool,

    /// `[-p]` - move to the previous session
    #[cfg(feature = "tmux_1_4")]
    pub previous_session: bool,

    /// `[-r]` - toggle whether a client is read-only
    #[cfg(feature = "tmux_1_6")]
    pub read_only: bool,

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-c target-client]` - specify the target-client
    #[cfg(feature = "tmux_1_0")]
    pub target_client: Option<Cow<'a, str>>,

    /// `[-t target-session]` - specify the target session
    #[cfg(feature = "tmux_1_0")]
    pub target_session: Option<Cow<'a, str>>,

    /// `[-T key-table]` - set the client's key table
    #[cfg(feature = "tmux_2_1")]
    pub key_table: Option<Cow<'a, str>>,
}

impl<'a> SwitchClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-E]` - update-environment option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.not_update_env = true;
        self
    }

    /// `[-l]` - move to the last session
    #[cfg(feature = "tmux_1_4")]
    pub fn last_session(&mut self) -> &mut Self {
        self.last_session = true;
        self
    }

    /// `[-n]` - move to the next session
    #[cfg(feature = "tmux_1_4")]
    pub fn next_session(&mut self) -> &mut Self {
        self.next_session = true;
        self
    }

    /// `[-p]` - move to the previous session
    #[cfg(feature = "tmux_1_4")]
    pub fn previous_session(&mut self) -> &mut Self {
        self.previous_session = true;
        self
    }

    /// `[-r]` - toggle whether a client is read-only
    #[cfg(feature = "tmux_1_6")]
    pub fn read_only(&mut self) -> &mut Self {
        self.read_only = true;
        self
    }

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-c target-client]` - specify the target-client
    #[cfg(feature = "tmux_1_0")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.target_client = Some(target_client.into());
        self
    }

    /// `[-t target-session]` - specify the target session
    #[cfg(feature = "tmux_1_0")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// `[-T key-table]` - set the client's key table
    #[cfg(feature = "tmux_2_1")]
    pub fn key_table<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.key_table = Some(key_table.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SWITCH_CLIENT);

        // `[-E]` - update-environment option will not be applied
        #[cfg(feature = "tmux_2_1")]
        if self.not_update_env {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-l]` - move to the last session
        #[cfg(feature = "tmux_1_4")]
        if self.last_session {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-n]` - move to the next session
        #[cfg(feature = "tmux_1_4")]
        if self.next_session {
            cmd.push_flag(N_LOWERCASE_KEY);
        }

        // `[-p]` - move to the previous session
        #[cfg(feature = "tmux_1_4")]
        if self.previous_session {
            cmd.push_flag(P_LOWERCASE_KEY);
        }

        // `[-r]` - toggle whether a client is read-only
        #[cfg(feature = "tmux_1_6")]
        if self.read_only {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-Z]` - keep the window zoomed if it was zoomed
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-c target-client]` - specify the target-client
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_client) = &self.target_client {
            cmd.push_option(C_LOWERCASE_KEY, target_client.as_ref());
        }

        // `[-t target-session]` - specify the target session
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        // `[-T key-table]` - set the client's key table
        #[cfg(feature = "tmux_2_1")]
        if let Some(key_table) = &self.key_table {
            cmd.push_option(T_UPPERCASE_KEY, key_table.as_ref());
        }

        cmd
    }
}
