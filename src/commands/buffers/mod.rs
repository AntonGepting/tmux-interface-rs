#[cfg(feature = "tmux_1_3")]
use crate::ChooseBuffer;
#[cfg(feature = "tmux_0_9")]
use crate::ClearHistory;
use crate::TmuxCommand;
#[cfg(feature = "tmux_0_8")]
use crate::{
    DeleteBuffer, ListBuffers, LoadBuffer, PasteBuffer, SaveBuffer, SetBuffer, ShowBuffer,
};

/// All functions from man tmux "Buffers" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS))
#[cfg(feature = "tmux_1_3")]
pub mod choose_buffer;
#[cfg(feature = "tmux_0_9")]
pub mod clear_history;
//#[cfg(feature = "tmux_1_0")]
//pub mod copy_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod delete_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod list_buffers;
#[cfg(feature = "tmux_0_8")]
pub mod load_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod paste_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod save_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod set_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod show_buffer;

#[cfg(feature = "tmux_1_3")]
pub mod choose_buffer_tests;
#[cfg(feature = "tmux_0_9")]
pub mod clear_history_tests;
// XXX: versions check
//#[cfg(feature = "tmux_1_0")]
//pub mod copy_buffer_tests;
#[cfg(feature = "tmux_0_8")]
pub mod delete_buffer_tests;
#[cfg(feature = "tmux_0_8")]
pub mod list_buffers_tests;
#[cfg(feature = "tmux_0_8")]
pub mod load_buffer_tests;
#[cfg(feature = "tmux_0_8")]
pub mod paste_buffer_tests;
#[cfg(feature = "tmux_0_8")]
pub mod save_buffer_tests;
#[cfg(feature = "tmux_0_8")]
pub mod set_buffer_tests;
#[cfg(feature = "tmux_0_8")]
pub mod show_buffer_tests;

/// All functions from man tmux "Buffers" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_1_3")]
    pub fn choose_buffer(&self) -> ChooseBuffer<'a> {
        ChooseBuffer::from(self)
    }

    #[cfg(feature = "tmux_0_9")]
    pub fn clear_history(&self) -> ClearHistory<'a> {
        ClearHistory::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn delete_buffer(&self) -> DeleteBuffer<'a> {
        DeleteBuffer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_buffers(&self) -> ListBuffers<'a> {
        ListBuffers::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn load_buffer(&self) -> LoadBuffer<'a> {
        LoadBuffer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn paste_buffer(&self) -> PasteBuffer<'a> {
        PasteBuffer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn save_buffer(&self) -> SaveBuffer<'a> {
        SaveBuffer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn set_buffer(&self) -> SetBuffer<'a> {
        SetBuffer::from(self)
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn show_buffer(&self) -> ShowBuffer<'a> {
        ShowBuffer::from(self)
    }
}
