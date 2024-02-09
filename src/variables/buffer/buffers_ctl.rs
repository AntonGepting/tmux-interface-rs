use crate::{Buffers, Error, Formats, ListBuffers, Tmux, TmuxCommand, TmuxOutput};
use std::str::FromStr;

// trait top level options, then server buffer window pane
pub struct BuffersCtl<'a> {
    // TODO: comment/doc
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for BuffersCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> BuffersCtl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get_all(&self) -> Result<Buffers, Error> {
        Self::get_all_ext(self.invoker())
    }

    pub fn get_all_ext(
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<Buffers, Error> {
        let mut format = Formats::new();
        format.separator(':');

        #[cfg(feature = "tmux_2_6")]
        format.buffer_created();
        #[cfg(feature = "tmux_2_3")]
        format.buffer_name();
        #[cfg(feature = "tmux_1_7")]
        format.buffer_sample();
        #[cfg(feature = "tmux_1_7")]
        format.buffer_size();

        let ls_format = format.to_string();

        let cmd = ListBuffers::new().format(ls_format).build();
        let output = (invoker)(cmd)?.to_string();
        Buffers::from_str(&output)
    }
}
