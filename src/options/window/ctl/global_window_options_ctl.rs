use crate::{
    Error, GetGlobalWindowOption, GetGlobalWindowOptionValue, SetGlobalWindowOption,
    SetGlobalWindowOptions, Tmux, TmuxCommand, TmuxOutput, WindowOptionsCtl,
};
use std::borrow::Cow;

// XXX: rename WindowOptionCtl?
// XXX: rename WindowOptionCtl?
// trait top level options, then server session window pane
pub struct GlobalWindowOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for GlobalWindowOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> GlobalWindowOptionsCtl<'a> {
    pub fn new(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self {
            invoker,
            ..Default::default()
        }
    }
}

impl<'a> WindowOptionsCtl<'a> for GlobalWindowOptionsCtl<'a> {
    type Getter = GetGlobalWindowOptionValue;
    type Setter = SetGlobalWindowOption;
    type GetterAll = GetGlobalWindowOption;
    type SetterMultiple = SetGlobalWindowOptions<'a>;

    fn target(&self) -> Option<Cow<'a, str>> {
        None
    }

    fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }
}
