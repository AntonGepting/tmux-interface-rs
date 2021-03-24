use crate::{
    BreakPane, CapturePane, ChooseClient, ChooseTree, CopyMode, DisplayPanes, FindWindow, JoinPane,
    KillPane, KillWindow, LastPane, LastWindow, LinkWindow, ListPanes, ListWindows, TmuxCommand,
};

/// All functions from man tmux "Windows and Panes" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES))
#[cfg(feature = "tmux_1_0")]
pub mod break_pane;
#[cfg(feature = "tmux_1_2")]
pub mod capture_pane;
#[cfg(feature = "tmux_1_0")]
pub mod choose_client;
//#[cfg(feature = "tmux_1_0")]
//pub mod choose_session;
#[cfg(feature = "tmux_1_7")]
pub mod choose_tree;
//#[cfg(feature = "tmux_1_0")]
//pub mod choose_window;
#[cfg(feature = "tmux_1_0")]
pub mod copy_mode;
#[cfg(feature = "tmux_1_0")]
pub mod display_panes;
//#[cfg(feature = "tmux_1_0")]
//pub mod down_pane;
#[cfg(feature = "tmux_1_0")]
pub mod find_window;
#[cfg(feature = "tmux_1_2")]
pub mod join_pane;
#[cfg(feature = "tmux_1_0")]
pub mod kill_pane;
#[cfg(feature = "tmux_1_0")]
pub mod kill_window;
#[cfg(feature = "tmux_1_4")]
pub mod last_pane;
#[cfg(feature = "tmux_1_0")]
pub mod last_window;
#[cfg(feature = "tmux_1_0")]
pub mod link_window;
#[cfg(feature = "tmux_1_1")]
pub mod list_panes;
#[cfg(feature = "tmux_1_0")]
pub mod list_windows;
#[cfg(feature = "tmux_1_7")]
pub mod move_pane;
#[cfg(feature = "tmux_1_0")]
pub mod move_window;
#[cfg(feature = "tmux_1_0")]
pub mod new_window;
#[cfg(feature = "tmux_1_0")]
pub mod next_layout;
#[cfg(feature = "tmux_1_0")]
pub mod next_window;
#[cfg(feature = "tmux_1_1")]
pub mod pipe_pane;
#[cfg(feature = "tmux_1_3")]
pub mod previous_layout;
#[cfg(feature = "tmux_1_0")]
pub mod previous_window;
#[cfg(feature = "tmux_1_0")]
pub mod rename_window;
#[cfg(feature = "tmux_1_0")]
pub mod resize_pane;
#[cfg(feature = "tmux_2_9")]
pub mod resize_window;
#[cfg(feature = "tmux_1_5")]
pub mod respawn_pane;
#[cfg(feature = "tmux_1_0")]
pub mod respawn_window;
#[cfg(feature = "tmux_1_0")]
pub mod rotate_window;
#[cfg(feature = "tmux_1_0")]
pub mod select_layout;
#[cfg(feature = "tmux_1_0")]
pub mod select_pane;
#[cfg(feature = "tmux_1_0")]
pub mod select_window;
#[cfg(feature = "tmux_1_0")]
pub mod split_window;
#[cfg(feature = "tmux_1_0")]
pub mod swap_pane;
#[cfg(feature = "tmux_1_0")]
pub mod swap_window;
#[cfg(feature = "tmux_1_0")]
pub mod unlink_window;
//#[cfg(feature = "tmux_1_0")]
//pub mod up_pane;

//#[cfg(feature = "tmux_1_0")]
//pub mod break_pane_tests;
//#[cfg(feature = "tmux_1_2")]
//pub mod capture_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod choose_client_tests;
//#[cfg(feature = "tmux_1_7")]
//pub mod choose_tree_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod copy_mode_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod display_panes_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod find_window_tests;
//#[cfg(feature = "tmux_1_2")]
//pub mod join_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod kill_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod kill_window_tests;
//#[cfg(feature = "tmux_1_4")]
//pub mod last_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod last_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod link_window_tests;
//#[cfg(feature = "tmux_1_1")]
//pub mod list_panes_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod list_windows_tests;
//#[cfg(feature = "tmux_1_7")]
//pub mod move_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod move_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod new_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod next_layout_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod next_window_tests;
//#[cfg(feature = "tmux_1_1")]
//pub mod pipe_pane_tests;
//#[cfg(feature = "tmux_1_3")]
//pub mod previous_layout_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod previous_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod rename_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod resize_pane_tests;
//#[cfg(feature = "tmux_2_9")]
//pub mod resize_window_tests;
//#[cfg(feature = "tmux_1_5")]
//pub mod respawn_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod respawn_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod rotate_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod select_layout_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod select_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod select_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod split_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod swap_pane_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod swap_window_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod unlink_window_tests;

/// All functions from man tmux "Windows and Panes" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES))
impl<'a> TmuxCommand<'a> {
    pub fn break_pane(&self) -> BreakPane<'a> {
        BreakPane::from(self)
    }

    pub fn capture_pane(&self) -> CapturePane<'a> {
        CapturePane::from(self)
    }

    pub fn choose_client(&self) -> ChooseClient<'a> {
        ChooseClient::from(self)
    }

    pub fn choose_tree(&self) -> ChooseTree<'a> {
        ChooseTree::from(self)
    }

    pub fn copy_mode(&self) -> CopyMode<'a> {
        CopyMode::from(self)
    }

    pub fn display_panes(&self) -> DisplayPanes<'a> {
        DisplayPanes::from(self)
    }

    pub fn find_window(&self) -> FindWindow<'a> {
        FindWindow::from(self)
    }

    pub fn join_pane(&self) -> JoinPane<'a> {
        JoinPane::from(self)
    }

    pub fn kill_pane(&self) -> KillPane<'a> {
        KillPane::from(self)
    }

    pub fn kill_window(&self) -> KillWindow<'a> {
        KillWindow::from(self)
    }

    pub fn last_pane(&self) -> LastPane<'a> {
        LastPane::from(self)
    }

    pub fn last_window(&self) -> LastWindow<'a> {
        LastWindow::from(self)
    }

    pub fn link_window(&self) -> LinkWindow<'a> {
        LinkWindow::from(self)
    }

    pub fn list_panes(&self) -> ListPanes<'a> {
        ListPanes::from(self)
    }

    pub fn list_windows(&self) -> ListWindows<'a> {
        ListWindows::from(self)
    }
}
