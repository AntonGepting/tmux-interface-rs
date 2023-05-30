use crate::variables::pane::pane::PANE_VARS_SEPARATOR;
use crate::{Error, Formats, ListPanes, Panes, Tmux, TmuxCommand, TmuxOutput};
use std::borrow::Cow;
use std::str::FromStr;

// trait top level options, then server session window pane
pub struct PanesCtl<'a> {
    // TODO: comment/doc
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for PanesCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> PanesCtl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get<S: Into<Cow<'a, str>>>(&self, target_window: Option<S>) -> Result<Panes, Error> {
        Self::get_all_ext(target_window, self.invoker())
    }

    pub fn get_all(&self) -> Result<Panes, Error> {
        Self::get_all_ext(None::<&str>, self.invoker())
    }

    pub fn get_all_ext<S>(
        target: Option<S>,
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<Panes, Error>
    where
        S: Into<Cow<'a, str>>,
    {
        let mut format = Formats::new();
        format.separator(PANE_VARS_SEPARATOR);

        #[cfg(feature = "tmux_1_6")]
        format.pane_active();
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_bottom();
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_left();
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_right();
        #[cfg(feature = "tmux_2_6")]
        format.pane_at_top();
        #[cfg(feature = "tmux_2_0")]
        format.pane_bottom();
        #[cfg(feature = "tmux_1_8")]
        format.pane_current_command();
        #[cfg(feature = "tmux_1_7")]
        format.pane_current_path();
        #[cfg(feature = "tmux_1_6")]
        format.pane_dead();
        #[cfg(feature = "tmux_2_0")]
        format.pane_dead_status();
        #[cfg(feature = "tmux_2_6")]
        format.pane_format();
        #[cfg(feature = "tmux_1_6")]
        format.pane_height();
        #[cfg(feature = "tmux_1_6")]
        format.pane_id();
        #[cfg(feature = "tmux_1_8")]
        format.pane_in_mode();
        #[cfg(feature = "tmux_1_7")]
        format.pane_index();
        #[cfg(feature = "tmux_2_0")]
        format.pane_input_off();
        #[cfg(feature = "tmux_2_0")]
        format.pane_left();
        #[cfg(feature = "tmux_3_0")]
        format.pane_marked();
        #[cfg(feature = "tmux_3_0")]
        format.pane_marked_set();
        #[cfg(feature = "tmux_2_5")]
        format.pane_mode();
        #[cfg(feature = "tmux_3_1")]
        format.pane_path();
        #[cfg(feature = "tmux_1_6")]
        format.pane_pid();
        #[cfg(feature = "tmux_2_6")]
        format.pane_pipe();
        #[cfg(feature = "tmux_2_0")]
        format.pane_right();
        #[cfg(feature = "tmux_2_5")]
        format.pane_search_string();
        #[cfg(feature = "tmux_1_6")]
        format.pane_start_command();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_0")))]
        format.pane_start_path();
        #[cfg(feature = "tmux_1_9")]
        format.pane_synchronized();
        #[cfg(feature = "tmux_1_8")]
        format.pane_tabs();
        #[cfg(feature = "tmux_1_6")]
        format.pane_title();
        #[cfg(feature = "tmux_2_0")]
        format.pane_top();
        #[cfg(feature = "tmux_1_6")]
        format.pane_tty();
        #[cfg(feature = "tmux_1_6")]
        format.pane_width();

        let lsp_format = format.to_string();

        // all or belonging to target
        let cmd = ListPanes::new().format(lsp_format);
        let cmd = match target {
            Some(target) => cmd.target(target),
            None => cmd.all(),
        };
        let cmd = cmd.build();

        let output = (invoker)(cmd)?.to_string();
        Panes::from_str(&output)
    }
}
