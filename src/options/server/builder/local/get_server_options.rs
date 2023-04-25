use crate::{GetServerOption, GetServerOptionsTr, GetUserOptions, TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct GetServerOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> GetServerOptionsTr<'a> for GetServerOptions<'a> {
    type Getter = GetServerOption;

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

impl<'a> GetUserOptions<'a> for GetServerOptions<'a> {
    type Getter = GetServerOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
