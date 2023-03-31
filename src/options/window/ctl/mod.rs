pub mod global_window_options_ctl;
pub mod local_window_options_ctl;
pub mod window_options_ctl;

pub use global_window_options_ctl::*;
pub use local_window_options_ctl::*;
pub use window_options_ctl::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod window_options_ctl_tests;

    #[cfg(feature = "tmux_1_2")]
    pub mod global_window_options_ctl_tests;

    #[cfg(feature = "tmux_1_2")]
    pub mod local_window_options_ctl_tests;
}
