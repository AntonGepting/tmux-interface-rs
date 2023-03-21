pub mod session_options_ctl;

pub mod global_session_options_ctl;
pub mod local_session_options_ctl;

pub use global_session_options_ctl::GlobalSessionOptionsCtl;
pub use local_session_options_ctl::LocalSessionOptionsCtl;

pub use session_options_ctl::SessionOptionsCtl;

#[cfg(test)]
#[path = "."]
mod session_tests {
    #[cfg(feature = "tmux_1_0")]
    pub mod session_options_ctl_tests;

    #[cfg(feature = "tmux_1_0")]
    pub mod global_session_options_ctl_tests;
    #[cfg(feature = "tmux_1_0")]
    pub mod local_session_options_ctl_tests;
}
