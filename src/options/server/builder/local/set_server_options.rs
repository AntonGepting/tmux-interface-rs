use crate::{SetServerOption, SetServerOptionsTr, SetUserOptions, TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct SetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetServerOptionsTr<'a> for SetServerOptions<'a> {
    type Setter = SetServerOption;

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

impl<'a> SetUserOptions<'a> for SetServerOptions<'a> {
    type Setter = SetServerOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
