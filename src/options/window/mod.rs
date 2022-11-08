pub mod common;
pub mod builder;

pub mod window_options;


pub use common::*;
pub use builder::*;

pub use window_options::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod window_options_tests;
}
