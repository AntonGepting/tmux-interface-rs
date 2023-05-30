use crate::{GetPaneOption, GetPaneOptionsTr, GetUserOptions, TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct GetPaneOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetPaneOptionsTr<'a> for GetPaneOptions<'a> {
    type Getter = GetPaneOption;

    fn new() -> Self
    where
        Self: Sized,
    {
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

impl<'a> GetUserOptions<'a> for GetPaneOptions<'a> {
    type Getter = GetPaneOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
