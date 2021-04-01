use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug)]
pub struct SwitchClient<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SwitchClient<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SWITCH_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> SwitchClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-E]` - update-environment option will not be applied
    #[cfg(feature = "tmux_2_1")]
    pub fn not_update_env(&mut self) -> &mut Self {
        self.0.push_flag(E_UPPERCASE_KEY);
        self
    }

    /// `[-l]` - move to the last session
    #[cfg(feature = "tmux_1_4")]
    pub fn last_session(&mut self) -> &mut Self {
        self.0.push_flag(L_LOWERCASE_KEY);
        self
    }

    /// `[-n]` - move to the next session
    #[cfg(feature = "tmux_1_4")]
    pub fn next_session(&mut self) -> &mut Self {
        self.0.push_flag(N_LOWERCASE_KEY);
        self
    }

    /// `[-p]` - move to the previous session
    #[cfg(feature = "tmux_1_4")]
    pub fn previous_session(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-r]` - toggle whether a client is read-only
    #[cfg(feature = "tmux_1_6")]
    pub fn read_only(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-c target-client]` - specify the target-client
    #[cfg(feature = "tmux_1_0")]
    pub fn target_client(&mut self, target_client: &'a str) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, target_client);
        self
    }

    /// `[-t target-session]` - specify the target session
    #[cfg(feature = "tmux_1_0")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_session);
        self
    }

    /// `[-T key-table]` - set the client's key table
    #[cfg(feature = "tmux_2_1")]
    pub fn key_table<S: Into<Cow<'a, str>>>(&mut self, key_table: S) -> &mut Self {
        self.0.push_option(T_UPPERCASE_KEY, key_table);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SwitchClient<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SWITCH_CLIENT)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SwitchClient<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SWITCH_CLIENT)),
            ..Default::default()
        })
    }
}
