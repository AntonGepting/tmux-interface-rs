use crate::options::{SetLocalSessionOption, SetSessionOptions, SetUserOptions};
use crate::{TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct SetLocalSessionOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetSessionOptions<'a> for SetLocalSessionOptions<'a> {
    type Setter = SetLocalSessionOption;

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

impl<'a> SetUserOptions<'a> for SetLocalSessionOptions<'a> {
    type Setter = SetLocalSessionOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
