pub mod constants;
#[cfg(feature = "tmux_3_2")]
pub mod extended_keys;
#[cfg(feature = "tmux_1_5")]
pub mod set_clipboard;

pub use constants::*;
#[cfg(feature = "tmux_3_2")]
pub use extended_keys::ExtendedKeys;
#[cfg(feature = "tmux_1_5")]
pub use set_clipboard::SetClipboard;

#[cfg(test)]
#[path = "."]
mod common_tests {
    #[cfg(feature = "tmux_3_2")]
    pub mod extended_keys_tests;
    #[cfg(feature = "tmux_1_5")]
    pub mod set_clipboard_tests;
}
