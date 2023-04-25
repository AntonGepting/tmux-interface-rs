use crate::{SetPaneOption, SetPaneOptionsTr, SetUserOptions, TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct SetPaneOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> SetPaneOptionsTr<'a> for SetPaneOptions<'a> {
    type Setter = SetPaneOption;

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

impl<'a> SetUserOptions<'a> for SetPaneOptions<'a> {
    type Setter = SetPaneOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
