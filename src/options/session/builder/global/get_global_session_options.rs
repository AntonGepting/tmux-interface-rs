use crate::options::{GetGlobalSessionOption, GetSessionOptionsTr, GetUserOptions};
use crate::{TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct GetGlobalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
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
            options: TmuxCommands::new(),
        }
    }

    fn push<T: Into<TmuxCommand<'a>>>(&mut self, cmd: T) {
        self.options.push(cmd.into())
    }

    fn into_commands(self) -> TmuxCommands<'a> {
        self.options
    }
}
