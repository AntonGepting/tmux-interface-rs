pub mod session;
pub mod session_stack;
pub mod sessions;

#[cfg(test)]
#[path = "."]
mod variables_session_tests {
    mod session_stack_tests;
    mod session_tests;
    mod sessions_tests;
}
