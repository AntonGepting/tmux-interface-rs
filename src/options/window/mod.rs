pub mod builder;
pub mod common;
pub mod parser;

pub mod window_options;

pub use builder::*;
pub use common::*;
pub use parser::*;

pub use window_options::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod window_options_tests;
}
