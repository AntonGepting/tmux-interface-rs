use crate::options::{
    GetLocalSessionOption, GetLocalSessionOptionValue, SessionOptionsCtl, SetLocalSessionOption,
    SetLocalSessionOptions,
};
use crate::{Error, Tmux, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// XXX: rename SessionOptionCtl?
// trait top level options, then server session window pane
pub struct LocalSessionOptionsCtl<'a> {
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

impl<'a> Default for LocalSessionOptionsCtl<'a> {
    fn default() -> Self {
        Self {
            invoker: &|cmd| Tmux::with_command(cmd).output(),
            target: None,
        }
    }
}

impl<'a> LocalSessionOptionsCtl<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(
        target: Option<S>,
        invoker: &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error>,
    ) -> Self {
        Self {
            invoker,
            target: target.map(|s| s.into()),
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

// XXX: mb no need for Local, Global only custom implementation?
// XXX: mb better name instead of Local (not a network meaning)
impl<'a> SessionOptionsCtl<'a> for LocalSessionOptionsCtl<'a> {
    type Getter = GetLocalSessionOptionValue;
    type Setter = SetLocalSessionOption;
    type GetterAll = GetLocalSessionOption;
    type SetterMultiple = SetLocalSessionOptions<'a>;

    fn target(&self) -> Option<Cow<'a, str>> {
        self.target.to_owned()
    }

    fn invoker(&self) -> &'a dyn Fn(TmuxCommand<'a>) -> Result<TmuxOutput, Error> {
        self.invoker
    }
}
