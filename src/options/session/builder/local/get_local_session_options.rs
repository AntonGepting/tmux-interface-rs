use crate::options::{GetLocalSessionOption, GetSessionOptionsTr, GetUserOptions};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct GetLocalSessionOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> GetUserOptions<'a> for GetLocalSessionOptions<'a> {
    type Getter = GetLocalSessionOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}

// XXX: both are same, optimize
impl<'a> GetSessionOptionsTr<'a, GetLocalSessionOption> for GetLocalSessionOptions<'a> {
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
