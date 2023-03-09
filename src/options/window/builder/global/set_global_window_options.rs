use crate::options::{SetGlobalWindowOption, SetUserOptions, SetWindowOptions};
use crate::{TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct SetGlobalWindowOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetWindowOptions<'a> for SetGlobalWindowOptions<'a> {
    type Setter = SetGlobalWindowOption;

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

impl<'a> SetUserOptions<'a> for SetGlobalWindowOptions<'a> {
    type Setter = SetGlobalWindowOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
