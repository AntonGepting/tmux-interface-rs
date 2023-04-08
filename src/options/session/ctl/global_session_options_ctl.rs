use crate::options::{
    GetGlobalSessionOption, GetGlobalSessionOptionValue, SessionOptionsCtl, SetGlobalSessionOption,
    SetGlobalSessionOptions,
};
use crate::{Error, Tmux, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// XXX: rename SessionOptionCtl?
// trait top level options, then server session window pane
pub struct GlobalSessionOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
}

impl<'a> Default for GlobalSessionOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
        }
    }
}

impl<'a> GlobalSessionOptionsCtl<'a> {
    pub fn new(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self { invoker }
    }
}

impl<'a> SessionOptionsCtl<'a> for GlobalSessionOptionsCtl<'a> {
    type Getter = GetGlobalSessionOptionValue;
    type Setter = SetGlobalSessionOption;
    type GetterAll = GetGlobalSessionOption;
    type SetterMultiple = SetGlobalSessionOptions<'a>;

    fn target(&self) -> Option<Cow<'a, str>> {
        None
    }

    fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }
}
