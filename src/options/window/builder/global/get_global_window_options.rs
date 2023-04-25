use crate::options::{GetGlobalWindowOption, GetUserOptions, GetWindowOptionsTr};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct GetGlobalWindowOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

// XXX: both are same, optimize
impl<'a> GetWindowOptionsTr<'a, GetGlobalWindowOption> for GetGlobalWindowOptions<'a> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            options: TmuxCommandList::new(),
        }
    }

    fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.options.push(cmd.into())
    }

    fn into_commands(self) -> TmuxCommandList<'a> {
        self.options
    }
}

impl<'a> GetUserOptions<'a> for GetGlobalWindowOptions<'a> {
    type Getter = GetGlobalWindowOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
