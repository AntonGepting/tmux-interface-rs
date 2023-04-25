use crate::options::{SetGlobalWindowOption, SetUserOptions, SetWindowOptionsTr};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct SetGlobalWindowOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> SetWindowOptionsTr<'a> for SetGlobalWindowOptions<'a> {
    type Setter = SetGlobalWindowOption;

    fn new() -> Self {
        Self {
            options: TmuxCommandList::new(),
        }
    }

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }

    fn push_cmds(&mut self, options: TmuxCommandList<'a>) {
        self.options.push_cmds(options);
    }

    fn build(self) -> TmuxCommandList<'a> {
        self.options
    }
}

impl<'a> SetUserOptions<'a> for SetGlobalWindowOptions<'a> {
    type Setter = SetGlobalWindowOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
