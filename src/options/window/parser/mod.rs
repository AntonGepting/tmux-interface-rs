pub mod window_options_ctl;

pub use window_options_ctl::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod window_options_ctl_tests;
}
