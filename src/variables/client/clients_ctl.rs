use crate::{Clients, Error, Formats, ListClients, Tmux, TmuxCommand, TmuxOutput};
use std::str::FromStr;

// trait top level options, then server client window pane
pub struct ClientsCtl<'a> {
    // TODO: comment/doc
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for ClientsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> ClientsCtl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }

    pub fn get_all(&self) -> Result<Clients, Error> {
        Self::get_all_ext(self.invoker())
    }

    pub fn get_all_ext(
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Result<Clients, Error> {
        let mut format = Formats::new();
        format.separator(':');

        #[cfg(feature = "tmux_1_6")]
        format.client_activity();
        #[cfg(feature = "tmux_3_1")]
        format.client_cell_height();
        #[cfg(feature = "tmux_3_1")]
        format.client_cell_width();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        format.client_activity_string();
        #[cfg(feature = "tmux_1_6")]
        format.client_created();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_2")))]
        format.client_created_string();
        #[cfg(feature = "tmux_2_1")]
        format.client_control_mode();
        #[cfg(feature = "tmux_2_1")]
        format.client_discarded();
        #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
        format.client_cwd();
        #[cfg(feature = "tmux_1_6")]
        format.client_height();
        #[cfg(feature = "tmux_2_2")]
        format.client_key_table();
        #[cfg(feature = "tmux_1_8")]
        format.client_last_session();
        #[cfg(feature = "tmux_2_4")]
        format.client_name();
        #[cfg(feature = "tmux_2_1")]
        format.client_pid();
        #[cfg(feature = "tmux_1_8")]
        format.client_prefix();
        #[cfg(feature = "tmux_1_6")]
        format.client_readonly();
        #[cfg(feature = "tmux_1_8")]
        format.client_session();
        #[cfg(feature = "tmux_1_6")]
        format.client_termname();
        #[cfg(all(feature = "tmux_2_4", not(feature = "tmux_3_1")))]
        format.client_termtype();
        #[cfg(feature = "tmux_1_6")]
        format.client_tty();
        #[cfg(feature = "tmux_1_6")]
        format.client_utf8();
        #[cfg(feature = "tmux_1_6")]
        format.client_width();
        #[cfg(feature = "tmux_2_4")]
        format.client_written();

        let ls_format = format.to_string();

        let cmd = ListClients::new().format(ls_format).build();
        let output = (invoker)(cmd)?.to_string();
        Clients::from_str(&output)
    }
}
