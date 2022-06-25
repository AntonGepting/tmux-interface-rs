pub mod pane;
pub mod pane_tabs;
pub mod panes;

#[cfg(test)]
#[path = "."]
mod variables_pane_tests {
    mod pane_tests;
    //pub mod pane_tabs_tests;
    mod panes_tests;
}
