pub mod buffer;
pub mod buffers;
pub mod buffers_ctl;

#[cfg(feature = "tmux_1_7")]
pub use buffer::Buffer;
#[cfg(feature = "tmux_1_7")]
pub use buffers::Buffers;
#[cfg(feature = "tmux_1_7")]
pub use buffers_ctl::BuffersCtl;

#[cfg(test)]
#[path = "."]
mod variables_buffers_tests {
    mod buffer_tests;
    mod buffers_ctl_tests;
    mod buffers_tests;
}
