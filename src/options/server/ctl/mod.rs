pub mod server_options_ctl;

pub use server_options_ctl::*;

#[cfg(test)]
#[path = "."]
mod server_options_ctl_tests {
    #[cfg(feature = "tmux_1_2")]
    pub mod server_options_ctl_tests;
}
