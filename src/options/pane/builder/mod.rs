pub mod get_pane_option;
pub mod get_pane_options;

pub use get_pane_option::*;
pub use get_pane_options::*;

#[cfg(test)]
#[path = "."]
mod pane_tests {
    #[cfg(feature = "tmux_3_0")]
    pub mod get_pane_option_tests;
}
