/// All functions from man tmux "Hooks" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#HOOKS)
#[cfg(feature = "tmux_2_2")]
pub mod set_hook;
#[cfg(feature = "tmux_2_2")]
pub mod show_hooks;

#[cfg(feature = "tmux_2_2")]
pub mod set_hook_tests;
#[cfg(feature = "tmux_2_2")]
pub mod show_hooks_tests;