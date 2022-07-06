#[cfg(feature = "tmux_3_0")]
pub mod remain_on_exit;

pub mod pane_options;

#[cfg(feature = "tmux_3_0")]
pub use remain_on_exit::RemainOnExit;

pub use pane_options::*;

#[cfg(test)]
#[path = "."]
mod pane_tests {
    #[cfg(feature = "tmux_3_1")]
    pub mod pane_options_tests;
    #[cfg(feature = "tmux_3_0")]
    pub mod remain_on_exit_tests;
}
