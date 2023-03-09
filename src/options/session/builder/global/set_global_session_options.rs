use crate::options::{SetGlobalSessionOption, SetSessionOptions, SetUserOptions};
use crate::{TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct SetGlobalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetSessionOptions<'a> for SetGlobalSessionOptions<'a> {
    type Setter = SetGlobalSessionOption;

    fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }

    fn push_cmds(&mut self, options: TmuxCommands<'a>) {
        self.options.push_cmds(options);
    }

    fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}

impl<'a> SetUserOptions<'a> for SetGlobalSessionOptions<'a> {
    type Setter = SetGlobalSessionOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
