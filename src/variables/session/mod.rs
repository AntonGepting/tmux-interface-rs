pub mod session;
pub mod session_stack;
pub mod sessions;

#[cfg(feature = "tmux_1_6")]
pub use session::Session;
#[cfg(feature = "tmux_1_6")]
pub use session_stack::SessionStack;
#[cfg(feature = "tmux_1_6")]
pub use sessions::Sessions;

#[cfg(test)]
#[path = "."]
mod variables_session_tests {
    mod session_stack_tests;
    mod session_tests;
    mod sessions_tests;
}
