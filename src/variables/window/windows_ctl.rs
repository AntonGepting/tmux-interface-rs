use crate::variables::window::window::WINDOW_VARS_SEPARATOR;
use crate::{Error, Formats, ListWindows, Tmux, TmuxCommand, TmuxOutput, Windows};
use std::borrow::Cow;
use std::str::FromStr;

// trait top level options, then server session window pane
pub struct WindowsCtl<'a> {
    // TODO: comment/doc
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for WindowsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> WindowsCtl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get<S: Into<Cow<'a, str>>>(&self, target_session: Option<S>) -> Result<Windows, Error> {
        Self::get_all_ext(target_session, self.invoker())
    }

    pub fn get_ext<S: Into<Cow<'a, str>>>(
        target_session: Option<S>,
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<Windows, Error> {
        Self::get_all_ext(target_session, invoker)
    }

    pub fn get_all(&self) -> Result<Windows, Error> {
        Self::get_all_ext(None::<&str>, self.invoker())
    }

    pub fn get_all_ext<S: Into<Cow<'a, str>>>(
        target_session: Option<S>,
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<Windows, Error> {
        let mut format = Formats::new();
        format.separator(WINDOW_VARS_SEPARATOR);

        #[cfg(feature = "tmux_1_6")]
        format.window_active();
        #[cfg(feature = "tmux_3_1")]
        format.window_active_clients();
        #[cfg(feature = "tmux_3_1")]
        format.window_active_clients_list();
        #[cfg(feature = "tmux_3_1")]
        format.window_active_sessions();
        #[cfg(feature = "tmux_3_1")]
        format.window_active_sessions_list();
        #[cfg(feature = "tmux_2_1")]
        format.window_activity();
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_2")))]
        format.window_activity_string();
        #[cfg(any(
            all(feature = "tmux_1_9", not(feature = "tmux_2_2")),
            feature = "tmux_2_3"
        ))]
        format.window_activity_flag();
        #[cfg(feature = "tmux_1_9")]
        format.window_bell_flag();
        #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
        format.window_content_flag();
        #[cfg(feature = "tmux_2_9")]
        format.window_bigger();
        #[cfg(feature = "tmux_3_1")]
        format.window_cell_height();
        #[cfg(feature = "tmux_3_1")]
        format.window_cell_width();
        #[cfg(feature = "tmux_2_9")]
        format.window_end_flag();
        #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
        format.window_find_matches();
        #[cfg(feature = "tmux_1_6")]
        format.window_flags();
        #[cfg(feature = "tmux_2_6")]
        format.window_format();
        #[cfg(feature = "tmux_1_6")]
        format.window_height();
        #[cfg(feature = "tmux_1_7")]
        format.window_id();
        #[cfg(feature = "tmux_1_6")]
        format.window_index();
        #[cfg(feature = "tmux_2_0")]
        format.window_last_flag();
        #[cfg(feature = "tmux_1_6")]
        format.window_layout();
        #[cfg(feature = "tmux_2_1")]
        format.window_linked();
        #[cfg(feature = "tmux_3_1")]
        format.window_linked_sessions();
        #[cfg(feature = "tmux_3_1")]
        format.window_linked_sessions_list();
        #[cfg(feature = "tmux_3_1")]
        format.window_marked_flag();
        #[cfg(feature = "tmux_1_6")]
        format.window_name();
        #[cfg(feature = "tmux_2_9")]
        format.window_offset_x();
        #[cfg(feature = "tmux_2_9")]
        format.window_offset_y();
        #[cfg(feature = "tmux_1_7")]
        format.window_panes();
        #[cfg(feature = "tmux_1_9")]
        format.window_silence_flag();
        #[cfg(feature = "tmux_2_5")]
        format.window_stack_index();
        #[cfg(feature = "tmux_2_9")]
        format.window_start_flag();
        #[cfg(feature = "tmux_2_2")]
        format.window_visible_layout();
        #[cfg(feature = "tmux_1_6")]
        format.window_width();
        #[cfg(feature = "tmux_2_0")]
        format.window_zoomed_flag();

        let lsw_format = format.to_string();

        let cmd = ListWindows::new().format(lsw_format);
        let cmd = match target_session {
            Some(target_session) => cmd.target_session(target_session),
            None => cmd.all(),
        };
        let cmd = cmd.build();

        let output = (invoker)(cmd)?.to_string();
        Windows::from_str(&output)
    }
}
