use crate::options::{GetLocalWindowOption, GetUserOptions, GetWindowOptionsTr};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct GetLocalWindowOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

// XXX: both are same, optimize
impl<'a> GetWindowOptionsTr<'a, GetLocalWindowOption> for GetLocalWindowOptions<'a> {
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

impl<'a> GetUserOptions<'a> for GetLocalWindowOptions<'a> {
    type Getter = GetLocalWindowOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
