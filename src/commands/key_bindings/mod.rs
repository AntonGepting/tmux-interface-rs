use crate::TmuxCommand;

/// All functions from man tmux "Key Bindings" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS))
#[cfg(feature = "tmux_0_8")]
pub mod bind_key;
#[cfg(feature = "tmux_0_8")]
pub mod bind_key_macro;

#[cfg(feature = "tmux_0_8")]
pub mod list_keys;
#[cfg(feature = "tmux_0_8")]
pub mod list_keys_macro;

#[cfg(feature = "tmux_0_8")]
pub mod send_keys;
#[cfg(feature = "tmux_0_8")]
pub mod send_keys_macro;

#[cfg(feature = "tmux_0_8")]
pub mod send_prefix;
#[cfg(feature = "tmux_0_8")]
pub mod send_prefix_macro;

#[cfg(feature = "tmux_0_8")]
pub mod unbind_key;
#[cfg(feature = "tmux_0_8")]
pub mod unbind_key_macro;

#[cfg(feature = "tmux_0_8")]
pub use bind_key::BindKey;
#[cfg(feature = "tmux_0_8")]
pub use list_keys::ListKeys;
#[cfg(feature = "tmux_0_8")]
pub use send_keys::SendKeys;
#[cfg(feature = "tmux_0_8")]
pub use send_prefix::SendPrefix;
#[cfg(feature = "tmux_0_8")]
pub use unbind_key::UnbindKey;

#[cfg(test)]
#[path = "."]
mod key_bindings_tests {
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
}

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

#[cfg(feature = "tmux_0_8")]
impl<'a> From<BindKey<'a>> for TmuxCommand<'a> {
    fn from(item: BindKey<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<ListKeys<'a>> for TmuxCommand<'a> {
    fn from(item: ListKeys<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<SendKeys<'a>> for TmuxCommand<'a> {
    fn from(item: SendKeys<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<SendPrefix<'a>> for TmuxCommand<'a> {
    fn from(item: SendPrefix<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<UnbindKey<'a>> for TmuxCommand<'a> {
    fn from(item: UnbindKey<'a>) -> Self {
        item.build()
    }
}
