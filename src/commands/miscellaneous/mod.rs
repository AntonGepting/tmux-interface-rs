use crate::TmuxCommand;
#[cfg(feature = "tmux_1_8")]
use crate::WaitFor;
#[cfg(feature = "tmux_0_8")]
use crate::{ClockMode, IfShell, LockServer};

#[cfg(feature = "tmux_1_1")]
use crate::RunShell;

/// All functions from man tmux "Miscellaneous" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS))
#[cfg(feature = "tmux_0_8")]
pub mod clock_mode;
#[cfg(feature = "tmux_0_8")]
pub mod if_shell;
#[cfg(feature = "tmux_0_8")]
pub mod lock_server;
//#[cfg(feature = "tmux_1_0")]
//pub mod server_info;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_password;
#[cfg(feature = "tmux_1_1")]
pub mod run_shell;
#[cfg(feature = "tmux_1_8")]
pub mod wait_for;

#[cfg(feature = "tmux_0_8")]
pub mod clock_mode_tests;
#[cfg(feature = "tmux_0_8")]
pub mod if_shell_tests;
#[cfg(feature = "tmux_0_8")]
pub mod lock_server_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod server_info_tests;
//#[cfg(feature = "tmux_1_0")]
//pub mod set_password_tests;
#[cfg(feature = "tmux_1_1")]
pub mod run_shell_tests;
#[cfg(feature = "tmux_1_8")]
pub mod wait_for_tests;

/// All functions from man tmux "Miscellaneous" listed below
/// ([man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#MISCELLANEOUS))
impl<'a> TmuxCommand<'a> {
    #[cfg(feature = "tmux_0_8")]
    pub fn clock_mode() -> ClockMode<'a> {
        ClockMode::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn if_shell() -> IfShell<'a> {
        IfShell::new()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn lock_server() -> LockServer {
        LockServer::new()
    }

    #[cfg(feature = "tmux_1_1")]
    pub fn run_shell() -> RunShell<'a> {
        RunShell::new()
    }

    #[cfg(feature = "tmux_1_8")]
    pub fn wait_for() -> WaitFor<'a> {
        WaitFor::new()
    }
}

impl<'a> From<ClockMode<'a>> for TmuxCommand<'a> {
    fn from(item: ClockMode<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<IfShell<'a>> for TmuxCommand<'a> {
    fn from(item: IfShell<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<LockServer> for TmuxCommand<'a> {
    fn from(item: LockServer) -> Self {
        item.build()
    }
}

impl<'a> From<RunShell<'a>> for TmuxCommand<'a> {
    fn from(item: RunShell<'a>) -> Self {
        item.build()
    }
}

impl<'a> From<WaitFor<'a>> for TmuxCommand<'a> {
    fn from(item: WaitFor<'a>) -> Self {
        item.build()
    }
}
