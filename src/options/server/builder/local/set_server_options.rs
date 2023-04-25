use crate::{SetServerOption, SetServerOptionsTr, SetUserOptions, TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct SetServerOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> SetServerOptionsTr<'a> for SetServerOptions<'a> {
    type Setter = SetServerOption;

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

impl<'a> SetUserOptions<'a> for SetServerOptions<'a> {
    type Setter = SetServerOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
