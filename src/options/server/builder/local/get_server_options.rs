use crate::{GetServerOption, GetServerOptionsTr, GetUserOptions, TmuxCommand, TmuxCommands};

#[derive(Debug)]
pub struct GetServerOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetServerOptionsTr<'a> for GetServerOptions<'a> {
    type Getter = GetServerOption;

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

impl<'a> GetUserOptions<'a> for GetServerOptions<'a> {
    type Getter = GetServerOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
