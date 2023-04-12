pub mod pane;
pub mod pane_tabs;
pub mod panes;
pub mod panes_ctl;

#[cfg(feature = "tmux_1_6")]
pub use pane::Pane;
#[cfg(feature = "tmux_1_6")]
pub use pane_tabs::PaneTabs;
#[cfg(feature = "tmux_1_6")]
pub use panes::Panes;
#[cfg(feature = "tmux_1_6")]
pub use panes_ctl::PanesCtl;

#[cfg(test)]
#[path = "."]
mod variables_pane_tests {
    mod pane_tests;
    //pub mod pane_tabs_tests;
    mod panes_tests;
}
