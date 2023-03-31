//! Module for working with window options
pub mod builder;
pub mod common;
pub mod ctl;

pub mod window_options;

pub use builder::*;
pub use common::*;
pub use ctl::*;

pub use window_options::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod window_options_tests;
}
