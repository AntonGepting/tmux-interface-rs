pub mod get_pane_option;
pub mod get_pane_options;
pub mod set_pane_option;
pub mod set_pane_options;

pub use get_pane_option::*;
pub use get_pane_options::*;
pub use set_pane_option::*;
pub use set_pane_options::*;

#[cfg(test)]
#[path = "."]
mod pane_tests {
    #[cfg(feature = "tmux_3_0")]
    pub mod get_pane_option_tests;
    #[cfg(feature = "tmux_3_0")]
    pub mod get_pane_options_tests;

    #[cfg(feature = "tmux_3_0")]
    pub mod set_pane_option_tests;
}
