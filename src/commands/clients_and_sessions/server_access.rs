// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

///  Change the access or read/write permission of user
///
/// # Manual
///
/// tmux >=3.3:
/// ```text
/// server-access [-adlrw] [user]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ServerAccess<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_3_3")]
    pub add: bool,

    /// `[-d]`
    #[cfg(feature = "tmux_3_3")]
    pub delete: bool,

    /// `[-l]`
    #[cfg(feature = "tmux_3_3")]
    pub list: bool,

    /// `[-r]`
    #[cfg(feature = "tmux_3_3")]
    pub read: bool,

    /// `[-w]`
    #[cfg(feature = "tmux_3_3")]
    pub write: bool,

    /// `[user]`
    #[cfg(feature = "tmux_3_3")]
    pub user: Option<Cow<'a, str>>,
}

impl<'a> ServerAccess<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_3_3")]
    pub fn add(mut self) -> Self {
        self.add = true;
        self
    }

    /// `[-d]`
    #[cfg(feature = "tmux_3_3")]
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    /// `[-l]`
    #[cfg(feature = "tmux_3_3")]
    pub fn list(mut self) -> Self {
        self.list = true;
        self
    }

    /// `[-r]`
    #[cfg(feature = "tmux_3_3")]
    pub fn read(mut self) -> Self {
        self.read = true;
        self
    }

    /// `[-w]`
    #[cfg(feature = "tmux_3_3")]
    pub fn write(mut self) -> Self {
        self.write = true;
        self
    }

    /// `[user]`
    #[cfg(feature = "tmux_3_3")]
    pub fn user<S: Into<Cow<'a, str>>>(mut self, user: S) -> Self {
        self.user = Some(user.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SERVER_ACCESS);

        // `[-a]`
        #[cfg(feature = "tmux_3_3")]
        if self.add {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-d]`
        #[cfg(feature = "tmux_3_3")]
        if self.delete {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-l]`
        #[cfg(feature = "tmux_3_3")]
        if self.list {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-r]`
        #[cfg(feature = "tmux_3_3")]
        if self.read {
            cmd.push_flag(R_LOWERCASE_KEY);
        }

        // `[-w]`
        #[cfg(feature = "tmux_3_3")]
        if self.write {
            cmd.push_flag(W_LOWERCASE_KEY);
        }

        // `[user]`
        #[cfg(feature = "tmux_3_3")]
        if let Some(user) = self.user {
            cmd.push_param(user);
        }

        cmd
    }
}
