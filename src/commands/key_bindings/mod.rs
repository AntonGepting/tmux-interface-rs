use crate::{BindKey, ListKeys, SendKeys, SendPrefix, TmuxCommand, UnbindKey};

/// All functions from man tmux "Key Bindings" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))
#[cfg(feature = "tmux_1_0")]
pub mod bind_key;
#[cfg(feature = "tmux_1_0")]
pub mod list_keys;
#[cfg(feature = "tmux_1_0")]
pub mod send_keys;
#[cfg(feature = "tmux_1_0")]
pub mod send_prefix;
#[cfg(feature = "tmux_1_0")]
pub mod unbind_key;

//#[cfg(feature = "tmux_1_0")]
//pub mod bind_key_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod list_keys_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod send_keys_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod send_prefix_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod unbind_key_tests;

/// All functions from man tmux "Key Bindings" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))
impl<'a> TmuxCommand<'a> {
    pub fn bind_key(&self) -> BindKey<'a> {
        BindKey::from(self)
    }

    pub fn list_keys(&self) -> ListKeys<'a> {
        ListKeys::from(self)
    }

    pub fn send_keys(&self) -> SendKeys<'a> {
        SendKeys::from(self)
    }

    pub fn send_prefix(&self) -> SendPrefix<'a> {
        SendPrefix::from(self)
    }

    pub fn unbind_key(&self) -> UnbindKey<'a> {
        UnbindKey::from(self)
    }
}
