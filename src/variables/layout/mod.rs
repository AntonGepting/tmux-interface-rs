pub mod layout;
pub mod layout_cell;
pub mod layout_checksum;

#[cfg(test)]
#[path = "."]
mod variables_layout_tests {
    mod layout_cell_tests;
    mod layout_checksum_tests;
    mod layout_tests;
}
