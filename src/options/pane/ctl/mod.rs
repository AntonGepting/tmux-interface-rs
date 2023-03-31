pub mod pane_options_ctl;

pub use pane_options_ctl::*;

#[cfg(test)]
#[path = "."]
mod pane_tests {
    #[cfg(feature = "tmux_3_1")]
    pub mod pane_options_ctl_tests;
}
