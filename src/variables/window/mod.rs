pub mod window;
pub mod window_flag;
pub mod windows;
pub mod windows_ctl;

#[cfg(feature = "tmux_1_6")]
pub use window::Window;
#[cfg(feature = "tmux_1_6")]
pub use window_flag::WindowFlags;
#[cfg(feature = "tmux_1_6")]
pub use windows::Windows;
#[cfg(feature = "tmux_1_6")]
pub use windows_ctl::WindowsCtl;

#[cfg(test)]
#[path = "."]
mod variables_window_tests {
    mod window_tests;
    //pub mod window_flag_tests;
    mod windows_tests;
}
