/// All functions from man tmux "Key Bindings" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS)
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

#[cfg(feature = "tmux_1_0")]
pub mod bind_key_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_keys_tests;
#[cfg(feature = "tmux_1_0")]
pub mod send_keys_tests;
#[cfg(feature = "tmux_1_0")]
pub mod send_prefix_tests;
#[cfg(feature = "tmux_1_0")]
pub mod unbind_key_tests;
