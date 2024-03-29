//! Module for working with pane options
pub mod builder;
pub mod common;
pub mod ctl;

pub mod pane_options;

pub use builder::*;
pub use common::*;
pub use ctl::*;

pub use pane_options::*;

#[cfg(test)]
#[path = "."]
mod pane_tests {
    #[cfg(feature = "tmux_3_1")]
    pub mod pane_options_tests;
}
