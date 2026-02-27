// auto-generated file
//

/// All functions from man tmux "Buffers" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS))
///
use crate::TmuxCommand;

#[cfg(feature = "tmux_1_5")]
pub mod choose_buffer;
#[cfg(feature = "tmux_1_5")]
pub mod choose_buffer_macro;

#[cfg(feature = "tmux_1_5")]
pub mod clear_history;
#[cfg(feature = "tmux_1_5")]
pub mod clear_history_macro;

#[cfg(feature = "tmux_0_8")]
pub mod delete_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod delete_buffer_macro;

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
pub mod copy_buffer;
#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
pub mod copy_buffer_macro;

#[cfg(feature = "tmux_0_8")]
pub mod list_buffers;
#[cfg(feature = "tmux_0_8")]
pub mod list_buffers_macro;

#[cfg(feature = "tmux_0_8")]
pub mod load_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod load_buffer_macro;

#[cfg(feature = "tmux_0_8")]
pub mod paste_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod paste_buffer_macro;

#[cfg(feature = "tmux_0_8")]
pub mod save_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod save_buffer_macro;

#[cfg(feature = "tmux_0_8")]
pub mod set_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod set_buffer_macro;

#[cfg(feature = "tmux_0_8")]
pub mod show_buffer;
#[cfg(feature = "tmux_0_8")]
pub mod show_buffer_macro;

#[cfg(feature = "tmux_1_5")]
pub use choose_buffer::ChooseBuffer;

#[cfg(feature = "tmux_1_5")]
pub use clear_history::{ClearHist, ClearHistory};

#[cfg(feature = "tmux_0_8")]
pub use delete_buffer::{DeleteB, DeleteBuffer};

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
pub use copy_buffer::{CopyB, CopyBuffer};

#[cfg(feature = "tmux_0_8")]
pub use list_buffers::{ListBuffers, LsB};

#[cfg(feature = "tmux_0_8")]
pub use load_buffer::{LoadB, LoadBuffer};

#[cfg(feature = "tmux_0_8")]
pub use paste_buffer::{PasteB, PasteBuffer};

#[cfg(feature = "tmux_0_8")]
pub use save_buffer::{SaveB, SaveBuffer};

#[cfg(feature = "tmux_0_8")]
pub use set_buffer::{SetB, SetBuffer};

#[cfg(feature = "tmux_0_8")]
pub use show_buffer::{ShowB, ShowBuffer};

#[cfg(test)]
#[path = "."]
mod buffers_tests {

    #[cfg(feature = "tmux_1_5")]
    mod choose_buffer_tests;

    #[cfg(feature = "tmux_1_5")]
    mod clear_history_tests;

    #[cfg(feature = "tmux_0_8")]
    mod delete_buffer_tests;

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    mod copy_buffer_tests;

    #[cfg(feature = "tmux_0_8")]
    mod list_buffers_tests;

    #[cfg(feature = "tmux_0_8")]
    mod load_buffer_tests;

    #[cfg(feature = "tmux_0_8")]
    mod paste_buffer_tests;

    #[cfg(feature = "tmux_0_8")]
    mod save_buffer_tests;

    #[cfg(feature = "tmux_0_8")]
    mod set_buffer_tests;

    #[cfg(feature = "tmux_0_8")]
    mod show_buffer_tests;
}

/// All functions from man tmux "Buffers" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#BUFFERS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_1_5")]
    pub fn choose_buffer() -> ChooseBuffer<'a> {
        ChooseBuffer::new()
    }

    #[cfg(feature = "tmux_1_5")]
    pub fn clear_history() -> ClearHistory<'a> {
        ClearHistory::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn delete_buffer() -> DeleteBuffer<'a> {
        DeleteBuffer::new()
    }

    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn copy_buffer() -> CopyBuffer<'a> {
        CopyBuffer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn list_buffers() -> ListBuffers<'a> {
        ListBuffers::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn load_buffer() -> LoadBuffer<'a> {
        LoadBuffer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn paste_buffer() -> PasteBuffer<'a> {
        PasteBuffer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn save_buffer() -> SaveBuffer<'a> {
        SaveBuffer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn set_buffer() -> SetBuffer<'a> {
        SetBuffer::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn show_buffer() -> ShowBuffer<'a> {
        ShowBuffer::new()
    }
}

#[cfg(feature = "tmux_1_5")]
impl<'a> From<ChooseBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: ChooseBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_1_5")]
impl<'a> From<ClearHistory<'a>> for TmuxCommand<'a> {
    fn from(item: ClearHistory<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<DeleteBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: DeleteBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
impl<'a> From<CopyBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: CopyBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<ListBuffers<'a>> for TmuxCommand<'a> {
    fn from(item: ListBuffers<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<LoadBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: LoadBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<PasteBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: PasteBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<SaveBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: SaveBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<SetBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: SetBuffer<'a>) -> Self {
        item.build()
    }
}

#[cfg(feature = "tmux_0_8")]
impl<'a> From<ShowBuffer<'a>> for TmuxCommand<'a> {
    fn from(item: ShowBuffer<'a>) -> Self {
        item.build()
    }
}
