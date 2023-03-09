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
    pub fn get_all() {}

    pub fn get_server_options() -> ServerOptions<'a> {}

    pub fn get_session_global_options() -> SessionOptions<'a> {}
    pub fn get_session_local_options() -> SessionOptions<'a> {}

    pub fn get_window_global_options() -> WindowOptions<'a> {}
    pub fn get_window_local_options() -> WindowOptions<'a> {}

    pub fn get_pane_options() -> PaneOptions<'a> {}
}

pub struct Options<'a> {
    pub server_options: ServerOptions<'a>,
    pub session_global_options: SessionOptions<'a>,
    pub session_local_options: SessionOptions<'a>,
    pub window_global_options: WindowOptions<'a>,
    pub window_local_options: WindowOptions<'a>,
    pub pane_options: PaneOptions<'a>,
}

impl<'a> Options<'a> {}

//pub struct WindowOptions<'a> {
// pub global: WindowOptions<'a>
// pub local: WindowOptions<'a>
//}
