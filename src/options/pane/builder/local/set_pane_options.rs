use crate::{SetPaneOption, SetPaneOptionsTr, SetUserOptions, TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct SetPaneOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetPaneOptionsTr<'a> for SetPaneOptions<'a> {
    type Setter = SetPaneOption;

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

impl<'a> SetUserOptions<'a> for SetPaneOptions<'a> {
    type Setter = SetPaneOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
