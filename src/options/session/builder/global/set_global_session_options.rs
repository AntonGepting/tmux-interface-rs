use crate::options::{SetGlobalSessionOption, SetSessionOptionsTr, SetUserOptions};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct SetGlobalSessionOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> SetSessionOptionsTr<'a> for SetGlobalSessionOptions<'a> {
    type Setter = SetGlobalSessionOption;

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

impl<'a> SetUserOptions<'a> for SetGlobalSessionOptions<'a> {
    type Setter = SetGlobalSessionOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
