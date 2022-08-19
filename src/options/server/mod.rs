#[cfg(feature = "tmux_3_2")]
pub mod extended_keys;
#[cfg(feature = "tmux_1_5")]
pub mod set_clipboard;

pub mod constants;
pub mod get_server_option;
pub mod get_server_options;
pub mod server_option;
pub mod server_options;
pub mod set_server_option;
pub mod set_server_options;

#[cfg(feature = "tmux_3_2")]
pub use extended_keys::ExtendedKeys;
#[cfg(feature = "tmux_1_5")]
pub use set_clipboard::SetClipboard;

pub use constants::*;
pub use get_server_option::*;
pub use server_option::*;
pub use server_options::*;
pub use set_server_option::*;

#[cfg(test)]
#[path = "."]
mod server_tests {
    #[cfg(feature = "tmux_3_2")]
    pub mod extended_keys_tests;
    #[cfg(feature = "tmux_1_2")]
    pub mod server_options_tests;
    #[cfg(feature = "tmux_1_5")]
    pub mod set_clipboard_tests;

    #[cfg(feature = "tmux_1_2")]
    pub mod server_option_tests;
}
