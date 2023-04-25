use crate::options::{SetLocalSessionOption, SetSessionOptionsTr, SetUserOptions};
use crate::{TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct SetLocalSessionOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> SetSessionOptionsTr<'a> for SetLocalSessionOptions<'a> {
    type Setter = SetLocalSessionOption;

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

impl<'a> SetUserOptions<'a> for SetLocalSessionOptions<'a> {
    type Setter = SetLocalSessionOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
