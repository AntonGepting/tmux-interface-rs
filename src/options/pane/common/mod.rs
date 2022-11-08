pub mod constants;

#[cfg(feature = "tmux_3_0")]
pub mod remain_on_exit;

pub use constants::*;

#[cfg(feature = "tmux_3_0")]
pub use remain_on_exit::RemainOnExit;

#[cfg(test)]
#[path = "."]
mod pane_tests {
   #[cfg(feature = "tmux_3_0")]
    pub mod remain_on_exit_tests;
}
