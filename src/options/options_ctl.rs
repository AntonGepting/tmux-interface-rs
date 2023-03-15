use crate::{
    Error, GlobalSessionOptionsCtl, GlobalWindowOptionsCtl, LocalSessionOptionsCtl,
    LocalWindowOptionsCtl, ServerOptions, ServerOptionsCtl, SessionOptions, SessionOptionsCtl,
    Tmux, TmuxCommand, TmuxOutput, WindowOptions, WindowOptionsCtl,
};

pub struct OptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for OptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: |cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> OptionsCtl<'a> {
    pub fn get_all(&self) -> Options<'a> {
        let mut options = Options::default();
        options.server_options = self.get_server_options();
        options
    }

    pub fn get_server_options(&self) -> ServerOptions<'a> {
        let server_options_ctl = ServerOptionsCtl::default();

        server_options_ctl.get_all().unwrap()
    }

    pub fn get_global_session_options(&self) -> SessionOptions<'a> {
        let session_options_ctl = GlobalSessionOptionsCtl::default();

        session_options_ctl.get_all().unwrap()
    }

    pub fn get_local_session_options(&self) -> SessionOptions<'a> {
        let session_options_ctl = LocalSessionOptionsCtl::default();

        session_options_ctl.get_all().unwrap()
    }

    pub fn get_global_window_global_options(&self) -> WindowOptions<'a> {
        let window_options_ctl = GlobalWindowOptionsCtl::default();
        window_options_ctl.get_all().unwrap()
    }

    pub fn get_local_window_global_options(&self) -> WindowOptions<'a> {
        let window_options_ctl = LocalWindowOptionsCtl::default();

        window_options_ctl.get_all().unwrap()
    }

    // pub fn get_pane_options() -> PaneOptions<'a> {}
}

// naming convention for moudules objects <Tmux>Options <=> <?>SessionOptions
// NOTE: separate call only, resulting command can't be merged in one, parsing differentation problems
#[derive(Default, Debug)]
pub struct Options<'a> {
    pub server_options: ServerOptions<'a>,
    pub global_session_options: SessionOptions<'a>,
    pub local_session_options: SessionOptions<'a>,
    pub global_window_options: WindowOptions<'a>,
    pub local_window_options: WindowOptions<'a>,
    // pub pane_options: PaneOptions<'a>,
}

pub struct GlobalOptions<'a> {
    pub server_options: ServerOptions<'a>,
    pub global_session_options: SessionOptions<'a>,
    pub global_window_options: WindowOptions<'a>,
    #[cfg(feature = "tmux_3_1")]
    pub pane_options: PaneOptions<'a>,
}
