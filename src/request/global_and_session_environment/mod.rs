/// All functions from man tmux "Global and session environment" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#GLOBAL_AND_SESSION_ENVIRONMENT)
#[cfg(feature = "tmux_1_0")]
pub mod set_environment;
#[cfg(feature = "tmux_1_0")]
pub mod show_environment;

#[cfg(feature = "tmux_1_0")]
pub mod set_environment_tests;
#[cfg(feature = "tmux_1_0")]
pub mod show_environment_tests;
