pub mod get_window_option;
pub mod set_window_option_ext;

pub use get_window_option::*;
pub use set_window_option_ext::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
}
