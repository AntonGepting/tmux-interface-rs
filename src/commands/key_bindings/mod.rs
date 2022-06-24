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
    pub fn bind_key() -> BindKey<'a> {
        BindKey::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_keys() -> ListKeys<'a> {
        ListKeys::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn send_keys() -> SendKeys<'a> {
        SendKeys::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn send_prefix() -> SendPrefix<'a> {
        SendPrefix::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn unbind_key() -> UnbindKey<'a> {
        UnbindKey::new()
    }
}

impl<'a> From<BindKey<'a>> for TmuxCommand<'a> {
    fn from(item: BindKey<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListKeys<'a>> for TmuxCommand<'a> {
    fn from(item: ListKeys<'a>) -> Self {
        item.build()
    }
}
impl<'a> From<SendKeys<'a>> for TmuxCommand<'a> {
    fn from(item: SendKeys<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<UnbindKey<'a>> for TmuxCommand<'a> {
    fn from(item: UnbindKey<'a>) -> Self {
        item.build()
    }
}
