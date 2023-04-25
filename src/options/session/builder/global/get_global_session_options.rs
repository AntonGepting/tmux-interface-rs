use crate::options::{GetGlobalSessionOption, GetSessionOptionsTr, GetUserOptions};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct GetGlobalSessionOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> GetUserOptions<'a> for GetGlobalSessionOptions<'a> {
    type Getter = GetGlobalSessionOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}

// XXX: both are same, optimize
impl<'a> GetSessionOptionsTr<'a, GetGlobalSessionOption> for GetGlobalSessionOptions<'a> {
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
