pub mod client;
pub mod clients;
pub mod clients_ctl;

#[cfg(feature = "tmux_1_6")]
pub use client::Client;
#[cfg(feature = "tmux_1_6")]
pub use clients::Clients;
#[cfg(feature = "tmux_1_6")]
pub use clients_ctl::ClientsCtl;

#[cfg(test)]
#[path = "."]
mod variables_clients_tests {
    mod client_tests;
    mod clients_ctl_tests;
    mod clients_tests;
}
