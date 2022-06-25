#[cfg(feature = "tmux_1_4")]
use crate::LastPane;
#[cfg(feature = "tmux_1_1")]
use crate::PipePane;
#[cfg(feature = "tmux_1_3")]
use crate::PreviousLayout;
#[cfg(feature = "tmux_2_9")]
use crate::ResizeWindow;
#[cfg(feature = "tmux_1_5")]
use crate::RespawnPane;
use crate::TmuxCommand;
#[cfg(feature = "tmux_0_8")]
use crate::{
    BreakPane, CopyMode, FindWindow, KillPane, KillWindow, LastWindow, LinkWindow, ListPanes,
    ListWindows, MoveWindow, NewWindow, NextLayout, PreviousWindow, RenameWindow, RespawnWindow,
    RotateWindow, SelectPane, SelectWindow, SplitWindow, SwapPane, SwapWindow, UnlinkWindow,
};
#[cfg(feature = "tmux_1_2")]
use crate::{CapturePane, JoinPane};
#[cfg(feature = "tmux_1_0")]
use crate::{ChooseClient, DisplayPanes};
#[cfg(feature = "tmux_1_7")]
use crate::{ChooseTree, MovePane};
#[cfg(feature = "tmux_0_9")]
use crate::{ResizePane, SelectLayout};

#[cfg(feature = "tmux_0_8")]
use crate::NextWindow;

/// All functions from man tmux "Windows and Panes" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES))
#[cfg(feature = "tmux_0_8")]
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
#[cfg(feature = "tmux_0_8")]
pub mod copy_mode;
#[cfg(feature = "tmux_1_0")]
pub mod display_panes;
//#[cfg(feature = "tmux_1_0")]
//pub mod down_pane;
#[cfg(feature = "tmux_0_8")]
pub mod find_window;
#[cfg(feature = "tmux_1_2")]
pub mod join_pane;
#[cfg(feature = "tmux_0_8")]
pub mod kill_pane;
#[cfg(feature = "tmux_0_8")]
pub mod kill_window;
#[cfg(feature = "tmux_1_4")]
pub mod last_pane;
#[cfg(feature = "tmux_0_8")]
pub mod last_window;
#[cfg(feature = "tmux_0_8")]
pub mod link_window;
#[cfg(feature = "tmux_0_8")]
pub mod list_panes;
#[cfg(feature = "tmux_0_8")]
pub mod list_windows;
#[cfg(feature = "tmux_1_7")]
pub mod move_pane;
#[cfg(feature = "tmux_0_8")]
pub mod move_window;
#[cfg(feature = "tmux_0_8")]
pub mod new_window;
#[cfg(feature = "tmux_0_8")]
pub mod next_layout;
#[cfg(feature = "tmux_0_8")]
pub mod next_window;
#[cfg(feature = "tmux_1_1")]
pub mod pipe_pane;
#[cfg(feature = "tmux_1_3")]
pub mod previous_layout;
#[cfg(feature = "tmux_0_8")]
pub mod previous_window;
#[cfg(feature = "tmux_0_8")]
pub mod rename_window;
#[cfg(feature = "tmux_0_9")]
pub mod resize_pane;
#[cfg(feature = "tmux_2_9")]
pub mod resize_window;
#[cfg(feature = "tmux_1_5")]
pub mod respawn_pane;
#[cfg(feature = "tmux_0_8")]
pub mod respawn_window;
#[cfg(feature = "tmux_0_8")]
pub mod rotate_window;
#[cfg(feature = "tmux_0_9")]
pub mod select_layout;
#[cfg(feature = "tmux_0_8")]
pub mod select_pane;
#[cfg(feature = "tmux_0_8")]
pub mod select_window;
#[cfg(feature = "tmux_0_8")]
pub mod split_window;
#[cfg(feature = "tmux_0_8")]
pub mod swap_pane;
#[cfg(feature = "tmux_0_8")]
pub mod swap_window;
#[cfg(feature = "tmux_0_8")]
pub mod unlink_window;
//#[cfg(feature = "tmux_1_0")]
//pub mod up_pane;

#[cfg(test)]
#[path = "."]
mod windows_and_panes_tests {
    #[cfg(feature = "tmux_0_8")]
    pub mod break_pane_tests;
    #[cfg(feature = "tmux_1_2")]
    pub mod capture_pane_tests;
    #[cfg(feature = "tmux_1_0")]
    pub mod choose_client_tests;
    #[cfg(feature = "tmux_1_7")]
    pub mod choose_tree_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod copy_mode_tests;
    #[cfg(feature = "tmux_1_0")]
    pub mod display_panes_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod find_window_tests;
    #[cfg(feature = "tmux_1_2")]
    pub mod join_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod kill_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod kill_window_tests;
    #[cfg(feature = "tmux_1_4")]
    pub mod last_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod last_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod link_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod list_panes_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod list_windows_tests;
    #[cfg(feature = "tmux_1_7")]
    pub mod move_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod move_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod new_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod next_layout_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod next_window_tests;
    #[cfg(feature = "tmux_1_1")]
    pub mod pipe_pane_tests;
    #[cfg(feature = "tmux_1_3")]
    pub mod previous_layout_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod previous_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod rename_window_tests;
    #[cfg(feature = "tmux_0_9")]
    pub mod resize_pane_tests;
    #[cfg(feature = "tmux_2_9")]
    pub mod resize_window_tests;
    #[cfg(feature = "tmux_1_5")]
    pub mod respawn_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod respawn_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod rotate_window_tests;
    #[cfg(feature = "tmux_0_9")]
    pub mod select_layout_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod select_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod select_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod split_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod swap_pane_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod swap_window_tests;
    #[cfg(feature = "tmux_0_8")]
    pub mod unlink_window_tests;
}

/// All functions from man tmux "Windows and Panes" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#WINDOWS_AND_PANES))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn break_pane() -> BreakPane<'a> {
        BreakPane::new()
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn capture_pane() -> CapturePane<'a> {
        CapturePane::new()
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn choose_client() -> ChooseClient<'a> {
        ChooseClient::new()
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn choose_tree() -> ChooseTree<'a> {
        ChooseTree::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn copy_mode() -> CopyMode<'a> {
        CopyMode::new()
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn display_panes() -> DisplayPanes<'a> {
        DisplayPanes::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn find_window() -> FindWindow<'a> {
        FindWindow::new()
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn join_pane() -> JoinPane<'a> {
        JoinPane::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill_pane() -> KillPane<'a> {
        KillPane::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill_window() -> KillWindow<'a> {
        KillWindow::new()
    }

    #[cfg(feature = "tmux_1_4")]
    pub fn last_pane() -> LastPane<'a> {
        LastPane::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn last_window() -> LastWindow<'a> {
        LastWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn link_window() -> LinkWindow<'a> {
        LinkWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_panes() -> ListPanes<'a> {
        ListPanes::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_windows() -> ListWindows<'a> {
        ListWindows::new()
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn move_pane() -> MovePane<'a> {
        MovePane::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn move_window() -> MoveWindow<'a> {
        MoveWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn new_window() -> NewWindow<'a> {
        NewWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn next_layout() -> NextLayout<'a> {
        NextLayout::new()
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn pipe_pane() -> PipePane<'a> {
        PipePane::new()
    }

    #[cfg(feature = "tmux_1_3")]
    pub fn previous_layout() -> PreviousLayout<'a> {
        PreviousLayout::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn previous_window() -> PreviousWindow<'a> {
        PreviousWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn rename_window() -> RenameWindow<'a> {
        RenameWindow::new()
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn resize_pane() -> ResizePane<'a> {
        ResizePane::new()
    }

    #[cfg(feature = "tmux_2_9")]
    pub fn resize_window() -> ResizeWindow<'a> {
        ResizeWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn respawn_window() -> RespawnWindow<'a> {
        RespawnWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn rotate_window() -> RotateWindow<'a> {
        RotateWindow::new()
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn select_layout() -> SelectLayout<'a> {
        SelectLayout::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn select_pane() -> SelectPane<'a> {
        SelectPane::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn select_window() -> SelectWindow<'a> {
        SelectWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn split_window() -> SplitWindow<'a> {
        SplitWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn swap_pane() -> SwapPane<'a> {
        SwapPane::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn swap_window() -> SwapWindow<'a> {
        SwapWindow::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn unlink_window() -> UnlinkWindow<'a> {
        UnlinkWindow::new()
    }
}

impl<'a> From<BreakPane<'a>> for TmuxCommand<'a> {
    fn from(item: BreakPane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<CapturePane<'a>> for TmuxCommand<'a> {
    fn from(item: CapturePane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ChooseClient<'a>> for TmuxCommand<'a> {
    fn from(item: ChooseClient<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ChooseTree<'a>> for TmuxCommand<'a> {
    fn from(item: ChooseTree<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<CopyMode<'a>> for TmuxCommand<'a> {
    fn from(item: CopyMode<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<DisplayPanes<'a>> for TmuxCommand<'a> {
    fn from(item: DisplayPanes<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<FindWindow<'a>> for TmuxCommand<'a> {
    fn from(item: FindWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<JoinPane<'a>> for TmuxCommand<'a> {
    fn from(item: JoinPane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<KillPane<'a>> for TmuxCommand<'a> {
    fn from(item: KillPane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<KillWindow<'a>> for TmuxCommand<'a> {
    fn from(item: KillWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<LastPane<'a>> for TmuxCommand<'a> {
    fn from(item: LastPane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<LastWindow<'a>> for TmuxCommand<'a> {
    fn from(item: LastWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<LinkWindow<'a>> for TmuxCommand<'a> {
    fn from(item: LinkWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListPanes<'a>> for TmuxCommand<'a> {
    fn from(item: ListPanes<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ListWindows<'a>> for TmuxCommand<'a> {
    fn from(item: ListWindows<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<MovePane<'a>> for TmuxCommand<'a> {
    fn from(item: MovePane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<MoveWindow<'a>> for TmuxCommand<'a> {
    fn from(item: MoveWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<NewWindow<'a>> for TmuxCommand<'a> {
    fn from(item: NewWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<NextLayout<'a>> for TmuxCommand<'a> {
    fn from(item: NextLayout<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<NextWindow<'a>> for TmuxCommand<'a> {
    fn from(item: NextWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<PipePane<'a>> for TmuxCommand<'a> {
    fn from(item: PipePane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<PreviousLayout<'a>> for TmuxCommand<'a> {
    fn from(item: PreviousLayout<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<PreviousWindow<'a>> for TmuxCommand<'a> {
    fn from(item: PreviousWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<RenameWindow<'a>> for TmuxCommand<'a> {
    fn from(item: RenameWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<ResizePane<'a>> for TmuxCommand<'a> {
    fn from(item: ResizePane<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_2_9")]
impl<'a> From<ResizeWindow<'a>> for TmuxCommand<'a> {
    fn from(item: ResizeWindow<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_1_5")]
impl<'a> From<RespawnPane<'a>> for TmuxCommand<'a> {
    fn from(item: RespawnPane<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<RespawnWindow<'a>> for TmuxCommand<'a> {
    fn from(item: RespawnWindow<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<RotateWindow<'a>> for TmuxCommand<'a> {
    fn from(item: RotateWindow<'a>) -> Self {
        item.build()
    }
}
