use crate::{GetPaneOption, GetPaneOptionsTr, GetUserOptions, TmuxCommand, TmuxCommandList};

#[derive(Debug)]
pub struct GetPaneOptions<'a> {
    pub options: TmuxCommandList<'a>,
}

impl<'a> GetPaneOptionsTr<'a> for GetPaneOptions<'a> {
    type Getter = GetPaneOption;

    fn new() -> Self
    where
        Self: Sized,
    {
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

impl<'a> GetUserOptions<'a> for GetPaneOptions<'a> {
    type Getter = GetPaneOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}
