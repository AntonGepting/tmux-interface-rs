pub mod get_server_option;
pub mod get_server_options;
pub mod set_server_option;
pub mod set_server_options;

pub use get_server_option::*;
pub use get_server_options::*;
pub use set_server_option::*;
pub use set_server_options::*;

#[cfg(test)]
#[path = "."]
mod server_tests {
    pub mod get_server_option_tests;
    pub mod get_server_options_tests;

    //pub mod set_server_option_tests;
    //pub mod set_server_options_tests;
}
