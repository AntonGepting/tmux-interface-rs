use crate::{
    Error, GetLocalWindowOption, GetLocalWindowOptionValue, SetLocalWindowOption,
    SetLocalWindowOptions, Tmux, TmuxCommand, TmuxOutput, WindowOptionsCtl,
};
use std::borrow::Cow;

// XXX: rename WindowOptionCtl?
// trait top level options, then server session window pane
pub struct LocalWindowOptionsCtl<'a> {
    // TODO: comment/doc
    //
    // function used for executing the given option get/set command
    //
    // ```
    // let tmux = Tmux::new();
    // ```
    pub invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    pub target: Option<Cow<'a, str>>,
}

impl<'a> Default for LocalWindowOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
            target: None,
        }
    }
}

impl<'a> LocalWindowOptionsCtl<'a> {
    pub fn new(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self {
            invoker,
            target: None,
        }
    }

    pub fn with_target<S: Into<Cow<'a, str>>>(target: Option<S>) -> Self {
        Self {
            target: target.map(|s| s.into()),
            ..Default::default()
        }
    }

    pub fn with_invoker(invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>) -> Self {
        Self {
            invoker,
            ..Default::default()
        }
    }
}

impl<'a> WindowOptionsCtl<'a> for LocalWindowOptionsCtl<'a> {
    type Getter = GetLocalWindowOptionValue;
    type Setter = SetLocalWindowOption;
    type GetterAll = GetLocalWindowOption;
    type SetterMultiple = SetLocalWindowOptions<'a>;

    fn target(&self) -> Option<Cow<'a, str>> {
        self.target.to_owned()
    }

    fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }
}
