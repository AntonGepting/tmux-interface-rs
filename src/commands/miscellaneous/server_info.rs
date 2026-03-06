// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Info<'a> = ServerInfo<'a>;

/// Show server information
///
/// tmux >=0.8 && <=1.9:
/// ```text
/// server-info
/// (alias: info)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ServerInfo<'a> {}

impl<'a> ServerInfo<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SERVER_INFO);

        cmd
    }
}
