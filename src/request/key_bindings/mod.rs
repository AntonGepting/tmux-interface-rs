/// All functions from man tmux "Key Bindings" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#KEY_BINDINGS)
pub mod bind_key;
pub mod list_keys;
pub mod send_keys;
pub mod send_prefix;
pub mod unbind_key;

pub mod bind_key_tests;
pub mod list_keys_tests;
pub mod send_keys_tests;
pub mod send_prefix_tests;
pub mod unbind_key_tests;
