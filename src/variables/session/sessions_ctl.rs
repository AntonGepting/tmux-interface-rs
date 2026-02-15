use crate::{Error, Formats, ListSessions, Sessions, Tmux, TmuxCommand, TmuxOutput};
use std::str::FromStr;

// trait top level options, then server session window pane
pub struct SessionsCtl<'a> {
    // TODO: comment/doc
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for SessionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> SessionsCtl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get_all(&self) -> Result<Sessions, Error> {
        Self::get_all_ext(self.invoker())
    }

    pub fn get_all_ext(
        invoker: impl FnOnce(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<Sessions, Error> {
        let mut format = Formats::new();
        format.separator(':');

        #[cfg(feature = "tmux_2_1")]
        format.session_activity();
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        format.session_activity_string();
        #[cfg(feature = "tmux_2_1")]
        format.session_alerts();
        #[cfg(feature = "tmux_1_6")]
        format.session_attached();
        #[cfg(feature = "tmux_3_1")]
        format.session_attached_list();
        #[cfg(feature = "tmux_1_6")]
        format.session_created();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        format.session_created_string();
        #[cfg(feature = "tmux_2_6")]
        format.session_format();
        #[cfg(feature = "tmux_1_6")]
        format.session_group();
        #[cfg(feature = "tmux_3_1")]
        format.session_group_attached();
        #[cfg(feature = "tmux_3_1")]
        format.session_group_attached_list();
        #[cfg(feature = "tmux_2_7")]
        format.session_group_list();
        #[cfg(feature = "tmux_3_1")]
        format.session_group_many_attached();
        #[cfg(feature = "tmux_2_7")]
        format.session_group_size();
        #[cfg(feature = "tmux_1_6")]
        format.session_grouped();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        format.session_height();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_9")))]
        format.session_width();
        #[cfg(feature = "tmux_1_8")]
        format.session_id();
        #[cfg(feature = "tmux_2_1")]
        format.session_last_attached();
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        format.session_last_attached_string();
        #[cfg(feature = "tmux_2_0")]
        format.session_many_attached();
        #[cfg(feature = "tmux_1_6")]
        format.session_name();
        #[cfg(feature = "tmux_2_5")]
        format.session_stack();
        #[cfg(feature = "tmux_1_6")]
        format.session_windows();

        let ls_format = format.to_string();

        let cmd = ListSessions::new().format(ls_format).build();
        let output = (invoker)(cmd)?.to_string();
        Sessions::from_str(&output)
    }
}
