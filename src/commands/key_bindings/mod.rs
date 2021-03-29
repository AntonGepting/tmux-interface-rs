use crate::TmuxCommand;
#[cfg(feature = "tmux_0_8")]
use crate::{BindKey, ListKeys, SendKeys, SendPrefix, UnbindKey};

/// All functions from man tmux "Key Bindings" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))
#[cfg(feature = "tmux_0_8")]
pub mod bind_key;
#[cfg(feature = "tmux_0_8")]
pub mod list_keys;
#[cfg(feature = "tmux_0_8")]
pub mod send_keys;
#[cfg(feature = "tmux_0_8")]
pub mod send_prefix;
#[cfg(feature = "tmux_0_8")]
pub mod unbind_key;

#[cfg(feature = "tmux_0_8")]
pub mod bind_key_tests;
#[cfg(feature = "tmux_0_8")]
pub mod list_keys_tests;
#[cfg(feature = "tmux_0_8")]
pub mod send_keys_tests;
#[cfg(feature = "tmux_0_8")]
pub mod send_prefix_tests;
#[cfg(feature = "tmux_0_8")]
pub mod unbind_key_tests;

/// All functions from man tmux "Key Bindings" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn bind_key(&self) -> BindKey<'a> {
        BindKey::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_keys(&self) -> ListKeys<'a> {
        ListKeys::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn send_keys(&self) -> SendKeys<'a> {
        SendKeys::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn send_prefix(&self) -> SendPrefix<'a> {
        SendPrefix::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn unbind_key(&self) -> UnbindKey<'a> {
        UnbindKey::from(self)
    }
}
