use crate::{
    ChooseBuffer, ClearHistory, DeleteBuffer, ListBuffers, LoadBuffer, PasteBuffer, SaveBuffer,
    SetBuffer, ShowBuffer, TmuxCommand,
};

/// All functions from man tmux "Buffers" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS)
#[cfg(feature = "tmux_1_3")]
pub mod choose_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod clear_history;
//#[cfg(feature = "tmux_1_0")]
//pub mod copy_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod delete_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod list_buffers;
#[cfg(feature = "tmux_1_0")]
pub mod load_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod paste_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod save_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod set_buffer;
#[cfg(feature = "tmux_1_0")]
pub mod show_buffer;

//#[cfg(feature = "tmux_1_3")]
//pub mod choose_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod clear_history_tests;
////#[cfg(feature = "tmux_1_0")]
////pub mod copy_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod delete_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod list_buffers_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod load_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod paste_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod save_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_buffer_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod show_buffer_tests;

impl<'a> TmuxCommand<'a> {
    pub fn choose_buffer(&self) -> ChooseBuffer<'a> {
        ChooseBuffer::from(self)
    }

    pub fn clear_history(&self) -> ClearHistory<'a> {
        ClearHistory::from(self)
    }

    pub fn delete_buffer(&self) -> DeleteBuffer<'a> {
        DeleteBuffer::from(self)
    }

    pub fn list_buffers(&self) -> ListBuffers<'a> {
        ListBuffers::from(self)
    }

    pub fn load_buffer(&self) -> LoadBuffer<'a> {
        LoadBuffer::from(self)
    }

    pub fn paste_buffer(&self) -> PasteBuffer<'a> {
        PasteBuffer::from(self)
    }

    pub fn save_buffer(&self) -> SaveBuffer<'a> {
        SaveBuffer::from(self)
    }

    pub fn set_buffer(&self) -> SetBuffer<'a> {
        SetBuffer::from(self)
    }

    pub fn show_buffer(&self) -> ShowBuffer<'a> {
        ShowBuffer::from(self)
    }
}
