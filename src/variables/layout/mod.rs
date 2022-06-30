pub mod layout;
pub mod layout_cell;
pub mod layout_checksum;

#[cfg(feature = "tmux_1_6")]
pub use layout::Layout;
#[cfg(feature = "tmux_1_6")]
pub use layout_cell::{LayoutCell, LayoutType};
#[cfg(feature = "tmux_1_6")]
pub use layout_checksum::LayoutChecksum;

#[cfg(test)]
#[path = "."]
mod variables_layout_tests {
    mod layout_cell_tests;
    mod layout_checksum_tests;
    mod layout_tests;
}
